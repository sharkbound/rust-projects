// use paste;

use paste::paste;
#[macro_export]
macro_rules! declare_vars {
    ($type:ty; $($element:ident),+) => {
        $(let $element: $type = Default::default();)+
    };
    ($type:ty => $default:expr; $($element:ident),+) => {
        $(let $element: $type = $default;)*
    }
}
#[macro_export]
macro_rules! declare_vars_mut {
    ($type:ty; $($element:ident),+) => {
        $(let mut $element: $type = Default::default();)+
    };
    ($type:ty => $default:expr; $($element:ident),+) => {
        $(let mut $element: $type = $default;)*
    }
}


// this macro is rather useless, just me trying stuff out
#[macro_export]
macro_rules! declare_mut_accessors {
    ($($field:ident:$type:ty);+) => {
        paste!( // the paste! macro is used for IDENT concatenation
            $(pub fn [<$field _mut>](&mut self) -> &mut $type {
                &mut self.$field
            })+
        );
    };
}

#[macro_export]
macro_rules! declare_impl_new {
    ($($field:ident:$type:ty);+) => {
        pub fn new($($field: $type),+) -> Self {
            Self {
                $($field: $field),+
            }
        }
    };
}
