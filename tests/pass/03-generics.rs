// Test provided in #4
// https://github.com/jmg-duarte/sealed-rs/issues/4
mod basic {
    use sealed::sealed;

    #[sealed]
    pub trait Set<V> {}
    // pub trait Set<V>: __seal_for_set::Sealed<V> {}
    // mod __seal_for_set {
    //     pub trait Sealed<V> {}
    // }

    #[sealed]
    impl<T> Set<Option<T>> for T {}
    // impl<T> __seal_for_set::Sealed<Option<T>> for T {}

    #[sealed]
    impl<T> Set<Option<T>> for Option<T> {}
    // impl<T> __seal_for_set::Sealed<Option<T>> for Option<T> {}
}

// Test where clause.
mod where_clause {
    use sealed::sealed;

    #[sealed]
    pub trait AsRef<T>
    where
        T: ?Sized,
    {
        fn as_ref(&self) -> &T;
    }

    #[sealed]
    impl AsRef<str> for String {
        fn as_ref(&self) -> &str {
            &*self
        }
    }
}

// Test lifetime parameters.
mod lifetime {
    use sealed::sealed;

    #[sealed]
    pub trait Deserialize<'de>: Sized {
        fn deserialize<D>(deserializer: D) -> Option<Self>;
    }

    #[sealed]
    impl<'de> Deserialize<'de> for () {
        fn deserialize<D>(_deserializer: D) -> Option<Self> {
            Some(())
        }
    }
}

// Test const generic parameter.
mod const_generic {
    use sealed::sealed;

    #[sealed]
    pub trait FromArray<T, const N: usize> {
        fn from(arr: [T; N]) -> Self;
    }

    #[sealed]
    impl<T, const N: usize> FromArray<T, N> for Vec<T> {
        fn from(arr: [T; N]) -> Self {
            IntoIterator::into_iter(arr).collect()
        }
    }
}

fn main() {}
