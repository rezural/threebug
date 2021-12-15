# Bevy Debug Server

WIP, pre-alpha!

Bevy Debug Server is probably not going to be it's future name.

This aims to be a visual debugging aid for 3d programs (currently only rust, bevy, nalgebra, parry). 

This is intended mainly for debugging 3d algorithms, that don't yet exist within a 3d environment. Usage should be as trivial as using tracing.rs.

Currently setting up a 3d environment takes about half a day or so, and comes with alot of choices WRT camera mode, plugins etc.

## Planned Features

Widgets etc. i.e. XYZ Axis etc.

Allow stepping through the creation of graphical objects, sorted by timestamp.

Selection of a subset of events, and stepping through these. Modifiers for per-entity visibility etc. 

Filters via type, and message.

Some mechanism for debugging mesh creation:
 * Create a mesh entity, client interface returns an Entity(usize)
 * Add mesh faces to entity, to debug creation order of faces, vertices & indices

## Building

```
git clone https://github.com/rezural/bevy_debug_server.git
```

Run the server: 

```
cargo run --bin dbg-server
```

Run example client: 

```
cargo run --bin dbg-client-example
```

This will send 10 AABB's to the dbg-server

## TODO

Client: 

[ ] Flesh out client debugging interface (see below)
[ ] Static initialization of network client, similar to tracing

Server:
[ ] Per client History
[*] WASD
[*] Actually display objects in 3d (currently only logging in server)

Other
[ ] 2d debugging, probably just display on plane in 3d

## Client interface

At the moment, something like (dbgg -> debug graphic):

```
dbgg::aabb(aabb: AABB, message: String)
dbgg::sphere(sphere: Sphere, message: String)
dbgg::line(line: Line, message: String)
// etc...
```