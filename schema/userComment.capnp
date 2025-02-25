@0xa33c26ceeb45eeef;

using import "aliases/color.capnp".Color;
using import "aliases/id.capnp".Id;
using import "aliases/statistics.capnp".Rating;
using import "aliases/time.capnp".Timestamp;
using import "user.capnp".UserReference;

struct UserComment {
    id @0 :Id;
    author @1 :UserReference;
    content @2 :Text;
    color @3 :Color;
    rating @4 :Rating;
    createdAt @5 :Timestamp;
}
