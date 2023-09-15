#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().cloned()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().cloned()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut numbers = self.numbers.to_vec();
        numbers.sort_unstable_by(|a, b| b.cmp(a));
        numbers.into_iter().take(3).collect()
    }
}