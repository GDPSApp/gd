@0xc116a4429777f6d7;

using import "aliases/enum.capnp".EnumValue;
using import "aliases/id.capnp".Id;
using import "aliases/time.capnp".Timestamp;
using import "option.capnp".Option;
using import "user.capnp".UserReference;

struct Message {
    id @0 :Id;
    user @1 :UserReference;
    type @2 :EnumValue;
    subject @3 :Text;
    content @4 :Option(Text);
    createdAt @5 :Timestamp;
    read @6 :Bool;
}
