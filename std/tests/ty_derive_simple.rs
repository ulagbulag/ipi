#[macro_use]
extern crate ipi_std as ipi;

#[macro_use]
extern crate rkyv;

use bytecheck::CheckBytes;

#[test]
fn test() {
    #[derive(Debug, PartialEq, Archive, Serialize, Deserialize, Class)]
    #[archive(compare(PartialEq))]
    #[archive_attr(derive(CheckBytes, Debug))]
    struct MyStruct {
        sub: MySubstruct,
    }

    #[derive(Debug, PartialEq, Archive, Serialize, Deserialize, Class)]
    #[archive(compare(PartialEq))]
    #[archive_attr(derive(CheckBytes, Debug))]
    struct MySubstruct {
        unit: (),
        bool: bool,
        i64: i64,
        u64: u64,
        f32: f32,
        f64: f64,
    }

    let value = MyStruct {
        sub: MySubstruct {
            unit: (),
            bool: true,
            i64: 42,
            u64: 42,
            f32: 42.0,
            f64: 42.0,
        },
    };

    // Serializing is as easy as a single function call
    let bytes = rkyv::to_bytes::<_, 256>(&value).unwrap();

    // You can use the safe API for fast zero-copy deserialization
    let archived = rkyv::check_archived_root::<MyStruct>(&bytes[..]).unwrap();
    dbg!(archived.sub.bool);
    assert_eq!(archived, &value);
}
