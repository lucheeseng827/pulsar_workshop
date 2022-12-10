use example::Foo;
use prost::Message;

let foo = Foo {
    value: "hello".to_string(),
};
let encoded = foo.encode();
