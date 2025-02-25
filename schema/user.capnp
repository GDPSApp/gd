@0x83252853124c73c2;

using import "aliases/enum.capnp".EnumValue;
using import "aliases/id.capnp".ColorId;
using import "aliases/id.capnp".IconId;
using import "aliases/id.capnp".Id;
using import "aliases/id.capnp".LongIconId;
using import "aliases/id.capnp".RoleId;
using import "aliases/statistics.capnp".CreatorPoints;
using import "aliases/statistics.capnp".Demons;
using import "aliases/statistics.capnp".Diamonds;
using import "aliases/statistics.capnp".Moons;
using import "aliases/statistics.capnp".OptionPlace;
using import "aliases/statistics.capnp".Rank;
using import "aliases/statistics.capnp".RewardCoins;
using import "aliases/statistics.capnp".SecretCoins;
using import "aliases/statistics.capnp".Stars;
using import "aliases/statistics.capnp".UserCoins;
using import "aliases/time.capnp".Timestamp;
using import "info.capnp".DemonInfo;
using import "info.capnp".LevelInfo;
using import "info.capnp".PlatformerInfo;
using import "eitherRecord.capnp".EitherRecord;
using import "option.capnp".Option;

struct UserReference {
    id @0 :Id;
    name @1 :Text;
    accountId @2 :Id;
}

struct UserStatistics {
    stars @0 :Stars;
    moons @1 :Moons;
    demons @2 :Demons;
    diamonds @3 :Diamonds;
    userCoins @4 :UserCoins;
    secretCoins @5 :SecretCoins;
    creatorPoints @6 :CreatorPoints;
    rank @7 :Rank;
    demonInfo @8 :DemonInfo;
}

struct UserCosmetics {
    color1Id @0 :ColorId;
    color2Id @1 :ColorId;
    color3Id @2 :ColorId;
    glow @3 :Bool;
    iconType @4 :EnumValue;
    iconId @5 :LongIconId;
    cubeId @6 :LongIconId;
    shipId @7 :IconId;
    ballId @8 :IconId;
    ufoId @9 :IconId;
    waveId @10 :IconId;
    robotId @11 :IconId;
    spiderId @12 :IconId;
    swingId @13 :IconId;
    jetpackId @14 :IconId;
    explosionId @15 :IconId;
    streakId @16 :IconId;
}

struct UserStates {
    messageState @0 :EnumValue;
    friendRequestState @1 :EnumValue;
    commentState @2 :EnumValue;
    friendState @3 :EnumValue;
}

struct UserSocials {
    youtube @0 :Option(Text);
    x @1 :Option(Text);
    twitch @2 :Option(Text);
    discord @3 :Option(Text);
}

struct UserLeaderboard {
    record @0 :EitherRecord;
    coins @1 :RewardCoins;
    recordedAt @2 :Timestamp;
}

struct User {
    id @0 :Id;
    name @1 :Text;
    accountId @2 :Id;
    roleId @3 :RoleId;
    banned @4 :Bool;
    statistics @5 :Option(UserStatistics);
    cosmetics @6 :Option(UserCosmetics);
    states @7 :Option(UserStates);
    socials @8 :Option(UserSocials);
    place @9 :OptionPlace;
    leaderboard @10 :Option(UserLeaderboard);
}
