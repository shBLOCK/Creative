#![allow(non_snake_case)]
#![allow(unused_macros)]

derive_aliases::define! {
    Eq = ::core::cmp::PartialEq, ::core::cmp::Eq;
    Ord = ..Eq, ::core::cmp::PartialOrd, ::core::cmp::Ord;
    Copy = ::core::marker::Copy, ::core::clone::Clone;
    DerefMut = ::derive_more::Deref, ::derive_more::DerefMut;
    NumOps = ::derive_more::Add, ::derive_more::Sub, ::derive_more::Mul, ::derive_more::Div, ::derive_more::Rem;
    NumOpsAssign = ..NumOps, ::derive_more::AddAssign, ::derive_more::SubAssign, ::derive_more::MulAssign, ::derive_more::DivAssign, ::derive_more::RemAssign;
    SerDe = ::serde::Serialize, ::serde::Deserialize;
}
