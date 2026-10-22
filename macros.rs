#[name]
#[name = value]
#[name(content)]
#[name(key = value, ...)]

#[test]
#[doc = "hello"]
#[allow(dead_code)]
#[link(name = "mylib", kind = "static")]
#[something()]
#[derive(Debug, Clone)]
#[allow(unused_variables)]
#[cfg(target_os = "linux")]
#[test]
#[inline(always)]
#[path = "my_module.rs"]
#[doc = "This is documentation"]
#[my_attribute]
#[my_attribute()]
#[my_attribute(value)]
#[my_attribute(key = "value")]
#[some_attr()] // ✅ Only valid if `some_attr` accepts an empty list

println!, print!, eprintln!
format!, write!, writeln!
vec!
assert!, assert_eq!, debug_assert!
panic!, todo!, unimplemented!
include_str!, include_bytes!
cfg!, env!, option_env!
stringify!, concat!

mymacro! (...);

#[derive(Debug, Clone)]

#[route(GET, "/")]

// Example of using a derive macro
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}




// Simple example: a macro that creates a vector
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3, 4];
    println!("{:?}", v); // [1, 2, 3, 4]
}




#[macro_export]

( $( $x:expr ),* )

$() is $x:expr and gives name to $x

vec![1, 2, 3]; the $x matches to 1,2,3

temp_vec.push() within $()* 

{
  let mut temp_vec = Vec::new();
  temp_vec.push(1);
  temp_vec.push(2);
  temp_vec.push(3);
  temp_vec
}

use proc_macro::TokenStream;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
  Pancakes::hello_macro();
}

$ cargo new hello_macro --lib

pub trait HelloMacro {
  fn hello_macro();
}

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
  fn hello_macro() {
    println!("Hello, Macro! My name is Pancakes!");
  }
}

fn main() {
  Pancakes::hello_macro();
}

$ cargo new hello_macro_derive --lib

[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"


THIS CODE DOES NOT COMPILE

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}


hello_macro_derive::HelloMacro
#[derive(HelloMacro)]


DeriveInput {
    // --snip--
    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}


syn::File
syn::Item
syn::Expr
syn::Type
syn::DeriveInput
fn(ParseStream) -> Result<T>


# Cargo.toml
[package]
...

[lib]
proc-macro = true

[dependencies]
syn = "3"
quote = "1"


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}


#[derive(HeapSize)]
struct Demo<'a, T: ?Sized> {
    a: Box<T>,
    b: u8,
    c: &'a str,
    d: String,
}


THIS Error. DOES NOT COMPILE
#[derive(HeapSize)]
struct Broken {
    ok: String,
    bad: std::thread::Thread,
}


match expr {
    #![cfg_attr(test, deny(non_exhaustive_omitted_patterns))]

    Expr::Array(expr) => { /*...*/ }
    Expr::Assign(expr) => { /*...*/ }
     ...
    Expr::Yield(expr) => { /*...*/ }

    _ => { /* some sane fallback */ }
}


fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}


[dependencies]
hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }


#[route(GET, "/")]
fn index() {

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}


#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
