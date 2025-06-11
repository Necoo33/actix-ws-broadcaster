# CHANGELOG

## v0.12.0

Actix-Web version is upgraded to 4.11.0 .
Added `.continuation()`, `.continuation_if()` and `.continuation_if_not()` methods to the `Connection` type.

## v0.11.0

Added `.binary()`, `.binary_if()` and `.binary_if_not()` methods to the `Connection` type.

## v0.10.0

`actix-web` version updated to v4.10.2 and `serde` version updated to v1.0.219 .
Added `.pong()`, `.pong_if()` and `.pong_if_not()` methods to the `Connection` type. Which benefits to send ping messages to client from connection directly.

## v0.9.0

Added `.ping()`, `.ping_if()` and `.ping_if_not()` methods to the `Connection` type. Which benefits to send ping messages to client from connection directly.

## v0.8.1

Documentation updates. Documentation of some of the methods are revised and improved.

## v0.8.0

Added `.each_room_immut()`, `.each_room()` and `.each_room_mut()` methods to the `Broadcaster` type. They benefits to iterate through rooms with slight nuances.

## v0.7.1

Documentation update. All the documentation reviewed and updated depending on most recent api.

## v0.7.0

Added `.close_conn()` method to the `Room` type. Arguments of that methods are changed, now they take `&String` rather than `String`: `.add_connection()` and `.check_connection()` methods of `Room` type and `handle()` constructor, `.handle_room()`, `.room()`, `.check_room()` methods of ``Broadcaster` type.

## v0.6.1

Documentation update. Added more comprehensive documentation and some deprecated things removed.

## v0.6.0

Added `.close()`, `.close_if()` and `.close_if_not()` methods to `Room` type. If you close all connections, it removes all the connections from room but don't removes the room. If you want to also remove rooms with all the inner connections, use `.remove_room()` method of `Broadcaster` instead.

## v0.5.0

Added `.continuation()`, `.continuation_if()` and `.continuation_if_not()` methods to `Room` Type. It benefits to send big chunks continuously to sessions.

## v0.4.0

Added `.binary()`, `.binary_if()` and `.binary_if_not()` methods to `Room` Type. It benefits to send raw binary bytes to sessions.

## v0.3.0

Added `.pong()`, `.pong_if()` and `.pong_if_not()` methods to `Room` Type. It benefits to send a pong message to sessions.

## v0.2.0

Added `.ping()`, `.ping_if()` and `.ping_if_not()` methods to `Room` Type. It benefits to send a ping message to sessions.

## v0.1.1

Documentation fixes made and MSRV version added.

## v0.1.0

Crate created.
