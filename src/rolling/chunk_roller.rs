use super::Roller;
use std::cmp::min;

#[derive(Debug)]
pub struct ChunkRoller<T> {
    buffer: Vec<T>,
    roller: Roller<T>
}

impl<T: Copy + Clone + Default> ChunkRoller<T> {
    pub fn new_default(size: usize) -> Self {
        Self::new(size, T::default())
    }

    pub fn new(size: usize, fill: T) -> Self {
        Self { buffer: Vec::new(), roller: Roller::new(size, fill) }
    }

    pub fn data(&self) -> &[T] {
        self.roller.data()
    }

    pub fn enqueue(&mut self, data: &[T]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn roll(&mut self, mut max_chunk_size: usize) -> usize {
        max_chunk_size = min(max_chunk_size, self.buffer.len());
        let (chunk, rest) = self.buffer.split_at(max_chunk_size);
        self.roller.roll(chunk);
        let rolled_len = chunk.len();
        self.buffer.splice(.., rest.to_vec());
        rolled_len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn roll() {
        let mut chunk_roller: ChunkRoller<i16> = ChunkRoller::new_default(3);
        assert_eq!(chunk_roller.data(), [0, 0, 0]);
        chunk_roller.roll(2);
        assert_eq!(chunk_roller.data(), [0, 0, 0]);
        chunk_roller.enqueue(&[1, 2]);
        chunk_roller.enqueue(&[3, 4]);
        assert_eq!(chunk_roller.data(), [0, 0, 0]);
        chunk_roller.roll(1);
        assert_eq!(chunk_roller.data(), [0, 0, 1]);
        chunk_roller.roll(2);
        assert_eq!(chunk_roller.data(), [1, 2, 3]);
        chunk_roller.roll(3);
        assert_eq!(chunk_roller.data(), [2, 3, 4]);
        chunk_roller.roll(4);
        assert_eq!(chunk_roller.data(), [2, 3, 4]);
    }
}