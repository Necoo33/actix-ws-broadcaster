use std::sync::{Arc, RwLock};
use actix_ws::{CloseReason, Item, Session};
use actix_web::web::Bytes;

#[derive(Clone)]
pub struct Connection {
    pub id: String,
    pub session: Session
}

#[derive(Clone)]
pub struct Room {
    pub id: String,
    pub connectors: Vec<Connection>
}

#[derive(Clone)]
pub struct Broadcaster {
    pub rooms: Vec<Room>
}

impl Connection {
    /// creates a single connection.
    pub fn create(id: String, session: Session) -> Self {
        Self {
            id, 
            session
        }
    }

    /// sends message from single connection.
    pub async fn send(&mut self, message: String) -> () {
        self.session.text(message).await.unwrap();
    }

    /// sends message from single connection if given condition is true.
    pub async fn send_if<F>(&mut self, message: String, condition: F) where F: Fn(&Connection) -> bool {
        if condition(&self) {
            self.session.text(message).await.unwrap();
        }
    }

    /// */ sends message from single connection if given condition is false.
    pub async fn send_if_not<F>(&mut self, message: String, condition: F) where F: Fn(&Connection) -> bool {
        if !condition(&self) {
            self.session.text(message).await.unwrap();
        }
    }

    /// sends a ping message from single connection.
    pub async fn ping(&mut self, bytes: &Vec<u8>) -> () {
        self.session.ping(bytes).await.unwrap();
    }

    /// sends a ping message from single connection if given condition is true.
    pub async fn ping_if<F>(&mut self, bytes: &Vec<u8>, condition: F) where F: Fn(&Connection) -> bool {
        if condition(&self) {
            self.session.ping(bytes).await.unwrap();
        }
    }

    /// */ sends a ping message from single connection if given condition is false.
    pub async fn ping_if_not<F>(&mut self, bytes: &Vec<u8>, condition: F) where F: Fn(&Connection) -> bool {
        if !condition(&self) {
            self.session.ping(bytes).await.unwrap();
        }
    }
}

impl Room {
    /// checks if a connection with given id exist and if it's not add a connection with given id and Session to a room.
    pub fn add_connection(&mut self, id: &String, session: Session) {
        let check_is_connection_exist = self.connectors.iter().any(|room| room.id == *id);

        match check_is_connection_exist {
            true => (),
            false => {
                let connection = Connection {
                    id: id.clone(),
                    session
                };

                self.connectors.push(connection);
            }
        }
    }

    /// removes if a connection with given id exist.
    pub fn remove_connection(&mut self, id: String) {
        self.connectors.retain(|connection| { 
            if connection.id == id { 
                false 
            } else { 
                true 
            }
        });
    }

    /// checks if a connection exist and returns it as an option.
    pub fn check_connection(&mut self, id: &String) -> Option<Connection> {
        let connection = self.connectors.iter().find(|room| room.id == *id);

        match connection {
            Some(connection) => Some(connection.clone()),
            None => None
        }
    }

    /// broadcastes the message to all room connectors.
    pub async fn broadcast(&mut self, message: String) {
        for connection in &mut self.connectors { 
            let message = message.clone(); 
            let session = &mut connection.session; 
            
            let _ = session.text(message).await;
        }
    }

    /// broadcastes the message if given condition for connection instances is true.
    pub async fn broadcast_if<F>(&mut self, message: String, condition: F) where F: Fn(&Connection) -> bool { 
        for connection in &mut self.connectors { 
            if condition(connection) { 
                let message = message.clone(); 
                let session = &mut connection.session; 
                let _ = session.text(message).await;
            } 
        } 
    }

    /// broadcastes the message if given condition for connection instances is false.
    pub async fn broadcast_if_not<F>(&mut self, message: String, condition: F) where F: Fn(&Connection) -> bool { 
        for connection in &mut self.connectors { 
            if !condition(connection) { 
                let message = message.clone(); 
                let session = &mut connection.session; 
                let _ = session.text(message).await;
            } 
        } 
    }

    /// broadcastes the ping to all room connectors.
    pub async fn ping(&mut self, bytes: Vec<u8>) { 
        for connection in &mut self.connectors { 
            let message = &bytes; 
            let session = &mut connection.session; 
                
            let _ = session.ping(message).await;
        }
    }

    /// broadcastes the ping if given condition for connection instances is true.
    pub async fn ping_if<F>(&mut self, bytes: Vec<u8>, condition: F) where F: Fn(&Connection) -> bool { 
        for connection in &mut self.connectors { 
            if condition(connection) { 
                let message = &bytes; 
                let session = &mut connection.session; 
                let _ = session.ping(message).await;
            } 
        } 
    }

