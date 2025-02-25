@0xb440d4a7ebbc1e92;

using import "../aliases/color.capnp".Color;
using import "../aliases/enum.capnp".EnumValue;
using import "aliases/id.capnp".ColorChannelId;
using import "aliases/opacity.capnp".Opacity;
using import "aliases/opacity.capnp".OptionOpacity;
using import "hsv.capnp".Hsv;

struct PlayerColorChannelOptions {
    playerColor @0 :EnumValue;
    opacity @1 :Opacity;
}

struct NormalColorChannelOptions {
    color @0 :Color;
    opacity @1 :Opacity;
}

struct CopiedColorChannelOptions {
    id @0 :ColorChannelId;
    hsv @1 :Hsv;
    opacity @2 :OptionOpacity;
}

struct ColorChannel {
    id @0 :ColorChannelId;
    blending @1 :Bool;
    union {
        player @2 :PlayerColorChannelOptions;
        normal @3 :NormalColorChannelOptions;
        copied @4 :CopiedColorChannelOptions;
    }
}
