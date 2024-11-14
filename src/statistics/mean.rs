/// Arithmetic mean
pub trait Mean<I> {
    /// Result type
    type Result;

    /// Compute the arithmetic mean
    fn mean(self) -> Self::Result;
}

impl<I> Mean<f32> for I
where
    I: Iterator<Item = f32>,
{
    type Result = f32;

    fn mean(self) -> f32 {
        let mut num_items = 0;
        let mut sum = 0.0;

        for item in self {
            num_items += 1;
            sum += item;
        }

        sum / (num_items as f32)
    }
}

impl<'a, I> Mean<&f32> for I
where
    I: Iterator<Item = &'a f32>,
{
    type Result = f32;

    fn mean(self) -> f32 {
        self.copied().mean()
    }
}

#[cfg(test)]
mod tests {
    use super::Mean;

    #[test]
    fn mean_test() {
        assert_eq!([1.0, 3.0, 5.0].iter().mean(), 3.0);

        assert_eq!([1.0, 3.0, 5.0].into_iter().mean(), 3.0);
    }
}
