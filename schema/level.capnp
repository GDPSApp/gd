@0xea1c9d34d3f009fd;

using import "aliases/capacity.capnp".Capacity;
using import "aliases/enum.capnp".EnumValue;
using import "aliases/id.capnp".Id;
using import "aliases/id.capnp".TimelyId;
using import "aliases/password.capnp".Password;
using import "aliases/statistics.capnp".Downloads;
using import "aliases/statistics.capnp".ObjectCount;
using import "aliases/statistics.capnp".Rating;
using import "aliases/statistics.capnp".RewardCoins;
using import "aliases/statistics.capnp".RewardStars;
using import "aliases/statistics.capnp".Score;
using import "aliases/time.capnp".Duration;
using import "aliases/time.capnp".OptionTimestamp;
using import "aliases/time.capnp".TimeSteps;
using import "aliases/version.capnp".Version;
using import "eitherReward.capnp".EitherReward;
using import "option.capnp".Option;
using import "song.capnp".SongReference;
using import "user.capnp".UserReference;

struct LevelReference {
    id @0 :Id;
    name @1 :Text;
}

struct Level {
    id @0 :Id;
    name @1 :Text;
    description @2 :Text;
    creator @3 :UserReference;
    song @4 :SongReference;
    data @5 :Option(Data);
    version @6 :Version;
    downloads @7 :Downloads;
    gameVersion @8 :Version;
    rating @9 :Rating;
    length @10 :EnumValue;
    difficulty @11 :EnumValue;
    reward @12 :EitherReward;
    requestedReward @13 :EitherReward;
    score @14 :Score;
    rateType @15 :EnumValue;
    password @16 :Password;
    originalId @17 :Id;
    twoPlayer @18 :Bool;
    capacity @19 :Option(Capacity);
    coins @20 :RewardCoins;
    verifiedCoins @21 :Bool;
    lowDetail @22 :Bool;
    objectCount @23 :ObjectCount;
    createdAt @24 :OptionTimestamp;
    updatedAt @25 :OptionTimestamp;
    editorTime @26 :Duration;
    copiesTime @27 :Duration;
    timelyType @28 :EnumValue;
    timelyId @29 :TimelyId;
    timeSteps @30 :TimeSteps;
}
