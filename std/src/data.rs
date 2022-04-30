#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(C)]
pub struct Data<Type = crate::ty::Type, Inner = crate::path::Path> {
    #[serde(rename = "type")]
    pub ty: Type,
    pub inner: Inner,
}

impl<Type, Inner> ::core::ops::Deref for Data<Type, Inner> {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
