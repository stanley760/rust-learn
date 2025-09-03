use std::marker::PhantomData;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Identifier<T> {
    inner: u64,
    _marker: PhantomData<T>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    id: Identifier<Self>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Product {
    id: Identifier<Self>,
}

#[derive(Debug, Default)]
pub struct Equation<T> {
    current: u32,
    _marker: PhantomData<T>,
}

#[derive(Debug, Default)]
pub struct Linear;

#[derive(Debug, Default)]
pub struct Quadratic;

impl Iterator for Equation<Linear> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None;
        }
        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        let result = self.current * self.current;
        if result >= u32::MAX {
            return None;
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_equals() {
        let user = User::default();
        let product = Product::default();

        assert_eq!(user.id.inner, product.id.inner);
    }

    #[test]
    fn test_linear() {
        let mut linear_eq = Equation::<Linear>::default();
        assert_eq!(linear_eq.next(), Some(1));
        assert_eq!(linear_eq.next(), Some(2));
        assert_eq!(linear_eq.next(), Some(3));
    }

    #[test]
    fn test_quadratic() {
        let mut quadratic_eq = Equation::<Quadratic>::default();
        assert_eq!(quadratic_eq.next(), Some(1));
        assert_eq!(quadratic_eq.next(), Some(4));
        assert_eq!(quadratic_eq.next(), Some(9));
    }
}
