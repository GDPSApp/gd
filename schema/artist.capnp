@0xcaf19768729afafd;

using import "aliases/id.capnp".Id;

struct Artist {
    id @0 :Id;
    name @1 :Text;
    verified @2 :Bool;
}
