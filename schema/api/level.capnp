@0xfa020de0587d329c;

using import "../aliases/capacity.capnp".Capacity;
using import "../aliases/enum.capnp".EnumValue;
using import "../aliases/flag.capnp".FlagValue;
using import "../aliases/id.capnp".Id;
using import "../aliases/id.capnp".TimelyId;
using import "../aliases/password.capnp".Password;
using import "../aliases/progress.capnp".Progress;
using import "../aliases/record.capnp".OptionRecord;
using import "../aliases/statistics.capnp".Attempts;
using import "../aliases/statistics.capnp".Clicks;
using import "../aliases/statistics.capnp".Downloads;
using import "../aliases/statistics.capnp".Jumps;
using import "../aliases/statistics.capnp".ObjectCount;
using import "../aliases/statistics.capnp".Rating;
using import "../aliases/statistics.capnp".RewardCoins;
using import "../aliases/statistics.capnp".RewardStars;
using import "../aliases/statistics.capnp".Score;
using import "../aliases/statistics.capnp".SecretCoins;
using import "../aliases/time.capnp".Duration;
using import "../aliases/version.capnp".Revision;
using import "../aliases/version.capnp".Version;
using import "../option.capnp".Option;
using import "../song.capnp".SongReference;
using import "../user.capnp".UserReference;
using import "aliases/id.capnp".FolderId;
using import "aliases/order.capnp".Order;
using import "recording.capnp".Recording;

struct OfficialLevelApiOptions {
    difficulty @0 :EnumValue;
    requiredCoins @1 :SecretCoins;
}

struct CreatedLevelApiOptions {
    revision @0 :Revision;
    verified @1 :Bool;
    uploaded @2 :Bool;
    recording @3 :Recording;
    collectedCoins @4 :FlagValue;
    unlisted @5 :Bool;
}

struct TimelyLevelApiOptions {
    timelyType @0 :EnumValue;
    timelyId @1 :TimelyId;
}

struct SavedLevelApiOptions {
    difficulty @0 :EnumValue;
    downloads @1 :Downloads;
    gameVersion @2 :Version;
    rating @3 :Rating;
    score @4 :Score;
    rateType @5 :FlagValue;
    hidden @6 :Bool;
    verifiedCoins @7 :Bool;
    favorite @8 :Bool;
    timely @9 :Option(TimelyLevelApiOptions);
}

struct CustomLevelApiOptions {
    song @0 :SongReference;
    creator @1 :UserReference;
    description @2 :Text;
    data @3 :Data;
    length @4 :EnumValue;
    password @5 :Password;
    originalId @6 :Id;
    twoPlayer @7 :Bool;
    objectCount @8 :ObjectCount;
    highObjectCount @9 :Bool;
    requestedStars @10 :RewardStars;
    lowDetail @11 :Bool;
    lowDetailEnabled @12 :Bool;
    editorTime @13 :Duration;
    copiesTime @14 :Duration;
    order @15 :Order;
    folderId @16 :FolderId;
    union {
        created @17 :CreatedLevelApiOptions;
        saved @18 :SavedLevelApiOptions;
    }
}

struct LevelApi {
    id @0 :Id;
    name @1 :Text;
    version @2 :Version;
    attempts @3 :Attempts;
    normalRecord @4 :OptionRecord;
    practiceRecord @5 :OptionRecord;
    stars @6 :RewardStars;
    jumps @7 :Jumps;
    binaryVersion @8 :Version;
    coins @9 :RewardCoins;
    capacity @10 :Capacity;
    orbRecord @11 :OptionRecord;
    bestClicks @12 :Clicks;
    bestTime @13 :Duration;
    progress @14 :Progress;
    check @15 :Bool;
    leaderboardRecord @16 :OptionRecord;
    union {
        official @17 :OfficialLevelApiOptions;
        custom @18 :CustomLevelApiOptions;
    }
}
