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
        data: 42,
    };

    let signed = ::ipi::account::GuaranteeSigned::sign(&account, metadata).unwrap();
    let signed = ::rkyv::to_bytes::<_, 4096>(&signed).unwrap();

    let bytes = &[
        85, 14, 132, 0, 226, 155, 65, 212, 167, 22, 68, 102, 85, 68, 0, 0, 0, 0, 0, 0, 24, 250,
        181, 234, 16, 84, 232, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 178, 127, 84, 7, 76, 6, 240, 252, 66, 76, 107, 153, 78,
        227, 199, 47, 255, 205, 198, 205, 169, 240, 131, 27, 107, 97, 3, 20, 99, 143, 106, 117, 0,
        0, 0, 0, 178, 127, 84, 7, 76, 6, 240, 252, 66, 76, 107, 153, 78, 227, 199, 47, 255, 205,
        198, 205, 169, 240, 131, 27, 107, 97, 3, 20, 99, 143, 106, 117, 20, 155, 255, 188, 70, 111,
        125, 199, 143, 89, 40, 85, 122, 83, 50, 246, 101, 130, 239, 19, 255, 248, 252, 51, 33, 3,
        53, 39, 207, 68, 254, 105, 94, 185, 75, 147, 107, 196, 15, 27, 11, 90, 199, 243, 72, 232,
        214, 40, 60, 73, 20, 91, 240, 36, 152, 59, 252, 45, 63, 65, 220, 234, 217, 15,
    ];
    assert_eq!(signed.as_slice(), bytes);
}
