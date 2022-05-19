pub type Serializer = ::rkyv::ser::serializers::AllocSerializer<SERIALIZER_HEAP_SIZE>;

pub const SERIALIZER_HEAP_SIZE: usize = 4096;

pub trait IsSigned {
    fn is_signed() -> bool {
        false
    }

    fn is_signed_dyn(&self) -> bool {
        Self::is_signed()
    }

    fn to_bytes(&self) -> Result<::rkyv::AlignedVec, <Serializer as ::rkyv::Fallible>::Error>
    where
        Self: ::rkyv::Serialize<Serializer> + Send + Sync + Sized,
    {
        ::rkyv::to_bytes(self)
    }
}

impl IsSigned for () {}
impl IsSigned for bool {}
impl IsSigned for char {}
impl IsSigned for i8 {}
impl IsSigned for i16 {}
impl IsSigned for i32 {}
impl IsSigned for i64 {}
impl IsSigned for i128 {}
impl IsSigned for isize {}
impl IsSigned for u8 {}
impl IsSigned for u16 {}
impl IsSigned for u32 {}
impl IsSigned for u64 {}
impl IsSigned for u128 {}
impl IsSigned for usize {}
impl IsSigned for str {}
impl IsSigned for String {}

impl<T: ?Sized> IsSigned for ::core::marker::PhantomData<T> {}
impl IsSigned for ::std::net::SocketAddr {}
impl IsSigned for ::std::net::SocketAddrV4 {}
impl IsSigned for ::std::net::SocketAddrV6 {}

macro_rules! impl_for_tuples {
    ( $( $ty:ident ,)* ) => {
        impl< $( $ty: IsSigned ,)* > IsSigned for ( $( $ty ,)* ) {}
    };
}

impl_for_tuples!(T1,);
impl_for_tuples!(T1, T2,);
impl_for_tuples!(T1, T2, T3,);
impl_for_tuples!(T1, T2, T3, T4,);
impl_for_tuples!(T1, T2, T3, T4, T5,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6, T7,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15,);
impl_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16,);

impl<T: IsSigned + ?Sized> IsSigned for &T {
    fn is_signed() -> bool {
        T::is_signed()
    }
}
impl<T: IsSigned> IsSigned for [T] {
    fn is_signed() -> bool {
        T::is_signed()
    }
}
impl<T: IsSigned + ?Sized> IsSigned for ::std::boxed::Box<T> {
    fn is_signed() -> bool {
        T::is_signed()
    }
}
impl<'a, B> IsSigned for ::std::borrow::Cow<'a, B>
where
    B: IsSigned + ::rkyv::Serialize<Serializer> + ToOwned + Send + Sync + Sized + ?Sized + 'a,
{
    fn is_signed() -> bool {
        B::is_signed()
    }
}
impl<T: IsSigned> IsSigned for ::std::pin::Pin<T> {
    fn is_signed() -> bool {
        T::is_signed()
    }
}
impl<T: IsSigned + ?Sized> IsSigned for ::std::rc::Rc<T> {
    fn is_signed() -> bool {
        T::is_signed()
    }
}
impl<T: IsSigned + ?Sized> IsSigned for ::std::sync::Arc<T> {
    fn is_signed() -> bool {
        T::is_signed()
    }
}

impl<T: IsSigned> IsSigned for ::std::vec::Vec<T> {
    fn is_signed() -> bool {
        T::is_signed()
    }
}
impl<K, V: IsSigned> IsSigned for ::std::collections::BTreeMap<K, V> {
    fn is_signed() -> bool {
        V::is_signed()
    }
}
impl<T: IsSigned> IsSigned for ::std::collections::BTreeSet<T> {
    fn is_signed() -> bool {
        T::is_signed()
    }
}
impl<K, V: IsSigned, S> IsSigned for ::std::collections::HashMap<K, V, S> {
    fn is_signed() -> bool {
        V::is_signed()
    }
}
impl<T: IsSigned, S> IsSigned for ::std::collections::HashSet<T, S> {
    fn is_signed() -> bool {
        T::is_signed()
    }
}
impl<T: IsSigned> IsSigned for ::std::collections::VecDeque<T> {
    fn is_signed() -> bool {
        T::is_signed()
    }
}

impl<T: IsSigned> IsSigned for ::core::option::Option<T> {
    fn is_signed() -> bool {
        T::is_signed()
    }
}
impl<T: IsSigned, E> IsSigned for ::core::result::Result<T, E> {
    fn is_signed() -> bool {
        T::is_signed()
    }
}

impl IsSigned for crate::account::Account {}
impl IsSigned for crate::account::AccountRef {}
impl<T> IsSigned for crate::account::GuaranteeSigned<T> {
    fn is_signed() -> bool {
        true
    }
}
impl<T> IsSigned for crate::account::GuarantorSigned<T> {
    fn is_signed() -> bool {
        true
    }
}
impl IsSigned for crate::credit::CreditRating {}
impl IsSigned for crate::credit::CreditRatingPayload {}
impl<T> IsSigned for crate::metadata::Metadata<T> {
    fn is_signed() -> bool {
        true
    }
}

impl IsSigned for crate::value::ValueType {}
impl<A, D: ::ndarray::Dimension> IsSigned for crate::value::array::Array<A, D> {}
impl<A, D> IsSigned for crate::value::array::ArrayRaw<A, D> {}
impl IsSigned for crate::value::chrono::DateTime {}
impl IsSigned for crate::value::chrono::NaiveDateTime {}
impl IsSigned for crate::value::hash::Hash {}
impl IsSigned for crate::value::primitives::U64 {}
impl IsSigned for crate::value::text::LanguageTag {}
impl IsSigned for crate::value::text::Text {}
impl IsSigned for crate::value::text::TextHash {}
impl IsSigned for crate::value::unit_interval::UnitInterval {}
impl IsSigned for crate::value::uuid::Uuid {}
impl IsSigned for crate::value::word::Word {}
impl IsSigned for crate::value::word::WordHash {}
