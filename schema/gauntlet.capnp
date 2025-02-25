@0xb4afe2f523da20f0;

using import "aliases/id.capnp".Id;

struct Gauntlet {
    id @0 :Id;
    name @1 :Text;
    levelIds @2 :List(Id);
}
