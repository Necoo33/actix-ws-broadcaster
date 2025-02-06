use actix_web::{rt::spawn, web::{get, Data, Payload, Query}, App, HttpRequest, HttpResponse, HttpServer, Responder};

use actix_wsb::Broadcaster;
use askama::Template;
use serde;
use std::sync::{Arc, RwLock};
use actix_ws::{Item, Message};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rooms = Broadcaster::new();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(rooms.clone()))
            .route("/", get().to(home_controller))
            .route("/chat", get().to(chat_controller))
            .route("/chats", get().to(websocket_controller))
    });

    server.bind("127.0.0.1:5000").unwrap().run().await.unwrap();

    Ok(())
}

pub async fn home_controller() -> impl Responder {
    let our_template = HomeTemplate {
        blabla: 1
    };

    let template = our_template.render().unwrap();

    HttpResponse::Ok().content_type("text/html").body(template)
}

pub async fn chat_controller(inputs: Query<ChatInputs>) -> impl Responder {
    let our_template = ChatTemplate {
        id: inputs.id.to_string(),
        name: inputs.name.to_string(),
        room: inputs.room.to_string()
    };

    let template = our_template.render().unwrap();

    HttpResponse::Ok().content_type("text/html").body(template)
}

pub async fn websocket_controller(req: HttpRequest, body: Payload, broadcaster: Data<Arc<RwLock<Broadcaster>>>, query: Query<WebsocketInput>) -> actix_web::Result<impl Responder> {
    let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let id = query.id.as_ref().unwrap().to_string();
    let room_id = query.room.as_ref().unwrap().to_string();

    let get_broadcaster = Broadcaster::handle(&broadcaster, &room_id, &id, session);

    // ".each_room_immut()" example
    get_broadcaster.read().unwrap().each_room_immut(|room| println!("Hello to room {}!", room.id));
    
    // ".each_room()" example
    let mut num = 0;
    
    get_broadcaster.read().unwrap().each_room(|room| {
        for _ in room.connectors.iter() {
            num = num + 1
        }
    });
    
    spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Text(msg) => {
                    let mut write_broadcaster = get_broadcaster.write().unwrap();

                    write_broadcaster.room(&room_id).broadcast(msg.to_string()).await;
                },
                 Message::Close(reason) => {
                    // warning, that closes and removes all the connections but not removes the room: 
                    //let _ = get_broadcaster.write().unwrap().room(room_id.clone()).close(reason).await;
                    
                    // if you want to remove a room with removing all the connections, use this instead:
                    // let _ = get_broadcaster.write().unwrap().remove_room(room_id.clone()).await;

                    // you can conditionally close connections:
                    //let _ = get_broadcaster.write().unwrap().room(room_id.clone()).close_if(reason, |conn| conn.id == query.id.clone().unwrap()).await;
                    
                    // or, use the new api:

                    let _ = get_broadcaster.write().unwrap().room(&room_id).close_conn(reason, &id).await;
                    break;
                 },
                 Message::Pong(bytes) => {
                    let mut write_broadcaster = get_broadcaster.write().unwrap();

                    write_broadcaster.room(&room_id).ping(bytes.to_vec()).await;
                 },
                 Message::Ping(bytes) => {
                    let mut write_broadcaster = get_broadcaster.write().unwrap();

                    write_broadcaster.room(&room_id).pong(bytes.to_vec()).await;
                 },
                 Message::Continuation(item) => {
                    let mut write_broadcaster = get_broadcaster.write().unwrap();

                    let room = write_broadcaster.room(&room_id);

                    let msg = format!(r"hello, your continuation message: {:#?}", item);
                    
                    let start = Item::FirstBinary(msg.into());
                    let _ = room.continuation(start).await;

                    let cont_cont = Item::Continue(r"continue".into());
                    let _ = room.continuation(cont_cont).await;

                    let last = Item::Last(r"end".into());
                    let _ = room.continuation(last);

                 }
                 _ => ()
            }
        }
    });
    
    Ok(response)
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate { 
    pub blabla: i32
}

#[derive(Template)]
#[template(path = "chat.html")]
pub struct ChatTemplate {
    pub id: String,
    pub name: String,
    pub room: String
}

#[derive(Debug, serde::Deserialize)]
pub struct ChatInputs {
    pub id: String,
    pub name: String,
    pub room: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct WebsocketInput {
    pub room: Option<String>,
    pub message: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WebsocketOutput {
    pub message: String,
    pub id: String,
    pub name: String
}