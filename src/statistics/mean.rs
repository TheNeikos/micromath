/// Arithmetic mean

/// Provide the Mean trait for Iterators
pub trait MeanExt: Iterator {
    /// Compute the arithmetic mean of this iterator
    fn mean(self) -> f32
    where
        Self: Sized,
        f32: Mean<Self::Item>,
    {
        f32::mean(self)
    }
}

impl<I: Iterator> MeanExt for I {}

/// Types for which a mean can be calculated
pub trait Mean<A = Self> {
    /// Compute the arithmetic mean of the given iterator
    fn mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = A>;
}

impl Mean for f32 {
    fn mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = f32>,
    {
        let mut num_items = 0;
        let mut sum = 0.0;

        for item in iter {
            num_items += 1;
            sum += item;
        }

        sum / (num_items as f32)
    }
}

impl<'a> Mean<&'a f32> for f32 {
    fn mean<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a f32>,
    {
        iter.copied().mean()
    }
}

#[cfg(test)]
mod tests {
    use super::MeanExt;

    #[test]
    fn mean_test() {
        assert_eq!([1.0, 3.0, 5.0].iter().mean(), 3.0);

        assert_eq!(IntoIterator::into_iter([1.0, 3.0, 5.0]).mean(), 3.0);
    }
}
