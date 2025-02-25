@0xafcfecace976de47;

using import "aliases/id.capnp".Id;
using import "aliases/enum.capnp".EnumValue;
using import "aliases/time.capnp".Timestamp;
using import "user.capnp".UserReference;

struct FriendRequest {
    id @0 :Id;
    user @1 :UserReference;
    type @2 :EnumValue;
    content @3 :Text;
    createdAt @4 :Timestamp;
    read @5 :Bool;
}
