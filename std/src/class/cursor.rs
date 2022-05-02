use super::metadata::ClassLeaf;

#[derive(
    Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Archive, Serialize, Deserialize,
)]
#[archive(compare(PartialEq, PartialOrd))]
#[archive_attr(derive(CheckBytes, Debug, PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct ClassCursorData(Vec<ClassLeaf>);

impl ::core::fmt::Display for ClassCursorData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        for (idx, leaf) in self.0.iter().enumerate() {
            ::core::fmt::Display::fmt(leaf, f)?;
            if idx > 0 {
                ::core::fmt::Display::fmt(".", f)?;
            }
        }
        Ok(())
    }
}

impl ClassCursorData {
    pub fn push(&mut self, value: ClassLeaf) {
        self.0.push(value);
    }
}
