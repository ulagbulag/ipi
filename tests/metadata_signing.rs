use bytecheck::CheckBytes;
use ipi::{
    account::{Account, Verifier},
    metadata::Metadata,
};
use rkyv::{Archive, Deserialize, Serialize};

#[test]
fn test_simple() {
    #[derive(Debug, Archive, Serialize, Deserialize)]
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

    // sign
    let account = Account::generate();
    let signed = builder.build(&account, data).unwrap();

    // verify
    let () = signed.verify().unwrap();
}
