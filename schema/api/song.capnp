@0xb01864dba3141a3b;

using import "../aliases/id.capnp".Id;
using import "../aliases/size.capnp".Size;
using import "../option.capnp".Option;
using import "aliases/priority.capnp".Priority;
using import "artist.capnp".ArtistApi;

struct SongApi {
    id @0 :Id;
    name @1 :Text;
    artist @2 :ArtistApi;
    size @3 :Size;
    url @4 :Option(Text);
    priority @5 :Priority;
}
