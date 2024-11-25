# Actix-Web Broadcaster

A broadcaster liblary for [actix-ws](https://crates.io/crates/actix-ws) that includes grouping and conditional broadcasting.

This liblary provides grouping and broadcasting mechanism for brand new websocket liblary of Actix-Web Ecosystem. You have individual `Connection`s for each `Session` implementation of [actix-ws](https://crates.io/crates/actix-ws), will be identified as the given id. And there is also rooms exist, which benefits to group related connections on a single entity.

## Guide

### Adding Dependency

Add that line to your `Cargo.toml` file:

```toml

actix-ws-broadcaster = "0.1.0"

```

### Import

```rust

use actix_wsb::Broadcaster;

```

### Initialize

```rust

let broadcaster = Broadcaster::new();

```

It returns an `Arc<RwLock<Broadcaster>>`. Which means you can pass it between threads.

### Handle Connections And Rooms

We have very basic api, when you get the broadcaster in websocket controller,
you'll handle all the whole grouping work with only one line of code:

```rust
// you handle the socket initially with actix_ws api's:

let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;

// get the broadcaster, then handle it:

// the room_id and connection_id has to be string:

let broadcaster = Broadcaster::handle(&broadcaster, room_id, connection_id, session);

```

### Broadcast The Messages

Note: You have to do broadcasting in same broadcaster instance,
don't clone it. Otherwise it could cause data race.

In the loop of websocket, if a message received, you can broadcast it by that code:

```rust

Message::Text(msg) => {
    // we get the broadcaster for each message with write access:
    let mut writeable_broadcaster = broadcaster.write().unwrap();

    // broadcast it.
    writeable_broadcaster.room(room_id).broadcast(msg.to_string()).await;
}

```

If you want to broadcast message conditionally, you can use
`.broadcast_if()` and `.broadcast_if_not()` methods.

In `.broadcast_if()` method, if given condition on the closure is true
per each connection, broadcastes the message.

```rust

// broadcast for all:

writeable_broadcaster.room(room_id).broadcast_if(msg.to_string(), |connection| true).await;

```

And i also implemented the reverse version of that function,
broadcastes if given condition is false:

```rust

// broadcast for all:

writeable_broadcaster.room(room_id).broadcast_if_not(msg.to_string(), |connection| false).await;

```

### Remove A Connection if it Disconnects

If a client disconnects, you should remove their assigned connection by that code:

```rust

Message::Close(reason) => {
    // because the async closures are not stable yet, 
    // we have to remove connections with explicitly 
    // calling .close() method.
    let _ = broadcaster.write().unwrap().remove_connection(id).unwrap().close(reason).await;

    // stop listening messages and break the loop if 
    // a connection is removed.
    break;
},

```

## Try it yourself

To try it yourself, run that command: `cargo run --example example`,
than go to the `http://localhost:5000` address on a firefox based
browser(such as firefox, librewolf etc.). Because chromium based
browsers don't support to send query parameters to websockets from
the javascript, our front-end configuration don't work on them.
In real world scenarios, you have to provide room and connection
id's with different approach.

## Contribution Guide

Issues, suggestions and pull requests are welcome.
