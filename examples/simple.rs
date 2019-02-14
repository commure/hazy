#[macro_use]
extern crate hazy;

#[derive(OpaqueDebug)]
struct SecretDemo<'a, T: ?Sized + ::hazy::OpaqueDebug> {
    a: Box<T>,
    b: u8,
    c: &'a str,
    d: SecretHeapSize,
    #[debug(Clear)]
    see_through: &'a str,
    #[debug(Hidden)]
    completely_hidden: SecretHeapSize,
}

#[derive(OpaqueDebug)]
struct SecretHeapSize {
    e: String,
}

#[derive(OpaqueDebug)]
enum SecretEnum {
    HeapSize { e: String },
}

#[derive(Debug)]
enum Enum {
    HeapSize { e: String },
}

#[derive(OpaqueDebug)]
struct A(usize);

#[derive(OpaqueDebug)]
struct B(#[debug(Clear)] A);

#[derive(OpaqueDebug)]
struct SecretFoo(usize);

#[derive(Debug)]
struct Demo<'a, T: ?Sized + ::std::fmt::Debug> {
    a: Box<T>,
    b: u8,
    c: &'a str,
    d: HeapSize,
}

#[derive(Debug)]
struct HeapSize {
    e: String,
}

#[derive(Debug)]
struct Foo(usize);

fn main() {
    let secret_demo = SecretDemo {
        a: Box::new(b"bytestring".to_vec()).into_boxed_slice(),
        b: 255,
        c: "&'static str",
        d: SecretHeapSize {
            e: "String".to_owned(),
        },
        see_through: "you can see this",
        completely_hidden: SecretHeapSize {
            e: "this entire struct will not be shown at all".into(),
        },
    };
    let demo = Demo {
        a: Box::new(b"bytestring".to_vec()).into_boxed_slice(),
        b: 255,
        c: "&'static str",
        d: HeapSize {
            e: "String".to_owned(),
        },
    };
    // alt output is not yet supported
    println!("{:#?}", secret_demo);
    println!("{:?}", demo);
    println!("{:?}", SecretFoo(3));
    println!("{:?}", Foo(3));
    let a = Enum::HeapSize {
        e: "an enum".to_owned(),
    };
    println!("{:?}", a);
    let e = SecretEnum::HeapSize {
        e: "an enum".to_owned(),
    };
    println!("{:?}", e);
    println!("{:?}", B(A(2)));
}
