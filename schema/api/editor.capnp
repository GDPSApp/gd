@0xabc0efd0406f8f97;

using import "header.capnp".Header;
using import "object.capnp".AnyObject;

struct Editor {
    header @0 :Header;
    objects @1 :List(AnyObject);
}
