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

// TODO: not yet implemented
// #[derive(OpaqueDebug)]
// enum SecretHeapSize {
//     HeapSize {
//         e: String,
//     }
// }

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
        }
    };
    let demo = Demo {
        a: Box::new(b"bytestring".to_vec()).into_boxed_slice(),
        b: 255,
        c: "&'static str",
        d: HeapSize {
            e: "String".to_owned(),
        },
    };
    println!(
        "{:#?}",
        secret_demo,
    );
    println!(
        "{:?}",
        demo,
    );
    println!(
        "{:?}",
        SecretFoo(3),
    );
    println!(
        "{:?}",
        Foo(3),
    );
}