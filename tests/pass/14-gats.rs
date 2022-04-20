// Tests generic associated types (GATs) with where clauses on the type.

#![feature(generic_associated_types)]

use sealed::sealed;

#[sealed]
pub trait TypeList {
    type Ref<'a>
    where
        Self: 'a;

    fn as_ref<'a>(&'a self) -> Self::Ref<'a>;
}

#[sealed]
impl<X, Rest> TypeList for (X, Rest)
where
    Rest: TypeList,
{
    type Ref<'a> = (&'a X, Rest::Ref<'a>)
    where
        Self: 'a;

    fn as_ref<'a>(&'a self) -> Self::Ref<'a> {
        let (this, rest) = self;
        (this, rest.as_ref())
    }
}

#[sealed]
impl TypeList for () {
    type Ref<'a> = ()
    where
        Self: 'a;

    fn as_ref<'a>(&'a self) -> Self::Ref<'a> {
        *self
    }
}

fn main() {}
