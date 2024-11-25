use actix_web::{web::{get, Data, Query, Payload}, rt::spawn, App, HttpResponse, HttpRequest, HttpServer, Responder};

use actix_wsb::Broadcaster;
use askama::Template;
use serde;
use std::sync::{Arc, RwLock};
use actix_ws::Message;

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

    let get_broadcaster = Broadcaster::handle(&broadcaster, room_id.clone(), id.clone(), session);
    
    spawn(async move {
        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Text(msg) => {
                    let mut write_broadcaster = get_broadcaster.write().unwrap();

                    write_broadcaster.room(room_id.clone()).broadcast(msg.to_string()).await;
                },
                 Message::Close(reason) => {
                    let _ = get_broadcaster.write().unwrap().remove_connection(id).unwrap().close(reason).await;
                    
                    break;
                 },
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