    /// broadcastes the ping if given condition for connection instances is false.
    pub async fn ping_if_not<F>(&mut self, bytes: Vec<u8>, condition: F) where F: Fn(&Connection) -> bool { 
        for connection in &mut self.connectors { 
            if !condition(connection) { 
                let message = &bytes; 
                let session = &mut connection.session; 
                let _ = session.ping(message).await;
            } 
        } 
    }

    /// broadcastes the pong to all room connectors.
    pub async fn pong(&mut self, bytes: Vec<u8>) { 
        for connection in &mut self.connectors { 
            let message = &bytes; 
            let session = &mut connection.session; 
                
            let _ = session.pong(message).await;
        }
    }

    /// broadcastes the pong if given condition for connection instances is true.
    pub async fn pong_if<F>(&mut self, bytes: Vec<u8>, condition: F) where F: Fn(&Connection) -> bool { 
        for connection in &mut self.connectors { 
            if condition(connection) { 
                let message = &bytes; 
                let session = &mut connection.session; 
                let _ = session.pong(message).await;
            } 
        } 
    }

    /// broadcastes the pong if given condition for connection instances is false.
    pub async fn pong_if_not<F>(&mut self, bytes: Vec<u8>, condition: F) where F: Fn(&Connection) -> bool { 
        for connection in &mut self.connectors { 
            if !condition(connection) { 
                let message = &bytes; 
                let session = &mut connection.session; 
                let _ = session.pong(message).await;
            } 
        } 
    }

    /// Broadcastes the raw binary bytes to all room connectors.
    pub async fn binary(&mut self, bytes: Bytes) { 
        for connection in &mut self.connectors { 
            let message = bytes.clone();
            let session = &mut connection.session; 
                
            let _ = session.binary(message).await;
        }
    }

    /// broadcastes the raw binary bytes if given condition for connection instances is true.
    pub async fn binary_if<F>(&mut self, bytes: Bytes, condition: F) where F: Fn(&Connection) -> bool { 
        for connection in &mut self.connectors { 
            if condition(connection) { 
                let message = bytes.clone(); 
                let session = &mut connection.session; 
                let _ = session.binary(message).await;
            } 
        } 
    }

    /// broadcastes the raw binary bytes if given condition for connection instances is false.
    pub async fn binary_if_not<F>(&mut self, bytes: Bytes, condition: F) where F: Fn(&Connection) -> bool { 
        for connection in &mut self.connectors { 
            if !condition(connection) { 
                let message = bytes.clone(); 
                let session = &mut connection.session; 
                let _ = session.binary(message).await;
            } 
        } 
    }

    /// Broadcastes the continuation message to all room connectors.
    pub async fn continuation(&mut self, item: Item) { 
        for connection in &mut self.connectors { 
            let session = &mut connection.session; 

            match item {
                Item::FirstText(ref text) => {
                    let text = text;
                    session.continuation(Item::FirstText(text.clone())).await.unwrap()
                },
                Item::FirstBinary(ref binary) => {
                    let binary = binary;
                    session.continuation(Item::FirstBinary(binary.clone())).await.unwrap()
                },
                Item::Continue(ref cont_msg) => {
                    let cont_msg = cont_msg;
                    session.continuation(Item::Continue(cont_msg.clone())).await.unwrap()
                },
                Item::Last(ref last_msg) => {
                    let last_msg = last_msg;
                    session.continuation(Item::Last(last_msg.clone())).await.unwrap()
                }
            }
        }
    }

    /// broadcastes the continuation messages if given condition for connection instances is true.
    pub async fn continuation_if<F>(&mut self, item: Item, condition: F) where F: Fn(&Connection) -> bool { 
        for connection in &mut self.connectors { 
            if condition(connection) { 
                let session = &mut connection.session; 

                match item {
                    Item::FirstText(ref text) => {
                        let text = text;
                        session.continuation(Item::FirstText(text.clone())).await.unwrap()
                    },
                    Item::FirstBinary(ref binary) => {
                        let binary = binary;
                        session.continuation(Item::FirstBinary(binary.clone())).await.unwrap()
                    },
                    Item::Continue(ref cont_msg) => {
                        let cont_msg = cont_msg;
                        session.continuation(Item::Continue(cont_msg.clone())).await.unwrap()
                    },
                    Item::Last(ref last_msg) => {
                        let last_msg = last_msg;
                        session.continuation(Item::Last(last_msg.clone())).await.unwrap()
                    }
                }
            } 
        } 
    }

