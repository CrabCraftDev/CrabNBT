import std.mem;

using NbtList;
using Compound;

enum NbtTag : u8 {
    End = 0x00,
    Byte = 0x01,
    Short = 0x02,
    Int = 0x03,
    Long = 0x04,
    Float = 0x05,
    Double = 0x06,
    ByteArray = 0x07,
    String = 0x08,
    List = 0x09,
    Compound = 0x0A,
    IntArray = 0x0B,
    LongArray = 0x0C,
};


struct Array<T> {
    be s32 len;
    T data[len];
};

struct NbtString {
    be u16 len;
    char data[len];
};

struct NbtValue<auto Tag> {
    match (Tag) {
        (NbtTag::Byte): be s8 value;
        (NbtTag::Short): be s16 value;
        (NbtTag::Int): be s32 value;
        (NbtTag::Long): be s64 value;
        (NbtTag::Float): be float value;
        (NbtTag::Double): be double value;
        (NbtTag::ByteArray): Array<u8> value;
        (NbtTag::String): NbtString value;
        (NbtTag::List): NbtList value;
        (NbtTag::Compound): Compound value;
        (NbtTag::IntArray): Array<s32> value;
        (NbtTag::LongArray): Array<s64> value;
    }
};

struct NbtList {
    NbtTag entryType;
    be s32 len;
    NbtValue<entryType> entires[len];
};


struct CompoundEntry {
    NbtTag type;
    be u16 nameLen [[hidden]];
    char name[nameLen];
    NbtValue<type> value [[inline]];
};


struct Compound {
    CompoundEntry elements[while(std::mem::read_unsigned($, 1) != NbtTag::End)] [[inline]];
    NbtTag end [[hidden]];
};

Compound nbt @ 0x00;