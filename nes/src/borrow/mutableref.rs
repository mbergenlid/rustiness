use std::ops::{Deref, DerefMut};

pub enum MutableRef<'a, T: ?Sized + 'a> {
    Box(Box<T>),
    Borrowed(&'a mut T),
}

impl<'a, T: ?Sized> Deref for MutableRef<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        match *self {
            MutableRef::Box(ref b) => b.deref(),
            MutableRef::Borrowed(ref t) => t,
        }
    }
}

impl<'a, T: ?Sized> DerefMut for MutableRef<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        match *self {
            MutableRef::Box(ref mut b) => b.deref_mut(),
            MutableRef::Borrowed(ref mut t) => t,
        }
    }
}

#[cfg(test)]
mod test {
    use super::MutableRef;

    trait SomeTrait {
        fn a_borrow(&self) -> u8;
        fn a_mut_borrow(&mut self) -> u8;
    }

    impl SomeTrait for u8 {
        fn a_borrow(&self) -> u8 {
            *self
        }
        fn a_mut_borrow(&mut self) -> u8 {
            *self
        }
    }

    #[test]
    fn test() {
        let v = &mut 6;
        let mut_ref_box: MutableRef<SomeTrait> = MutableRef::Box(Box::new(5));
        let mut_ref_borrowed: MutableRef<SomeTrait> = MutableRef::Borrowed(v);

        assert_eq!(5, mut_ref_box.a_borrow());
        assert_eq!(6, mut_ref_borrowed.a_borrow());
    }

    #[test]
    fn test_mutable_references_in_vector() {
        let v = &mut 6;
        let mut vector: Vec<MutableRef<SomeTrait>> =
            vec![MutableRef::Box(Box::new(5)), MutableRef::Borrowed(v)];

        assert_eq!(5, vector[0].a_borrow());
        assert_eq!(6, vector[1].a_borrow());

        assert_eq!(5, vector[0].a_mut_borrow());
        assert_eq!(6, vector[1].a_mut_borrow());
    }
}
