@0xa64a59e494f00145;

using import "../aliases/enum.capnp".EnumValue;
using import "../aliases/id.capnp".Id;
using import "../aliases/id.capnp".OptionId;

struct Like {
    type @0 :EnumValue;
    targetId @1 :Id;
    relatedId @2 :OptionId;
    liked @3 :Bool;
}
