@0xc93e648518af9c87;

using import "../option.capnp".Option;
using import "aliases/id.capnp".FolderId;

struct Folder {
    id @0 :FolderId;
    name @1 :Option(Text);
}
