/// crate
extern crate rand;
extern crate my_crate as thing;
extern crate std; // implicitly added to the root of every Rust project

pub(crate) use std::io::Error as IoError;
pub(crate) enum CoolMarkerType { }
pub struct PublicThing {
    pub(crate) semi_secret_thing: bool,
}

// 1. Absolute path starting from the current crate root
use crate::utils::helper;

// 2. Visibility restricted to the current crate
pub(crate) fn internal_helper() { ... }

// 3. (Older style) declaring an external crate
extern crate rand;          // rarely needed since Rust 2018
extern crate my_crate as mc;

/// dyn
struct Sheep {}
struct Cow {}
trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}
// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}
// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}
// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}
fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

impl Trait;
dyn Trait;

&dyn Trait;

// &dyn -> T.
// vtable -> destructor, size, align, method1, method2.

Box<dyn Trait>;

// dyn -> T.
// vtable -> destructor, size, align, method1, method2.

/// enum
enum P {
  A = 1,
  B = 2,
  C = 3,
}
impl T<i32> for P {
  type Error = ValidationError;
  fn t(value: i32) -> Result<Self, Self::Error> {
    match value {
      1 -> Ok(P::A),
      2 -> Ok(P::B),
      3 -> Ok(P::C),
      _ -> Err(ValidationError::new(code: "")),
    }
  }
}

enum SimpleEnum {
    FirstVariant,
    SecondVariant,
    ThirdVariant,
}

enum Location {
    Unknown,
    Anonymous,
    Known(Coord),
}

enum ComplexEnum {
    Nothing,
    Something(u32),
    LotsOfThings {
        usual_struct_stuff: bool,
        blah: String,
    }
}

enum EmptyEnum { }

/// impl
struct Example {
    number: i32,
}

impl Example {
    fn boo() {
        println!("boo! Example::boo() was called!");
    }

    fn answer(&mut self) {
        self.number += 42;
    }

    fn get_number(&self) -> i32 {
        self.number
    }
}

struct Example {
    number: i32,
}

trait Thingy {
    fn do_thingy(&self);
}

impl Thingy for Example {
    fn do_thingy(&self) {
        println!("doing a thing! also, number is {}!", self.number);
    }
}

fn thing_returning_closure() -> impl Fn(i32) -> bool {
    println!("here's a closure for you!");
    |x: i32| x % 3 == 0
}

/// loop
loop {
    println!("hello world forever!");
}

let mut i = 1;
loop {
    println!("i is {i}");
    if i > 100 {
        break;
    }
    i *= 2;
}
assert_eq!(i, 128);

let mut i = 1;
let something = loop {
    i *= 2;
    if i > 100 {
        break i;
    }
};
assert_eq!(something, 128);

/// match
let opt: Option<usize> = None;
let x = match opt {
    Some(int) => int,
    None => 10,
};
assert_eq!(x, 10);

let a_number = Some(10);
match a_number {
    Some(x) if x <= 5 => println!("0 to 5 num = {x}"),
    Some(x @ 6..=10) => println!("6 to 10 num = {x}"),
    None => panic!(),
    // all other numbers
    _ => panic!(),
}

enum Outer {
    Double(Option<u8>, Option<String>),
    Single(Option<u8>),
    Empty
}

let get_inner = Outer::Double(None, Some(String::new()));
match get_inner {
    Outer::Double(None, Some(st)) => println!("{st}"),
    Outer::Single(opt) => println!("{opt:?}"),
    _ => panic!(),
}

/// mod
mod foo {
    mod bar {
        type MyType = (u8, u8);
        fn baz() {}
    }
}

/// move
let data = vec![1, 2, 3];
let closure = move || println!("captured {data:?} by value");

// data is no longer available, it is owned by the closure

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {text}")
}

let fn_plain = create_fn();
fn_plain();

let data = vec![1, 2, 3];

std::thread::spawn(move || {
    println!("captured {data:?} by value")
}).join().unwrap();

// data was moved to the spawned thread, so we cannot use it here

let capture = "hello".to_owned();
let block = async move {
    println!("rust says {capture} from async block");
};

/// mut
// A mutable variable in the parameter list of a function.
fn foo(mut x: u8, y: u8) -> u8 {
    x += y;
    x
}

// Modifying a mutable variable.
let mut a = 5;
a = 6;

assert_eq!(foo(3, 4), 7);
assert_eq!(a, 6);

// Taking a mutable reference.
fn push_two(v: &mut Vec<u8>) {
    v.push(2);
}

// A mutable reference cannot be taken to a non-mutable variable.
let mut v = vec![0, 1];
// Passing a mutable reference.
push_two(&mut v);

assert_eq!(v, vec![0, 1, 2]);

// Fail to compile
let mut v = vec![0, 1];
let mut_ref_v = &mut v;
let ref_v = &v;
mut_ref_v.push(2);

/// ref
// Fail to compile
let maybe_name = Some(String::from("Alice"));
// The variable 'maybe_name' is consumed here ...
match maybe_name {
    Some(n) => println!("Hello, {n}"),
    _ => println!("Hello, world"),
}
// ... and is now unavailable.
println!("Hello again, {}", maybe_name.unwrap_or("world".into()));

