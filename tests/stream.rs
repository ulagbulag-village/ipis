#[macro_use]
extern crate rkyv;

use std::io::Cursor;

use bytecheck::CheckBytes;
use ipi::value::text::Text;
use ipis::{class::Class, core::signed::IsSigned, stream::DynStream, tokio};

#[tokio::test]
async fn test() {
    #[derive(Class, Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
    #[archive(compare(PartialEq))]
    #[archive_attr(derive(CheckBytes, Debug, PartialEq))]
    pub struct MyStruct {
        sub: MySubstruct,
    }

    impl IsSigned for MyStruct {}

    #[derive(Class, Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
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

    // pack
    let mut buf: Vec<u8> = vec![];
    let mut stream = DynStream::Borrowed(&value);
    stream
        .copy_to(&mut buf)
        .await
        .expect("failed to copy to the buffer");

    // unpack
    let mut stream: DynStream<MyStruct> = DynStream::recv(Cursor::new(buf))
        .await
        .expect("failed to copy from the buffer");
    let unpacked = stream
        .to_owned()
        .await
        .expect("failed to parse from buffer");
    assert_eq!(&value, &unpacked);
}
