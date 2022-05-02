use super::metadata::{ClassLeaf, ClassName};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct ClassCursorData(Vec<ClassCursorLeaf>);

impl ::core::fmt::Display for ClassCursorData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        for (idx, leaf) in self.0.iter().enumerate() {
            if idx > 0 {
                ::core::fmt::Display::fmt(".", f)?;
            }
            ::core::fmt::Display::fmt(leaf, f)?;
        }
        Ok(())
    }
}

impl ClassCursorData {
    pub fn push_en_us(&mut self, key: impl ToString, value: ClassLeaf) {
        self.0.push(ClassCursorLeaf {
            key: ClassName::with_en_us(key),
            value,
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Archive, Serialize, Deserialize)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, Hash))]
pub struct ClassCursorLeaf {
    pub key: ClassName,
    pub value: ClassLeaf,
}

impl ::core::ops::Deref for ClassCursorLeaf {
    type Target = ClassLeaf;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ::core::fmt::Display for ClassCursorLeaf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.key, f)
    }
}