let maybe_name = Some(String::from("Alice"));
// Using `ref`, the value is borrowed, not moved ...
match maybe_name {
    Some(ref n) => println!("Hello, {n}"),
    _ => println!("Hello, world"),
}
// ... so it's available here!
println!("Hello again, {}", maybe_name.unwrap_or("world".into()));

/// struct
struct Regular {
    field1: f32,
    field2: String,
    pub field3: bool
}

struct Tuple(u32, String);

struct Unit;

example_struct.field;
foo.bar = 2;

let T(x, y) = foo;
foo.0;
foo.1;

let example = Foo {
    field1: 42.0,
    field2: "blah".to_string(),
    etc: true,
};

struct User {
    name: String,
    admin: bool,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
            admin: false,
        }
    }
}

let updated_thing = Foo {
    field1: "a new value".to_string(),
    ..thing
};

Foo(123, false, 0.1);

let t = EmptyStruct;

/// super
mod a {
    pub fn foo() {}
}
mod b {
    pub fn foo() {
        super::a::foo(); // call a's foo function
    }
}

/// trait
trait Zero {
    const ZERO: Self;
    fn is_zero(&self) -> bool;
}

impl Zero for i32 {
    const ZERO: Self = 0;

    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

assert_eq!(i32::ZERO, 0);
assert!(i32::ZERO.is_zero());
assert!(!4.is_zero());

trait Builder {
    type Built;

    fn build(&self) -> Self::Built;
}

trait MaybeFrom<T> {
    fn maybe_from(value: T) -> Option<Self>
    where
        Self: Sized;
}

trait ThreeIterator: Iterator {
    fn next_three(&mut self) -> Option<[Self::Item; 3]>;
}

fn debug_iter<I: Iterator>(it: I) where I::Item: std::fmt::Debug {
    for elem in it {
        println!("{elem:#?}");
    }
}

// u8_len_1, u8_len_2 and u8_len_3 are equivalent

fn u8_len_1(val: impl Into<Vec<u8>>) -> usize {
    val.into().len()
}

fn u8_len_2<T: Into<Vec<u8>>>(val: T) -> usize {
    val.into().len()
}

fn u8_len_3<T>(val: T) -> usize
where
    T: Into<Vec<u8>>,
{
    val.into().len()
}

fn from_zero_to(v: u8) -> impl Iterator<Item = u8> {
    (0..v).into_iter()
}

// Fail to compile
trait A {}
trait B {}

let _: Box<dyn A + B>;

// Fail to compile
trait A {}
trait B {}

let _: Box<dyn A + dyn B>;

// Correct
trait A {}

let _: Box<dyn A + Send + Sync>;

trait NewTrait: A + B {};

unsafe trait UnsafeTrait {}
unsafe impl UnsafeTrait for i32 {}


struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}


/// type
type Meters = u32;
type Kilograms = u32;

let m: Meters = 3;
let k: Kilograms = 3;

assert_eq!(m, k);

type ArcMutex<T> = Arc<Mutex<T>>;

trait Iterator {
    // associated type declaration
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Once<T>(Option<T>);

impl<T> Iterator for Once<T> {
    // associated type definition
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take()
    }
}

/// use
use path::to::item;
use a::b::{c, d, e::f, g::h::i};
use a::b::{self, c, d::e};
use p::q::r as x;
use a::b::{self as ab, c as abc};
use a::b::*;
use a::b::{self as ab, c, d::{*, e::f}};
pub use a::b;
use ::std::io::Read as _;
use crate::a::b;
use super::a::b;
use self::a::b;

// Fail to compile
enum ExampleEnum {
    VariantA,
    VariantB,
}

impl ExampleEnum {
    fn new() -> Self {
        Self::VariantA
    }
}

use ExampleEnum::*;

// Compiles.
let _ = VariantA;

// Does not compile!
let n = new();

// Fail to compile
fn f(x: &()) -> impl Sized { x }

// Correct
fn f(x: &()) -> impl Sized + use<'_> { x }

/// where
fn new<T: Default>() -> T {
    T::default()
}

fn new_where<T>() -> T
where
    T: Default,
{
    T::default()
}

assert_eq!(0.0, new());
assert_eq!(0.0, new_where());

assert_eq!(0, new());
assert_eq!(0, new_where());

fn select<'short, 'long>(s1: &'short str, s2: &'long str, second: bool) -> &'short str
where
    'long: 'short,
{
    if second { s2 } else { s1 }
}

let outer = String::from("Long living ref");
let longer = &outer;
{
    let inner = String::from("Short living ref");
    let shorter = &inner;

    assert_eq!(select(shorter, longer, false), shorter);
    assert_eq!(select(shorter, longer, true), longer);
}

// fail to compile
fn select<'a, 'b>(s1: &'a str, s2: &'b str, second: bool) -> &'a str
{
    if second { s2 } else { s1 }
}

fn first_or_default<I>(mut i: I) -> I::Item
where
    I: Iterator,
    I::Item: Default,
{
    i.next().unwrap_or_else(I::Item::default)
}

assert_eq!(first_or_default([1, 2, 3].into_iter()), 1);
assert_eq!(first_or_default(Vec::<i32>::new().into_iter()), 0);

pub enum Cow<'a, B>
where
    B: ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
