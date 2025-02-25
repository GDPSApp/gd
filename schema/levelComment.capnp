@0xcc0963fa4c67ef7e;

using import "aliases/color.capnp".Color;
using import "aliases/id.capnp".Id;
using import "aliases/record.capnp".OptionRecord;
using import "aliases/statistics.capnp".Rating;
using import "aliases/time.capnp".Timestamp;
using import "level.capnp".LevelReference;
using import "user.capnp".UserReference;

struct LevelComment {
    id @0 :Id;
    author @1 :UserReference;
    content @2 :Text;
    color @3 :Color;
    rating @4 :Rating;
    createdAt @5 :Timestamp;
    level @6 :LevelReference;
    record @7 :OptionRecord;
}
