use bytecheck::CheckBytes;
use ipi::{
    account::{Account, GuarantorSigned, Signer, Verifier},
    data::Data,
    signed::{IsSigned, SERIALIZER_HEAP_SIZE},
    value::hash::Hash,
};
use rkyv::{de::deserializers::SharedDeserializeMap, Archive, Deserialize, Serialize};

#[test]
fn test_simple() {
    #[derive(Clone, Debug, PartialEq, Eq, Archive, Serialize, Deserialize)]
    #[archive(compare(PartialEq))]
    #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
    pub struct MyData {
        pub msg: String,
        pub num: u64,
    }

    impl IsSigned for MyData {}

    // create a data
    let data = MyData {
        msg: "Hello world!".to_string(),
        num: 42,
    };

    // create a builder
    let builder = Data::builder();

    // create client pair
    let guarantee = Account::generate();
    let guarantor = Account::generate();

    // sign as guarantee
    let signed = builder
        .build(&guarantee, guarantor.account_ref(), &data)
        .unwrap();

    // sign as guarantor
    let signed = signed.sign(&guarantor).unwrap();
    let signed = signed.to_owned();

    // verify
    signed.verify(Some(&guarantor.account_ref())).unwrap();

    // archive
    let bytes = ::rkyv::to_bytes::<_, SERIALIZER_HEAP_SIZE>(&signed).unwrap();
    let archived =
        ::rkyv::check_archived_root::<Data<GuarantorSigned, MyData>>(&bytes[..]).unwrap();

    // deserialize
    let deserialized: Data<GuarantorSigned, MyData> =
        Deserialize::deserialize(archived, &mut SharedDeserializeMap::default()).unwrap();
    assert_eq!(&signed, &deserialized);
}

#[test]
fn test_strict() {
    let account = ::ipi::account::Account {
        keypair: "3z1r5hpJByoqwuBuBWo6vREmpFrCeEXieicJ8gHVpz2ihDhNzCHsV4vUvxuokXAAwJMaDAUP6TJ2PuVGagHjFzsE".parse().unwrap(),
    };
    let metadata = ::ipi::metadata::Metadata {
        nonce: ::ipi::value::nonce::Nonce("550e8400-e29b-41d4-a716-446655440000".parse().unwrap()),
        created_date: ::ipi::value::chrono::DateTime(
            ::ipi::chrono::DateTime::parse_from_str(
                "1983 Apr 13 12:09:14.274 +0000",
                "%Y %b %d %H:%M:%S%.3f %z",
            )
            .unwrap()
            .with_timezone(&::ipi::chrono::Utc),
        ),
        expiration_date: None,
        guarantor: account.account_ref(),
        hash: Hash::with_bytes(&42i32.to_le_bytes()),
    };

    let signed = ::ipi::account::GuaranteeSigned::sign(&account, metadata).unwrap();
    let signed = ::rkyv::to_bytes::<_, 4096>(&signed).unwrap();

    let bytes = &[
        1, 85, 18, 32, 232, 164, 178, 238, 126, 222, 121, 163, 175, 179, 50, 181, 182, 204, 61,
        149, 42, 101, 253, 140, 255, 184, 151, 245, 209, 128, 22, 87, 124, 51, 215, 204, 0, 0, 0,
        0, 178, 127, 84, 7, 76, 6, 240, 252, 66, 76, 107, 153, 78, 227, 199, 47, 255, 205, 198,
        205, 169, 240, 131, 27, 107, 97, 3, 20, 99, 143, 106, 117, 137, 176, 89, 240, 232, 169,
        238, 78, 189, 119, 215, 67, 215, 11, 178, 58, 29, 128, 67, 169, 249, 53, 163, 120, 237, 57,
        14, 226, 105, 206, 206, 100, 198, 51, 207, 148, 239, 161, 77, 21, 4, 87, 236, 239, 160,
        240, 61, 118, 162, 2, 235, 224, 223, 129, 89, 125, 171, 93, 35, 186, 162, 204, 182, 4, 0,
        0, 68, 85, 102, 68, 22, 167, 212, 65, 155, 226, 0, 132, 14, 85, 234, 181, 250, 24, 0, 0, 0,
        0, 128, 232, 84, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 178, 127, 84, 7, 76, 6, 240, 252, 66, 76, 107, 153, 78, 227, 199, 47, 255,
        205, 198, 205, 169, 240, 131, 27, 107, 97, 3, 20, 99, 143, 106, 117, 32, 255, 255, 255, 36,
        0, 0, 0,
    ];
    assert_eq!(signed.as_slice(), bytes);
}
