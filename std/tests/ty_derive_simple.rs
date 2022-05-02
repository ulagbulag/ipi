extern crate ipi_std as ipi;

#[macro_use]
extern crate rkyv;

use bytecheck::CheckBytes;
use ipi::{object::Object, Class};
use ipi_core::value::text::Text;
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
        bool: bool,
        i64: i64,
        u64: u64,
        f32: f32,
        f64: f64,
        bytes: Vec<u8>,
        string: String,
        text: Text,
    }

    let value = MyStruct {
        sub: MySubstruct {
            unit: (),
            bool: true,
            i64: 42,
            u64: 42,
            f32: 42.0,
            f64: 42.0,
            bytes: vec![0x12, 0x34, 0x56, 0x78],
            string: "hello world!".to_string(),
            text: Text::with_en_us("hello world!"),
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
    assert_eq!(archived, &value);
    assert_eq!(archived.sub.i64, 42);
    assert_eq!(&archived.sub.bytes, &[0x12, 0x34, 0x56, 0x78]);
    assert_eq!(&archived.sub.string, "hello world!");

    // And you can always deserialize back to the original type
    let deserialized: MyStruct = archived.deserialize(&mut rkyv::Infallible).unwrap();
    assert_eq!(&deserialized, &value);
}
