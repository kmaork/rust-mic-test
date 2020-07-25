use super::ChunkRoller;
use std::cmp::max;

#[derive(Debug)]
pub struct SmoothChunkRoller<T> {
    enqueued_data: u32,
    num_rolls: u32,
    roller: ChunkRoller<T>,
}

impl<T: Copy + Clone + Default> SmoothChunkRoller<T> {
    pub fn new_default(size: usize) -> Self {
        Self::new(size, T::default())
    }

    pub fn new(size: usize, fill: T) -> Self {
        Self { enqueued_data: 0, num_rolls: 0, roller: ChunkRoller::new(size, fill) }
    }

    pub fn data(&self) -> &[T] {
        self.roller.data()
    }

    pub fn enqueue(&mut self, data: &[T]) {
        self.enqueued_data += data.len() as u32;
        self.roller.enqueue(data);
    }

    pub fn roll(&mut self) {
        self.num_rolls += 1;
        let ratio = self.enqueued_data as f32 / self.num_rolls as f32;
        self.roller.roll(max(ratio.round() as usize, 1));
    }
}