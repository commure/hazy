pub extern crate hazy_derive;
pub use hazy_derive::*;

pub trait OpaqueDebug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result;
}

macro_rules! debug_impls {
    ( $( $t:ty => $val:expr,)* ) => {
        $(
        impl OpaqueDebug for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, $val)
            }
        }
        )*
    };
    ( $( $t:ty,)* ) => {
        debug_impls! {$(
            $t => "_",
        )*}
    };
    ( tuple $($t:ident,)*) => {
        impl<$($t,)*> OpaqueDebug for ($($t,)*) {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "(")?;
                // Getting around the need to use `$t` in the repetition:
                #[allow(non_snake_case)]
                let ($(ref $t,)*) = *self;
                $(
                    let _ = $t;
                    write!(f, "_, ")?;
                )*
                write!(f, ")")
            }
        }
    };
}

debug_impls! {
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    bool,
}

debug_impls!(tuple T0,);
debug_impls!(tuple T0, T1,);
debug_impls!(tuple T0, T1, T2,);
debug_impls!(tuple T0, T1, T2, T3,);
debug_impls!(tuple T0, T1, T2, T3, T4,);
debug_impls!(tuple T0, T1, T2, T3, T4, T5,);
debug_impls!(tuple T0, T1, T2, T3, T4, T5, T6,);
debug_impls!(tuple T0, T1, T2, T3, T4, T5, T6, T7,);
debug_impls!(tuple T0, T1, T2, T3, T4, T5, T6, T7, T8,);
debug_impls!(tuple T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,);
debug_impls!(tuple T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,);
debug_impls!(tuple T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,);

impl OpaqueDebug for ::std::string::String {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if self.is_empty() {
            write!(f, "\"\"")
        } else {
            write!(f, "\"...\"")
        }
    }
}

impl<'a> OpaqueDebug for &'a str {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if self.is_empty() {
            write!(f, "\"\"")
        } else {
            write!(f, "\"...\"")
        }
    }
}

impl<T, E> OpaqueDebug for Result<T, E>
where
    T: OpaqueDebug,
    E: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Ok(ref x) => {
                write!(f, "Ok(")?;
                OpaqueDebug::fmt(x, f)?;
                write!(f, ")")
            }
            Err(ref e) => write!(f, "Err({:?})", e),
        }
    }
}

impl<T> OpaqueDebug for Option<T>
where
    T: OpaqueDebug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Some(ref x) => {
                write!(f, "Some(")?;
                OpaqueDebug::fmt(x, f)?;
                write!(f, ")")
            }
            None => write!(f, "None"),
        }
    }
}

impl<T> OpaqueDebug for Box<T>
where
    T: OpaqueDebug + ?Sized,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Box {{ ")?;
        OpaqueDebug::fmt(&**self, f)?;
        write!(f, " }}")
    }
}

impl<T> OpaqueDebug for Vec<T>
where
    T: OpaqueDebug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self.len() {
            x if x < 13 => {
                write!(f, "[")?;
                let mut it = self.iter().peekable();
                while let Some(ref next) = it.next() {
                    OpaqueDebug::fmt(next, f)?;
                    if it.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
                Ok(())
            }
            _ => write!(f, "[...]"),
        }
    }
}

impl<T> OpaqueDebug for [T]
where
    T: OpaqueDebug,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self.len() {
            x if x < 13 => {
                write!(f, "[")?;
                let mut it = self.iter().peekable();
                while let Some(ref next) = it.next() {
                    OpaqueDebug::fmt(next, f)?;
                    if it.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")?;
                Ok(())
            }
            _ => write!(f, "[...]"),
        }
    }
}

impl<'a, T: ?Sized + OpaqueDebug> OpaqueDebug for &'a T {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        OpaqueDebug::fmt(&**self, f)
    }
}

impl<'a, T: ?Sized + OpaqueDebug> OpaqueDebug for &'a mut T {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        OpaqueDebug::fmt(&**self, f)
    }
}
