pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clamp_test() {
        assert_eq!(clamp(2., 1., 3.), 2.);
        assert_eq!(clamp(100, 1, 3), 3);
        assert_eq!(clamp(-1, 1, 3), 1);
    }
}
