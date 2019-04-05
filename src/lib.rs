#[derive(Debug, PartialEq)]
pub enum Never {}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn size_of_never_is_zero() {
        assert_eq!(0, size_of::<Never>());
    }
}
