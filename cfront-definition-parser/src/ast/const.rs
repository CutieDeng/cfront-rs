use std::marker::PhantomData;

pub enum Const<'a> {
    NoImpl(PhantomData<&'a !>)
}

