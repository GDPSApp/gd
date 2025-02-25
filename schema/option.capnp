@0xeb50502d690763c0;

struct Option(T) {
    union {
        some @0 :T;
        none @1 :Void;
    }
}

# define specializations for primitives

struct OptionBool {
    union {
        some @0 :Bool;
        none @1 :Void;
    }
}

struct OptionInt8 {
    union {
        some @0 :Int8;
        none @1 :Void;
    }
}

struct OptionInt16 {
    union {
        some @0 :Int16;
        none @1 :Void;
    }
}

struct OptionInt32 {
    union {
        some @0 :Int32;
        none @1 :Void;
    }
}

struct OptionInt64 {
    union {
        some @0 :Int64;
        none @1 :Void;
    }
}

struct OptionUInt8 {
    union {
        some @0 :UInt8;
        none @1 :Void;
    }
}

struct OptionUInt16 {
    union {
        some @0 :UInt16;
        none @1 :Void;
    }
}

struct OptionUInt32 {
    union {
        some @0 :UInt32;
        none @1 :Void;
    }
}

struct OptionUInt64 {
    union {
        some @0 :UInt64;
        none @1 :Void;
    }
}

struct OptionFloat32 {
    union {
        some @0 :Float32;
        none @1 :Void;
    }
}

struct OptionFloat64 {
    union {
        some @0 :Float64;
        none @1 :Void;
    }
}
