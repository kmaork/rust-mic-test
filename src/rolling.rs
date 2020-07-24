#[derive(Debug)]
pub struct RollingData<T> {
    data: Vec<T>
}

impl<T: Copy> RollingData<T> {
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

impl<T> RollingData<T> {
    pub fn data(&self) -> &[T] {
        &self.data
    }
}

impl<T: Clone> RollingData<T> {
    pub fn new(size: usize, fill: T) -> Self {
        Self { data: vec![fill; size] }
    }
}

impl<T: Default + Clone> RollingData<T> {
    pub fn new_default(size: usize) -> Self {
        Self::new(size, T::default())
    }
}

#[cfg(test)]
mod test {
    use super::RollingData;

    #[test]
    fn roll() {
        let mut rolling: RollingData<i16> = RollingData::new_default(3);
        assert_eq!(rolling.data, [0, 0, 0]);
        rolling.roll(&[1]);
        assert_eq!(rolling.data, [0, 0, 1]);
        rolling.roll(&[2, 3]);
        assert_eq!(rolling.data, [1, 2, 3]);
        rolling.roll(&[4, 5, 6]);
        assert_eq!(rolling.data, [4, 5, 6]);
        rolling.roll(&[7, 8, 9, 10]);
        assert_eq!(rolling.data, [8, 9, 10]);
    }
}