    /// broadcastes the continuation messages if given condition for connection instances is false.
    pub async fn continuation_if_not<F>(&mut self, item: Item, condition: F) where F: Fn(&Connection) -> bool { 
        for connection in &mut self.connectors { 
            if !condition(connection) { 
                let session = &mut connection.session; 
    
                match item {
                    Item::FirstText(ref text) => {
                        let text = text;
                        session.continuation(Item::FirstText(text.clone())).await.unwrap()
                    },
                    Item::FirstBinary(ref binary) => {
                        let binary = binary;
                        session.continuation(Item::FirstBinary(binary.clone())).await.unwrap()
                    },
                    Item::Continue(ref cont_msg) => {
                        let cont_msg = cont_msg;
                        session.continuation(Item::Continue(cont_msg.clone())).await.unwrap()
                    },
                    Item::Last(ref last_msg) => {
                        let last_msg = last_msg;
                        session.continuation(Item::Last(last_msg.clone())).await.unwrap()
                    }
                }
            } 
        } 
    }

    /// closes the connection with given id and removes it from it's room. This is the convenient way of closing a connection.
    /// 
    /// ```rust
    /// 
    /// Message::Close(reason) => {
    ///     let _ = get_broadcaster.write().unwrap().room(&room_id).close_conn(reason, &id).await;
    /// 
    ///     break;
    /// },
    /// 
    /// ```
    pub async fn close_conn(&mut self, reason: Option<CloseReason>, id: &String) {
        self.connectors.retain(|conn| {
            if conn.id == *id {
                let reason = reason.clone();
                    
                let _ = async {
                    let _ = conn.session.clone().close(reason).await;
                };
        
                false
            } else {
                true
            }
        });
    }

    /// closes all the connections and entire room. Warning: it closes all connections but keeps room open, if you want to close all the connections directly, use the `.remove_room()` method of the Broadcaster struct instead.
    pub async fn close(&mut self, reason: Option<CloseReason>) {
        self.connectors.retain(|conn| {
            let reason = reason.clone();
                
            let _ = async {
                let _ = conn.session.clone().close(reason).await;
            };

            false
        });
    }
    
    /// closes the connection and removes it from room if given condition for connection instances is true. Room still stay open.
    pub async fn close_if<F>(&mut self, reason: Option<CloseReason>, condition: F) where F: Fn(&Connection) -> bool { 
        self.connectors.retain(|connection| {
            if condition(connection) {
                let reason = reason.clone();
                
                let _ = async {
                    let _ = connection.session.clone().close(reason).await;
                };
    
                false
            } else {
                true
            }
        });
    }
    
    /// closes the connection and removes it from room if given condition for connection instances is false. Room still stay open.
    pub async fn close_if_not<F>(&mut self, reason: Option<CloseReason>, condition: F) where F: Fn(&Connection) -> bool { 
        self.connectors.retain(|connection| {
            if !condition(connection) {
                let reason = reason.clone();
                
                let _ = async {
                    let _ = connection.session.clone().close(reason).await;
                };
    
                false
            } else {
                true
            }
        });
    }
}

impl Broadcaster {
    /// create a new broadcaster instance. 
    pub fn new() -> Arc<RwLock<Self>> { 
        Arc::new(RwLock::new(Self::default())) 
    }

    /// does all the setup basically. You don't have to use other functions for all the grouping of rooms and connections. You can give the same room id for all instances if you don't want to seperate communication groups. But you have to give different connection id's to each session, otherwise it'll introduce bugs.
    ///     let id = query.id.as_ref().unwrap().to_string();
    /// 
    ///```rust
    /// 
    /// let id = query.id.as_ref().unwrap().to_string();
    /// let room_id = query.room.as_ref().unwrap().to_string();
    ///
    /// let get_broadcaster = Broadcaster::handle(&broadcaster, &room_id, &id, session);
    /// 
    ///```
    pub fn handle(broadcaster: &Arc<RwLock<Self>>, room_id: &String, conn_id: &String, session: Session) -> Arc<RwLock<Self>> {
        let mut broadcaster_write = broadcaster.write().unwrap();

        broadcaster_write.handle_room(room_id).add_connection(conn_id, session);

        Arc::clone(&broadcaster)
    }
    
    /// this function check if a room exist and if it's exist returns it, if it's not then creates it. If you just want to check if a room exist, use .check() instead.
    /// 
    ///```rust
    /// 
    /// let mut broadcaster_write = broadcaster.write().unwrap();
    /// 
    /// let room_id = "1".to_string();
    ///
    /// broadcaster_write.handle_room(&room_id)
    /// 
    ///```
    /// 
    pub fn handle_room(&mut self, id: &String) -> &mut Room {
        if let Some(index) = self.rooms.iter().position(|room| room.id == *id) {
            return &mut self.rooms[index];
        }
    
        self.rooms.push(Room {
            id: id.clone(),
            connectors: vec![],
        });
    
        self.rooms.last_mut().unwrap()
    }

