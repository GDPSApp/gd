@0xbbf49df9a6501603;

using import "aliases/statistics.capnp".RewardStars;
using import "aliases/statistics.capnp".RewardMoons;

struct EitherReward {
    union {
        stars @0 :RewardStars;
        moons @1 :RewardMoons;
    }
}
