#[derive(Debug)]
pub struct Roller<T> {
    data: Vec<T>
}

impl<T: Copy + Clone + Default> Roller<T> {
    pub fn new_default(size: usize) -> Self {
        Self::new(size, T::default())
    }

    pub fn new(size: usize, fill: T) -> Self {
        Self { data: vec![fill; size] }
    }
    
    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn roll(&mut self, mut new_data: &[T]) {
        if new_data.len() < self.data.len() {
            self.data.copy_within(new_data.len().., 0);
        } else {
            new_data = &new_data[(new_data.len() - self.data.len())..];
        }
        let override_range = (self.data.len() - new_data.len())..;
        self.data.splice(override_range, new_data.iter().cloned());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn roll() {
        let mut roller: Roller<i16> = Roller::new_default(3);
        assert_eq!(roller.data, [0, 0, 0]);
        roller.roll(&[1]);
        assert_eq!(roller.data, [0, 0, 1]);
        roller.roll(&[2, 3]);
        assert_eq!(roller.data, [1, 2, 3]);
        roller.roll(&[4, 5, 6]);
        assert_eq!(roller.data, [4, 5, 6]);
        roller.roll(&[7, 8, 9, 10]);
        assert_eq!(roller.data, [8, 9, 10]);
    }
}