    /// it scans a room with given id and it returns it if it's exist. if there is a risk that room isn't exist than use ".check_room()"
    pub fn room(&mut self, id: &String) -> &mut Room {
        return self.rooms.iter_mut().find(|room| room.id == *id).unwrap();
    }

    /// checks a room and if it's exist, returns a mutable reference of that room.
    pub fn check_room(&mut self, id: &String) -> Option<&mut Room> {
        match self.rooms.iter_mut().find(|room| room.id == *id) {
            Some(room) => Some(room),
            None => None
        }
    }

    /// it returns room if exist with given ip. Use .handle_room() method if you want to create a room with given id.
    pub fn check(&self, id: &String) -> bool {
        return self.rooms.iter().any(|room| room.id == *id);
    }

    /// iterates through every room and does something with them immutably. You cannot mutate anything inside of it, even rooms and not captured variables.      
    /// 
    /// ```rust
    /// 
    /// use actix_wsb::Broadcaster;
    /// 
    /// fn main () {
    ///     let broadcaster = Broadcaster::new();
    /// 
    ///     broadcaster.read().unwrap().each_room_immut(|room| println!("hello, {}. guest!", room.id));
    /// }
    /// 
    /// 
    /// ```
    pub fn each_room_immut<F>(&self, f: F) where F: Fn(&Room) {
        for room in &self.rooms {
            f(room);
        }
    }
    
    /// iterates through every room and does something with them immutably. You cannot mutate rooms itself but can mutate captured variables.
    /// 
    /// ```rust
    /// 
    /// use actix_wsb::Broadcaster;
    /// 
    /// fn main () {
    ///     let broadcaster = Broadcaster::new();
    /// 
    ///     let mut num = 0;
    /// 
    ///     broadcaster.read().unwrap().each_room(|room| {
    ///         num = num + 1;
    ///     });
    /// 
    ///     println!("here is number: {}", num)
    /// }
    /// 
    /// 
    /// ```
    pub fn each_room<F>(&self, mut f: F) where F: FnMut(&Room) {
        for room in &self.rooms {
            f(room);
        }
    }

    /// iterates through every room and does something with them mutably. You can mutate everything belong to it. But warning, for now, you cannot send messages to client from it right now and until async closures will be stable probably we're not be able to do it. Because of that, we're not able to give examples for that.
    pub async fn each_room_mut<F>(&mut self, mut f: F) where F: FnMut(&mut Room) {
        for room in &mut self.rooms {
            f(room);
        }
    }

    /// it removes a room with given id.
    /// 
    /// 
    /// ```rust 
    /// Message::Close(reason) => {
    ///     // warning, that closes and removes all the connections but not removes the room: 
    ///     //let _ = get_broadcaster.write().unwrap().room(room_id.clone()).close(reason).await;
    ///                
    ///     // if you want to remove a room with removing all the connections, use this instead:
    ///     // let _ = get_broadcaster.write().unwrap().remove_room(room_id.clone()).await;
    ///
    ///     let _ = get_broadcaster.write().unwrap()
    ///                                    .room(&room_id)
    ///                                    .remove_room(reason, |conn| conn.id == *id).await;
    ///                
    ///     break;
    ///  },
    /// ```
    ///
    pub async fn remove_room(&mut self, id: String) {
        self.rooms.retain(|room| { 
            if room.id == id { 
                let _ = async {
                    room.clone().close(None).await;
                };
                
                false 
            } else { 
                true 
            }
        });
    }

    /// it removes all empty rooms.
    pub fn remove_empty_rooms(&mut self) {
        self.rooms.retain(|room| { 
            if room.connectors.is_empty() { 
                false 
            } else { 
                true 
            }
        });
    }

    /// it removes a connection and returns the session struct of it. since async closures not stable yet, we cannot close the actual "Session" implementation in that method. For making that cleanup, we have to get actual Session implementation and close that connection manually - check out the example and readme.
    /// This is the old way of removing connections. It'll not be removed but we don't recommend to use it unless you don't used it yet.
    pub fn remove_connection(&mut self, id: String) -> Option<Session> {
        for room in &mut self.rooms {
            if let Some(pos) = room.connectors.iter().position(|connection| connection.id == id) {
                let connection = room.connectors.remove(pos);
                
                return Some(connection.session);
            }
        }
        None
    }
} 

impl Default for Broadcaster { 
    fn default() -> Self { 
        Self { 
            rooms: vec![], 
        } 
    }
}