pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Direction {
    pub fn get_reverse(&self) -> Self {
        match self {
            Self::Clockwise => Self::CounterClockwise,
            Self::CounterClockwise => Self::Clockwise,
        }
    }

    fn to_num(&self) -> i32 {
        match self {
            Self::Clockwise => 1,
            Self::CounterClockwise => -1,
        }
    }
}

///
pub struct ReversibleRing<T> {
    vec: Vec<T>,
    index: i32,
    direction: Direction,
}

impl<T> ReversibleRing<T> {
    fn new(vec: Vec<T>) -> Self {
        Self {
            vec,
            // We set the index to -1 so that next() returns 0 first
            index: -1,
            direction: Direction::Clockwise,
        }
    }

    pub fn from_iter<I>(iter: I) -> ReversibleRing<T>
    where
        I: IntoIterator<Item = T>,
    {
        let vec: Vec<T> = iter.into_iter().collect();
        Self::new(vec)
    }

    fn adjust_index(&mut self, step: i32) {
        let len = self.vec.len() as i32;
        self.index += step;
        self.index %= len;
        if self.index < 0 {
            self.index += len
        }
    }

    /// Returns the a mutable reference to the next element
    pub fn next(&mut self) -> &mut T {
        self.skip();
        &mut self.vec[self.index as usize]
    }

    /// Reverses the direction
    pub fn reverse(&mut self) {
        self.direction = self.direction.get_reverse();
    }

    //
    pub fn skip(&mut self) {
        self.adjust_index(self.direction.to_num());
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.vec.iter_mut()
    }
}
