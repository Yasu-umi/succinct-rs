//! Traits for working with bit vectors.

use num::ToPrimitive;

use block_type::BlockType;

/// Interface for read-only bit vector operations.
pub trait BitVector<Block: BlockType> {
    /// The length of the bit vector in blocks.
    fn block_len(&self) -> usize;

    /// The length of the bit vector in bits.
    ///
    /// Default implementation is `self.block_len() * Block::nbits()`.
    #[inline]
    fn bit_len(&self) -> u64 {
        self.block_len() as u64 * Block::nbits() as u64
    }

    /// Gets the value of the block at `position`
    fn get_block(&self, position: usize) -> Block;

    /// Gets the bit at `position`
    #[inline]
    fn get_bit(&self, position: u64) -> bool {
        assert!(position < self.bit_len(), "BitVector::get: out of bounds");
        let block_bits = Block::nbits() as u64;
        let block_index = (position / block_bits).to_usize().unwrap();
        let bit_offset = (position % block_bits) as usize;
        self.get_block(block_index).get_bit(bit_offset)
    }
}

/// Interface for mutable bit vector operations.
pub trait BitVectorMut<Block: BlockType> : BitVector<Block> {
    /// Sets the block at `position` to `value`.
    fn set_block(&mut self, position: usize, value: Block);

    /// Sets the bit at `position` to `value`.
    #[inline]
    fn set_bit(&mut self, position: u64, value: bool) {
        assert!(position < self.bit_len(), "BitVector::set: out of bounds");
        let block_bits = Block::nbits() as u64;
        let block_index = (position / block_bits).to_usize().unwrap();
        let bit_offset = (position % block_bits) as usize;
        let old_block = self.get_block(block_index);
        let new_block = old_block.set_bit(bit_offset, value);
        self.set_block(block_index, new_block);
    }
}

impl<Block: BlockType> BitVector<Block> for [Block] {
    #[inline]
    fn block_len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn get_block(&self, position: usize) -> Block {
        self[position]
    }
}

impl<Block: BlockType> BitVectorMut<Block> for [Block] {
    #[inline]
    fn set_block(&mut self, position: usize, value: Block) {
        self[position] = value;
    }
}

/// Interface for types that support rank queries.
pub trait Rank {
    /// Returns the rank at a given position.
    ///
    /// This is the number of 1s up to and including that position.
    fn rank(&self, position: u64) -> u64;

    /// Returns the rank of 0s at a given position.
    ///
    /// This is the number of 0s up to and including that position.
    fn rank0(&self, position: u64) -> u64 {
        position + 1 - self.rank(position)
    }
}

/// Interface for types that support select queries.
pub trait Select {
    /// Returns the position of the `index`th 1 bit.
    fn select(&self, index: u64) -> u64;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_len() {
        let v = vec![ 0u32; 4 ];
        assert_eq!(128, v.bit_len());
    }

    #[test]
    fn block_len() {
        let v = vec![ 0u32; 4 ];
        assert_eq!(4, v.block_len());
    }

    #[test]
    fn set_get_bit() {
        let mut v = vec![ 0b10101010u8; 4 ];
        assert!(  v.get_bit(0));
        assert!(! v.get_bit(1));
        assert!(  v.get_bit(2));
        assert!(! v.get_bit(3));

        v.set_bit(2, false);

        assert!(  v.get_bit(0));
        assert!(! v.get_bit(1));
        assert!(! v.get_bit(2));
        assert!(! v.get_bit(3));
    }
}