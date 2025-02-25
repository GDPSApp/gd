@0xf23d96c5f3c74c0d;

using import "aliases/color.capnp".Color;
using import "aliases/enum.capnp".EnumValue;
using import "aliases/id.capnp".Id;
using import "aliases/statistics.capnp".RewardCoins;
using import "aliases/statistics.capnp".RewardStars;

struct MapPack {
    id @0 :Id;
    name @1 :Text;
    levelIds @2 :List(Id);
    stars @3 :RewardStars;
    coins @4 :RewardCoins;
    difficulty @5 :EnumValue;
    color @6 :Color;
}
