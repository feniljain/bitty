# Bitty

- A simple bit packer/unpacker in Rust
- Packing uses a naive approach of finding number of total bytes needed for packing and then serially packs all the bits in different bytes
- Unpacking uses a pattern matching approach to take in pattern of bits and match upon them to complete unpacking
- Inspired from [this](https://www.youtube.com/watch?v=74co_YG39Bw&list=PLP29wDx6QmW5xJ6yz_MInDL_AnLbpafyL&index=1) video by Low Byte Productions
