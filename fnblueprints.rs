Function -> FunctionQualifiers fn IDENTIFIER GenericParams ( FunctionParameters ) {
  FunctionReturnType WhereClause
  ( BlockExpression | ; )
}

const fn i<>() {};
async fn i<>() {};
extern fn i<>() {};



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
impl !Send for GenericParam
impl !Sync for GenericParam
impl Freeze for GenericParam
impl RefUnwindSafe for GenericParam
impl Unpin for GenericParam
impl UnsafeUnpin for GenericParam
impl UnwindSafe for GenericParam


ItemSafety -> safe | unsafe                                 BARDZO WAŻNE




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




SelfParam -> OuterAttribute* ( ShorthandSelf | TypedSelf )  BARDZO WAŻNE




ShorthandSelf -> ( & | & Lifetime ) mut self

                  
TypedSelf -> mut self : Type

                  
FunctionParamPattern -> PatternNoTopAlt : ( Type | ... )

                  
FunctionReturnType -> Type

                  
WhereClause → where ( WhereClauseItem , )* WhereClauseItem?

                  
WhereClauseItem →
      LifetimeWhereClauseItem
    | TypeBoundWhereClauseItem

                  
LifetimeWhereClauseItem → Lifetime : LifetimeBounds?

                  
TypeBoundWhereClauseItem → ForLifetimes? Type : Bounds?
