fn convert_to_u32(bits: &Vec<u8>) -> u32 {
    let mut result: u32 = 0;
    for (idx, bit) in bits.iter().rev().enumerate() {
        // println!("bit: {} at idx: {}", bit, idx);
        // These unwraps should not fail
        result += 2u32.pow(idx.try_into().unwrap()) * (*bit as u32);
    }

    result
}

struct BitDescriptor {
    bits: Vec<u8>,
    value: u32,
}

impl BitDescriptor {
    fn new(bits: Vec<u8>) -> Self {
        let value = convert_to_u32(&bits);

        Self { bits, value }
    }
}

struct BitPacker {
    descriptors: Vec<BitDescriptor>,
}

impl BitPacker {
    fn pack(descriptors: Vec<BitDescriptor>) -> Vec<u32> {
        let mut buf_size = descriptors.iter().fold(0, |acc, e| acc + e.bits.len());

        // We want to get buf size to lowest
        // number of bytes, so we add 7 and right
        // shift by 3 ( i.e. divide by 8 )
        //
        // let's say we have 3 bits, that
        // will be (3 + 7) / 8 = 1
        //
        // As we add 7, we cannot ever reach
        // 2 after division with 8, hence this automatically
        // truncates floating points and also rounds
        // our bits to nearest bytes
        buf_size = (buf_size + 7) >> 3;

        // vector of bytes
        let buffer = vec![0u32; buf_size];

        BitPacker::pack_into_buffer(descriptors, buffer)
    }

    fn pack_into_buffer(descriptors: Vec<BitDescriptor>, mut buffer: Vec<u32>) -> Vec<u32> {
        // This keeps track of bytes
        // in buffer vector
        let mut byte_idx = 0;

        // This keeps track of number of
        // unfilled bits in "each" byte
        // ( byte under consideration in
        // that iteration )
        //
        // If this is at position 1, it means
        // idx 0 is the one that is filled and
        // we need to fill further bits from position 1
        let mut bit_idx = 0;

        for desc in descriptors {
            let mut n_bits = desc.bits.len();
            let value = desc.value;

            while n_bits > 0 {
                // Checking if remaining space in current byte
                // is enough to fit number of bits ( n_bits )
                // left to be encoded, if yes this will be our
                // last iteration for this descriptor
                if n_bits <= (8 - bit_idx) {
                    // create a mask which only fills needed number
                    // of bits in needed position
                    let mask = (1 << n_bits) - 1;
                    // println!("mask: {:08b}", mask);

                    let masked_value = value & mask;
                    // We write to the byte left to right
                    //
                    // We calculate how many bits to shift the writing mask
                    // for byte, by finding the current index which we want
                    // the front of written bits to shift to minus the current
                    // position of front bit ( front bit = bit on leftmost side )
                    buffer[byte_idx] |= masked_value << ((8 - bit_idx) - n_bits);
                    // println!("result: {:08b}", buffer[byte_idx]);

                    bit_idx += n_bits;
                    if bit_idx == 8 {
                        bit_idx = 0;
                        byte_idx += 1;
                    }

                    // We have filled in all bits so
                    // we can mark this operation complete
                    n_bits = 0;
                    break;
                } else {
                    // write 8 bits directly to byte in this case
                    let mask = ((1 << (7 - bit_idx)) - 1) << (n_bits - (8 - bit_idx));
                    buffer[byte_idx] |= (value & mask) << (n_bits - (8 - bit_idx));
                    // println!("mask: {:08b}", mask);

                    n_bits -= 8 - bit_idx;
                    bit_idx = 0;
                    byte_idx += 1;
                }
            }
        }

        buffer
    }
}

fn main() {
    // BitPacker::pack(vec![BitDescriptor::new(vec![1, 0, 1])]);
    // BitPacker::pack(vec![BitDescriptor::new(vec![0, 0, 0, 0, 1, 0, 0, 0])]);
    let buffer = BitPacker::pack(vec![BitDescriptor::new(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]);
    for (idx, byte) in buffer.iter().enumerate() {
        println!("idx: {idx} - byte: {:08b}", byte);
    }
}
