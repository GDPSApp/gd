@0xbc24b7b4fc694ae9;

using import "aliases/statistics.capnp".Demons;
using import "aliases/statistics.capnp".Levels;

struct DemonInfoGroup {
    easy @0 :Demons;
    medium @1 :Demons;
    hard @2 :Demons;
    insane @3 :Demons;
    extreme @4 :Demons;
}

struct DemonInfoSpecial {
    weekly @0 :Demons;
    gauntlet @1 :Demons;
}

struct DemonInfo {
    regular @0 :DemonInfoGroup;
    platformer @1 :DemonInfoGroup;
    special @2 :DemonInfoSpecial;
}

struct LevelInfo {
    auto @0 :Levels;
    easy @1 :Levels;
    normal @2 :Levels;
    hard @3 :Levels;
    harder @4 :Levels;
    insane @5 :Levels;
    daily @6 :Levels;
    gauntlet @7 :Levels;
}

struct PlatformerInfo {
    auto @0 :Levels;
    easy @1 :Levels;
    normal @2 :Levels;
    hard @3 :Levels;
    harder @4 :Levels;
    insane @5 :Levels;
}
