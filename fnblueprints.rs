Function -> FunctionQualifiers fn IDENTIFIER GenericParams ( FunctionParameters ) {
  FunctionReturnType WhereClause
  ( BlockExpression | ; )
}

const fn i<[<>]>[<>](...<> -> [<>]<>, [<>]<>) -> [<>]<> {};
async fn i<[<>]>[<>](...<> -> [<>]<>, [<>]<>) -> [<>]<> {};
extern fn i<[<>]>[<>](...<> -> [<>]<>, [<>]<>) -> [<>]<> {};




FunctionQualifiers -> const async ItemSafety ( extern Abi ) BARDZO WAŻNE



   
GenericParams -> < ( GenericParam ( , GenericParam )* ,  ) >
GenericParam -> OuterAttribute* ( LifetimeParam | TypeParam | ConstParam )
LifetimeParam -> Lifetime ( : LifetimeBounds )
TypeParam -> IDENTIFIER ( : Bounds ) ( = Type )
ConstParam -> const IDENTIFIER : Type ( = ( BlockExpression | IDENTIFIER | - LiteralExpression ) )

<T>(x1: &Vec<T>) -> f32;
T: Into::<i32> + P<i32> + std::fmt::Debug,
(x1 : &Vec<i32>) -> f32 {};
P::<f32>();
fn F[T any](param T) T {};
<u32, _>;
<const N: i64>;
<const T: u32>([i32; N]);
<const T: u32, const N: i32>;
<const N: usize> Foo<N>;
<const N: usize> Trait for Foo<N> {};
<const N: usize>() -> [u8; {N + 1}] {};
<const N: i64>() -> S<N> { S }
<const N: usize>() -> [u8; N]
<'a, T: AsRef<str>>(a: &'a T, b: &'a T) -> &'a T
<T: PartialOrd + Copy>(items: &[T]) -> T 
<T, U>(a: T, b: U) -> String;
<*ptr=data, &mut>;
<T: U>(f: &mut std::fs::File) -> T;
std::mem::transmutate::<[u8; 4], u32>(bytes);
Foo<{N}> { todo!() };
<'a, T>() {}
<U> {};
<'a, T> where T: 'a { r: &'a T };
<T, const N: usize>([T; N]);
<const N: bool, U>(U);
[u8; 1024] = make_buf::<_>();
f::<1>();
f::<-1>();
f::<{ 1 + 2 }>();
f::<C>();
f::<{ C + 1 }>();
S<1> = f::<_>();
S<1> = f::<(((_)))>();
<const N: usize>() -> [u8; N] { [0; _] };
[u8; 1024] = m::<_>();
<T: Into<String>, 'a: 'b, const LEN: usize>
<'a: 'b + 'c + 'd>;
T: Into<String>;
<const LENGTH: usize>;
impl Clone for GenericParam;
fn clone(&self) -> Self;
fn clone_from(&mut self, source: &Self);
impl Debug for GenericParam;
fn fmt(&self, formatter: &mut Formatter<'_>) -> Result;
impl Eq for GenericParam;
impl Hash for GenericParam;
fn hash<H>(&self, state: &mut H)
where
  H: Hasher,
fn hash_slice<H>(data: &[Self], state: &mut H)
where
  H: Hasher,
  Self: Sized,
impl Parse for GenericParam
fn parse(input: ParseStream<'_>) -> Result<Self>
impl PartialEq for GenericParam
fn eq(&self, other: &Self) -> bool
fn ne(&self, other: &Rhs) -> bool
impl ToTokens for GenericParam
fn to_tokens(&self, tokens: &mut TokenStream)
fn to_token_stream(&self) -> TokenStream
fn into_token_stream(self) -> TokenStream
where
  Self: Sized,
impl !Send for GenericParam;
impl !Sync for GenericParam;
impl Freeze for GenericParam;
impl RefUnwindSafe for GenericParam;
impl Unpin for GenericParam;
impl UnsafeUnpin for GenericParam;
impl UnwindSafe for GenericParam;
impl<T> Any for T
  where
  T: 'static + ?Sized,
fn type_id(&self) -> TypeId;
fn type_id(&self) -> self;
impl<T> Borrow<T> for T
  where
  T: ?Sized,
fn borrow(&self) -> &T;
impl <T> BorrowMut<T> for T
  where
  T: ?Sized,
fn borrow_mut(&mut self) -> &mut T;
impl<T> CloneToUninit for T
  where
  T: Clone,
unsafe fn clone_to_uninit(&self, dest: *mut u8);
impl<T> From<T> for T;
fn from(t: T) -> T;
impl<T, U> Into<U> for T
  where
  U: From<T>,
fn into(self) -> U
  U::from(self)
  From<T> for U,
impl<T> Spanned for T
  where
  T: Spanned + ?Sized,
fn span(&self) -> Span
  Span::call_site(),
impl<T> ToOwned for T
  where
  T: Clone,
type Owned = T;
fn to_owned(&self) -> T;
fn clone_into(&self, target: &mut T);
impl<T, U> TryFrom<U> for T
  where
  U: Into<T>
type Error = !
fn try_from(value: U) -> Result<T, !>
impl<T, U> TryInto<U> for T
  where
  U: TryFrom<T>
type Error = <U as TryFrom<T>>::Error
fn try_into(self) -> Result<U, <U as TryFrom<T>>::Error>


ItemSafety -> safe | unsafe                                 BARDZO WAŻNE

fn a() -> i32 {};

fn f((value, _): (i32, i32)) -> i32 { value };

unsafe extern "C" {
  unsafe fn f1(...);
  unsafe fn f2(ap: ...) {}
}
unsafe extern "C" fn f3(ap: ...) {}
trait Tr {
  unsafe extern "C" fn f4(ap: ...);
}

unsafe extern "C" fn f(...) {} // ERROR: Missing pattern.

trait Tr {
  unsafe extern "C" fn f(...); // ERROR: Missing pattern.
}

// argument_0 is the actual first argument passed from the caller
let (value, _) = argument_0;
return {
    value
};

// foo is generic over A and B
fn foo<A, B>(x: A, y: B) {}

fn foo<T>(x: T) where T: Debug {}

use std::fmt::Debug;
fn foo<T>(x: &[T]) where T: Debug {
    // details elided
}
foo(&[1, 2]);

extern "ABI" fn foo() { /* ... */ }

unsafe extern "ABI" {
  unsafe fn foo(); /* no body */
  safe fn bar(); /* no body */
}
unsafe { foo() };
bar();

fn foo() {}

extern "Rust" fn foo() {}

// Declares a function with the "C" ABI
extern "C" fn new_i32() -> i32 { 0 }
// Declares a function with the "stdcall" ABI
extern "stdcall" fn new_i32_stdcall() -> i32 { 0 }

extern fn new_i32() -> i32 { 0 }
let fptr: extern fn() -> i32 = new_i32;
// is equivalent to:
extern "C" fn new_i32() -> i32 { 0 }
let fptr: extern "C" fn() -> i32 = new_i32;

async fn regular_example() { }
async unsafe fn unsafe_example() { }

// Source
async fn example(x: &str) -> usize {
    x.len()
}

// Desugared
fn example<'a>(x: &'a str) -> impl Future<Output = usize> + 'a {
    async move { x.len() }
}

// Returns a future that, when awaited, dereferences `x`.
//
// Soundness condition: `x` must be safe to dereference until
// the resulting future is complete.
async unsafe fn unsafe_example(x: *const i32) -> i32 {
  *x
}
async fn safe_example() {
    // An `unsafe` block is required to invoke the function initially:
    let p = 22;
    let future = unsafe { unsafe_example(&p) };

    // But no `unsafe` block required here. This will
    // read the value of `p`:
    let q = future.await;
}

unsafe extern "C" fn f(mut ap: ...) -> f64 {
    unsafe { ap.next_arg::<f64>() }
}

unsafe extern "C" fn f(ap: ..., _: ()) {} // ERROR: `...` must be last.

unsafe extern "C" fn f(ap: ...) {
    let _: VaList<'_> = ap;
}

fn b_outlives_a<'a, 'b: 'a>(_: &mut VaList<'a>, _: &mut &'b mut u8) {}
unsafe extern "C" fn f(mut r: &mut u8, mut ap: ...) {
    b_outlives_a(&mut ap, &mut r); // ERROR: May not live long enough.
}

fn a_outlives_b<'a: 'b, 'b>(_: &mut VaList<'a>, _: &mut &'b mut u8) {}
unsafe extern "C" fn f(mut r: &mut u8, mut ap: ...) {
    a_outlives_b(&mut ap, &mut r); // ERROR: May not live long enough.
}

struct MockVaList<'data>(&'data u8);
fn b_outlives_a<'a, 'b: 'a>(_: &mut MockVaList<'a>, _: &mut &'b mut u8) {}
unsafe extern "C" fn f(mut r: &mut u8) {
    let data = 0;
    let mut ap = MockVaList(&data);
    b_outlives_a(&mut ap, &mut r); // OK.
}

