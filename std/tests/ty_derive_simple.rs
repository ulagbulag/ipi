extern crate ipi_std as ipi;

#[macro_use]
extern crate rkyv;

use bytecheck::CheckBytes;
use ipi::{object::Object, Class};
use rkyv::Deserialize;

#[test]
fn test() {
    #[derive(Class, Debug, PartialEq, Archive, Serialize, Deserialize)]
    #[archive(compare(PartialEq))]
    #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
    pub struct MyStruct {
        sub: MySubstruct,
    }

    #[derive(Class, Debug, PartialEq, Archive, Serialize, Deserialize)]
    #[archive(compare(PartialEq))]
    #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
    pub struct MySubstruct {
        unit: (),
        // bool: bool,
        // i64: i64,
        // u64: u64,
        // f32: f32,
        // f64: f64,
    }

    let value = MyStruct {
        sub: MySubstruct {
            unit: (),
            // bool: true,
            // i64: 42,
            // u64: 42,
            // f32: 42.0,
            // f64: 42.0,
        },
    };

    // Test derived class methods
    assert_eq!(
        MyStruct::class_cursor()
            .sub()
            .unit()
            .__object_name()
            .to_string(),
        "()",
    );
    assert_eq!(
        MyStruct::class_cursor().sub().unit().to_string(),
        "sub.unit",
    );

    // Test derived object methods
    assert_eq!(
        value.cursor().sub().unit().__object_name().to_string(),
        "()",
    );

    // Serializing
    let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();

    // You can use the safe API for fast zero-copy deserialization
    let archived = rkyv::check_archived_root::<MyStruct>(&bytes[..]).unwrap();
    assert_eq!(&value, archived);
    assert_eq!(archived.sub.unit, ());

    // And you can always deserialize back to the original type
    let deserialized: MyStruct = archived.deserialize(&mut rkyv::Infallible).unwrap();
    assert_eq!(&value, &deserialized);
}
