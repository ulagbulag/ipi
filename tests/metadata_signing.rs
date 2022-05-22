use bytecheck::CheckBytes;
use ipi::{
    account::{Account, GuarantorSigned, Signer, Verifier},
    metadata::Metadata,
    signed::SERIALIZER_HEAP_SIZE,
};
use rkyv::{de::deserializers::SharedDeserializeMap, Archive, Deserialize, Serialize};

#[test]
fn test_simple() {
    #[derive(Debug, PartialEq, Archive, Serialize, Deserialize)]
    #[archive(compare(PartialEq))]
    #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
    pub struct MyData {
        pub msg: String,
        pub num: u64,
    }

    // create a data
    let data = MyData {
        msg: "Hello world!".to_string(),
        num: 42,
    };

    // create a builder
    let builder = Metadata::builder();

    // create client pair
    let guarantee = Account::generate();
    let guarantor = Account::generate();

    // sign as guarantee
    let signed = builder
        .build(&guarantee, guarantor.account_ref(), data)
        .unwrap();

    // sign as guarantor
    let signed = GuarantorSigned::sign(&guarantor, signed).unwrap();

    // verify
    let () = signed.verify(Some(guarantor.account_ref())).unwrap();

    // archive
    let bytes = ::rkyv::to_bytes::<_, SERIALIZER_HEAP_SIZE>(&signed).unwrap();
    let archived = ::rkyv::check_archived_root::<GuarantorSigned<MyData>>(&bytes[..]).unwrap();

    // deserialize
    let deserialized: GuarantorSigned<MyData> =
        Deserialize::deserialize(archived, &mut SharedDeserializeMap::default()).unwrap();
    assert_eq!(&signed, &deserialized);
}
