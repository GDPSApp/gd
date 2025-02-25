@0x9d8a4802f4a1b9c7;

using import "../aliases/time.capnp".Timestamp;

using GuidelineColor = Float32;

struct Guideline {
    timestamp @0 :Timestamp;
    color @1 :GuidelineColor;
}

using Guidelines = List(Guideline);
