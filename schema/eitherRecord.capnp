@0xc7c81742162b8987;

using import "aliases/record.capnp".Record;
using import "aliases/time.capnp".Duration;

struct EitherRecord {
    union {
        regular @0 :Record;
        platformer @1 :Duration;
    }
}
