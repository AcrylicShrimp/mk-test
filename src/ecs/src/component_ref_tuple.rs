use super::component_ref::ComponentRef;
use super::component_tuple::ComponentTuple;
use std::slice::IterMut;

pub trait ComponentRefTuple<'a>: 'a + Sized {
    type Tuple: ComponentTuple;
    type IterTuple: 'a + Sized;
}

macro_rules! component_ref_tuple_impls {
    (@next) => {};
    (@next $first:ident, $($rest:ident,)*) => {
        component_ref_tuple_impls!($($rest,)*);
    };
    ($($components:ident,)*) => {
        impl<'a, $($components: ComponentRef<'a>,)*> ComponentRefTuple<'a> for ($($components,)*) {
            type Tuple = ($($components::Component,)*);
            type IterTuple = ($(IterMut<'a, $components>,)*);
        }

        component_ref_tuple_impls!(@next $($components,)*);
    };
    () => {};
}

component_ref_tuple_impls!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31,
);
