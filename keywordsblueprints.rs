// as keyword
let thing1: u8 = 89.0 as u8;
let x = 123 as u32
let x: u32 = 123
assert_eq!('B' as u32, 66);
assert_eq!(thing1 as char, 'Y');
let thing2: f32 = thing1 as f32 + 10.5;
assert_eq!(true as u8 + thing2 as u8, 100);
(From) u8, bool, str, *ptr, *const T, *mut T, &my, &mut my, String, Vec as (Into) u8, bool, str, *ptr, *const T, *mut T, &my, &mut my, String, Vec
let thing3: u32 as *const _;
let thing3: u32 = 89.0 as *const _;
let thing3 = 89.0 as *const _;
let thing3: u32 as *mut _;
let thing3: u32 = 89.0 as *mut _;
let thing3 = 89.0 as *mut _;
use std::{mem as memory, net as network};
extern crate std::{mem as memory, net as network};
Into::<u32>::into
Into::<u64>::into
<MyThing as Into<u32>>::into(my_thing)
T::from(my_thing)
Into::into

// async/await keyword
async fn function() -> i32 {
  let x = async {
    return 1;
  };
  x.await
}
async fn function ( let var: u64 = var_outside as u64; ) {
  if u64 == 1 { return 0; };
  var.await
}; 
gen || {}
hir::Closure
ClosureKind::CoroutineClosure(_)
ClosureKind::Closure(CoroutineKind::Desugared(_, CoroutineSource::Closure))

fn(tupled_inputs, resume_ty) -> (return_ty, yield_ty)

let c = async || -> String { .. };
let s = c().await;

fn force_fnonce<T: async FnOnce()>(t: T) -> T { t }
let x = String::new();
let c = force_fnonce(async move || {
  println!("{x}");
})

let x = String::new();
let y = String::new();
let c = async move || {
  drop(y);
  println!("{x}");
};

let x = &1i32; // Let's call this lifetime `'1`.
let c = async move || {
    println!("{:?}", *x);
    // Even though the inner coroutine borrows by ref, we're only capturing `*x`,
    // not `x`, so the inner closure is allowed to reborrow the data for `'1`.
};

let mut x = 1i32;
let c = async || {
    x = 1;
    // The parent borrows `x` for some `&'1 mut i32`.
    // However, when we call `c()`, we implicitly autoref for the signature of
    // `AsyncFnMut::async_call_mut`. Let's call that lifetime `'call`. Since
    // the maximum that `&'call mut &'1 mut i32` can be reborrowed is `&'call mut i32`,
    // the inner coroutine should capture w/ the lifetime of the coroutine-closure.
};

s.as_bytes();
F: AsyncFN*() -> T;
F: Fn*() -> T, T: Future<Output = U>
Option::map
AsyncFn*() -> T
Fn*() -> Fut
Future<Output = T>

u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, char, bool
const async extern let move mut pub static
const fn IDENTIFIER<const N: i64>() {}; 
const fn IDENTIFIER<const N: usize>() -> [u8; {N + 1}] {}; 
async fn IDENTIFIER<const N: i64>() {}; 
extern fn IDENTIFIER<const N: i64>() {}; 
fn foo<const N: usize>(arr: [i32; N]) {}; 
fn first((value, _): (i32, i32)) -> i32 { value };
fn foo<const N: usize>() {};
fn bad_function<const N: usize>() -> [u8; {N + 1}] {};
fn f<const N: i64>() -> S<N> { S };
fn make_buf<const N: usize>() -> [u8; N] {};
fn f<const N: usize>() -> [u8; N] { [0; _] };
fn f<const N: usize>(x: [u8; N]) -> [u8; _] { x };
fn bar<const N: usize>() -> Foo<{ N }> { todo!() }; // ok
fn needs_bar(_: impl Bar) {};
fn generic<const B: bool>() {};
fn foo<'a, T>() {};
fn f<const N: usize>(x: [u8; N]) -> [u8; _] { x }
//                                       ^ ERROR not allowed

Function -> FunctionQualifiers fn IDENTIFIER GenericParams ( FunctionParameters ) {
  FunctionReturnType WhereClause
  ( BlockExpression | ; )
}

FunctionQualifiers -> const async ItemSafety ( extern Abi ) BARDZO WAŻNE

ItemSafety -> safe | unsafe                                 BARDZO WAŻNE

Abi -> STRING_LITERAL | RAW_STRING_LITERAL                  BARDZO WAŻNE

FunctionParameters -> SelfParam ,
  | ( SelfParam , ) functionParam ( , FunctionParam )* ,

SelfParam -> OuterAttribute* ( ShorthandSelf | TypedSelf )  BARDZO WAŻNE

ShorthandSelf -> ( & | & Lifetime ) mut self

TypedSelf -> mut self : Type

FunctionParamPattern -> PatternNoTopAlt : ( Type | ... )

FunctionReturnType -> Type
