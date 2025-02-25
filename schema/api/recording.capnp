@0xf81fbb40b7d6e784;

using import "../aliases/time.capnp".Timestamp;

struct RecordingItem {
    timestamp @0 :Timestamp;
    previous @1 :Bool;
    next @2 :Bool;
    secondary @3 :Bool;
}

using Recording = List(RecordingItem);
