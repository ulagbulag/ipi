use ipi_core::anyhow::Result;

use super::Class;

impl Class for () {
    type Cursor = ();

    fn __class_name() -> Result<super::metadata::ClassName> {
        todo!()
    }

    fn __class_doc() -> Result<super::metadata::ClassDoc> {
        todo!()
    }

    fn __class_value_ty() -> ipi_core::value::ValueType {
        todo!()
    }

    fn __class_children() -> Result<Vec<super::metadata::ClassMetadata>> {
        todo!()
    }

    fn __class_metadata() -> Result<super::metadata::ClassMetadata> {
        todo!()
    }

    fn __class_metadata_leaf() -> Result<super::metadata::ClassLeaf> {
        todo!()
    }

    fn cursor() -> <Self as Class>::Cursor {
        todo!()
    }
}