unsafe extern "C" fn f(mut ap: ...) -> i32 {
    unsafe { ap.next_arg::<i32>() }
}

use core::intrinsics::{va_arg, va_end};
// `va_start` is magic and has no intrinsic.
fn va_start(ap: *mut VaList<'_>) { /* magic */ }
unsafe extern "C" fn f() -> i32 {
    unsafe {
        let mut ap: MaybeUninit<VaList<'_>> = MaybeUninit::uninit();
        va_start(ap.as_mut_ptr());
        let mut ap = ap.assume_init();
        let x = va_arg::<i32>(&mut ap);
        va_end(&mut ap);
        x
    }
}

unsafe extern "C" {
    // The C `vprintf` function is:
    //     int vprintf(const char *format, va_list ap);
    unsafe fn vprintf(fmt: *const c_char, ap: VaList<'_>) -> c_int;
}
unsafe extern "C" fn print(fmt: *const c_char, ap: ...) -> c_int {
    // The `VaList` is passed directly to the C function.
    unsafe { vprintf(fmt, ap) }
}

unsafe fn f(ap: ...) {} // ERROR: Not supported.

unsafe extern "sysv64" fn f(ap: ...) {} // ERROR: Not supported.

extern "C" fn f(ap: ...) {} // ERROR: Must be `unsafe`.

trait Tr {
    extern "C" fn f(ap: ...); // ERROR: Must be `unsafe`.
}

unsafe extern "C" {
    safe fn f(ap: ...); // OK.
}

async unsafe extern "C" fn f(ap: ...) {} // ERROR: Cannot be `async`.

const unsafe extern "C" fn f(ap: ...) {} // ERROR: Cannot be `const`.

fn documented() {
    #![doc = "Example"]
}

fn len(
    #[cfg(windows)] slice: &[u16],
    #[cfg(not(windows))] slice: &[u8],
) -> usize {
    slice.len()
}

#[some_proc_macro_attribute]
fn foo_oof(#[some_inert_attribute] arg: u8) {
}




Abi -> STRING_LITERAL | RAW_STRING_LITERAL                  BARDZO WAŻNE

macro_rules! blackhole { ($tt:tt) => () }
blackhole!("string"suffix); // OK
r#""#;
let p: &'static str = "";
string.push_str(", ");
let c: &[char] = &[' ', ','];
let t: &str = string.trim(c);
println!("Used characters: {}", t);


FunctionParameters -> SelfParam ,
  | ( SelfParam , ) functionParam ( , FunctionParam )* ,

fn a(w: u32, h: u32) -> u32 {
  w*h
}
fn main() {
  println!("{}", r_a(1, 1));
}

fn h(x: f64) -> f64 {
  x / 2.0
}
fn main() {
  println!("{}", h(10.0));
  println!("{}", h(10 as f64));
}

fn double(x: i32) -> i32 {
  x*2
}
fn main() {
  let n = 1;
  println!("{}", double(n));
  println!("{n}");
}

fn length(s: &String) -> usize {
    s.len()          // read through the borrow
}
fn main() {
    let name = String::from("DevCampus");
    println!("{}", length(&name));   // pass a reference with &
    println!("{name}");              // still usable — we only borrowed it
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}
fn main() {
    let mut msg = String::from("hi");
    append_exclamation(&mut msg);
    println!("{msg}");   // hi!
}

fn countdown(mut n: i32) {
    while n > 0 {
        print!("{n} ");
        n -= 1;         // allowed because parameter is `mut`
    }
    println!();
}

// Parameters are declared name: Type; types are always required and arguments must match exactly.
// Passing by value copies Copy types (integers, bool, ...) but moves others (String, Vec).
// Pass &T to borrow for reading and &mut T to allow modification, keeping ownership with the caller.
// A plain parameter is an immutable binding; add mut to mutate the local copy.




SelfParam -> OuterAttribute* ( ShorthandSelf | TypedSelf )  BARDZO WAŻNE

#[Attr]
Attr -> unsafe ( SimplePath AttrInput = Expression)
AttrInput -> ( TokenTree* ) [ TokenTree* ] { TokenTree* }
SimplePath ! ( TokenTree* ) ;
SimplePath ! [ TokenTree* ] ;
SimplePath ! { TokenTree* }

(ShorthandSelf);
self
mut self
&self
&mut self
&'a self
&'a mut self

(TypedSelf);
self: Type
mut self: Type
Type is usually: Self, &self, &mut Self, Box<Self>, Pin<&mut Self>;

fn foo(#[cfg(...)] &self) {...};

SelfParam associated inside an impl or Trait;




ShorthandSelf -> ( & | & Lifetime ) mut self

impl Foo {
  fn a(self) {};
  fn b(mut self) {};
  fn c(&self) {};
  fn d(&mut self) {};
  fn e<'a>(&'a self) {};
  fn f<'a>(&'a mut self) {};
}
                  
                  

                  
TypedSelf -> mut self : Type

impl Foo {
    fn a(self: Self) {};
    fn b(mut self: Self) {}
    fn c(self: &Self) {}
    fn d(self: &mut Self) {}
    fn e(self: Box<Self>) {}
    fn f(self: Pin<&mut Self>) {}
    fn g(mut self: Box<Self>) {};
}


                  
                  
FunctionParamPattern -> PatternNoTopAlt : ( Type | ... )

// Ordinary parameters
fn foo(x: i32) {}
fn bar(mut y: String) {}
fn baz((a, b): (i32, i32)) {}
fn qux([x, y, z]: [u8; 3]) {}
fn quux(Foo { field }: Foo) {}
// Variadic (only allowed as the last parameter of an `extern "C"` function)
extern "C" {
    fn printf(format: *const i8, ...) -> i32;
}


                  
                  
FunctionReturnType -> Type

fn name(...) -> Type { ... }   // FunctionReturnType is the "-> Type" part
fn name(...) { ... }           // no return type (implicitly ())

fn add(a: i32, b: i32) -> i32 { a + b }

fn unit() -> () {}

fn never() -> ! { loop {} }

async fn example() -> Result<String, Error> { ... }


                  
                  
WhereClause → where ( WhereClauseItem , )* WhereClauseItem?

fn example<T, U>()
where
    T: Clone,
    U: Debug + PartialEq,
    T: 'static,
{ ... }

fn with_lifetime<'a, T>()
where
    'a: 'static,
    T: 'a,
{ ... }

// Trailing comma is allowed
fn trailing<T>()
where
    T: Clone,
{ ... }


                  
                  
WhereClauseItem →
      LifetimeWhereClauseItem
    | TypeBoundWhereClauseItem

where 'a: 'b
where 'a: 'b + 'c

where T: Clone
where T: Clone + Debug
where T: 'static
where for<'a> T: Trait<'a>
where T:                    // empty bounds are allowed (rare)


                  
                  
LifetimeWhereClauseItem → Lifetime : LifetimeBounds?

// Normal case
where 'a: 'b
where 'a: 'b + 'c

// Empty bounds (allowed by the grammar)
where 'a:


                  
                  
TypeBoundWhereClauseItem → ForLifetimes? Type : Bounds?

// Ordinary type bound
where T: Clone

// Multiple bounds
where T: Clone + Debug

// Higher-ranked trait bound
where for<'a> T: Trait<'a>

// Bound on a non-parameter type
where i32: Default
where <T as Iterator>::Item: Debug

// Empty bounds (allowed)
where T:
