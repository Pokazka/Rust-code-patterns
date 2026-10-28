// W Rust mod jest przestrzenią nazw i granicą prywatności, a jednostką kompilacji jest crate, nie moduł


// Deklaracja modułu
// Moduł wchodzi do kompilacji tylko wtedy, gdy ktoś go zadeklaruje przez mod. Sam plik .rs leżący w src/ niczego nie robi
// Inline: ciało w nawiasach
mod net {
    pub fn send() {}
    fn helper() {}            // prywatne
}

// Z pliku: tylko deklaracja, ciało jest w osobnym pliku
mod net;                      // szuka net.rs albo net/mod.rs

// Zagnieżdżone
mod a {
    pub mod b {
        pub mod c { pub fn f() {} }
    }
}

// Pusty moduł
mod empty {}


// Układ plików
// Styl nowoczesny (od edycji 2018): net.rs + katalog net/ na podmoduły
// Styl starszy: net/mod.rs. Oba działają, ale w jednym module nie mieszaj obu naraz dla tej samej nazwy
// Zmiana ścieżki: #[path = "other/file.rs"] mod x;
// lib.rs
mod net;
pub mod mm;

// net.rs
pub mod tcp;
pub mod udp;


// Widoczność
// Zasada, którą warto zapamiętać: prywatne = widoczne w module, w którym zdefiniowano, i we wszystkich jego potomkach. Rodzic nie widzi prywatności dziecka, a dziecko widzi prywatność rodzica
mod m {
    fn a() {}                       // prywatne: widoczne w m i w jego potomkach
    pub fn b() {}                   // wszędzie, gdzie widoczny jest sam moduł m
    pub(crate) fn c() {}            // w całym crate
    pub(super) fn d() {}            // w module nadrzędnym (i jego potomkach)
    pub(self) fn e() {}             // to samo co brak pub
    pub(in crate::a::b) fn f() {}   // w wskazanym module przodku
}
mod parent {
    fn secret() {}
    mod child {
        fn f() { super::secret(); }     // OK: dziecko widzi rodzica
    }
    fn g() { child::f(); }              // błąd: f prywatne w child
}


// Widoczność poszczególnych elementów
// To, że struktura jest pub, nie czyni jej pól publicznymi. Wzorzec: prywatne pole zmusza do używania konstruktora, który pilnuje niezmienników
pub struct S { pub a: u32, pub(crate) b: u32, c: u32 }   // widoczność per pole
pub struct Tok(u32);                    // pole krotkowe prywatne: spoza modułu nie skonstruujesz
pub struct Tok2(pub u32);               // publiczne pole

pub enum E { A, B(u32) }                // warianty i ich pola dziedziczą pub, nie da się inaczej
pub trait T { fn m(&self); }            // elementy traitu dziedziczą widoczność traitu
impl T for S { fn m(&self) {} }         // w impl Trait nie piszesz pub

pub const X: u32 = 1;
pub static Y: u32 = 2;
pub type Alias = u32;
pub mod inner {}
pub use inner::*;


// use: wprowadzanie nazw do zakresu
use std::collections::HashMap;                    // pojedynczy element
use std::{fmt, io};                               // grupa
use std::io::{self, Read, Write};                 // self = sam moduł io
use std::collections::{HashMap as Map, HashSet};  // zmiana nazwy
use std::fmt::Write as _;                         // import traitu bez nazwy (metody dostępne)
use std::io::*;                                   // glob (ostrożnie)
use std::{cmp::{max, min}, mem::{swap, replace}}; // zagnieżdżone grupy
// Początek ścieżki
use crate::mm::paging;      // od korzenia własnego crate'a
use self::local::f;         // od bieżącego modułu
use super::helper;          // od rodzica
use super::super::top;      // dziadek
use ::std::mem;             // jawnie z zewnętrznego crate'a (przydatne przy konflikcie nazw)
use std::mem;               // crate zewnętrzny w "extern prelude"
// Re-eksport
// use jest prywatny domyślnie: use a::b; w module nie powoduje, że b jest dostępne spoza niego. Dopiero pub use tworzy publiczny alias
pub use self::tcp::Socket;          // wystawia Socket jako net::Socket
pub(crate) use inner::Helper;       // re-eksport tylko w crate
pub use crate::mm::paging as pg;    // z aliasem
pub use inner::*;                   // wszystko z inner
// use w dowolnym zakresie
fn f() {
    use std::mem::swap;               // tylko w tej funkcji
    swap(&mut a, &mut b);
}
fn g() { use Color::*; match c { Red => {} Green => {} } }   // enum tylko lokalnie


// Co use importuje
// use bierze nazwę ze wszystkich przestrzeni nazw naraz (typy, wartości, makra). Jeśli moduł ma funkcję foo i typ foo, use m::foo; wciągnie oba
// Makra
#[macro_export] macro_rules! my_macro { () => {} }   // ląduje w korzeniu crate'a
use crate::my_macro;                                  // import jak zwykły element
#[macro_use] mod macros;                              // starszy styl: makra widoczne po tej deklaracji


// Atrybuty modułów
#[cfg(test)]
mod tests {
    use super::*;                    // wciąga wszystko z rodzica, w tym prywatne
    #[test] fn works() {}
}

#[cfg(target_arch = "x86_64")] mod x86_64;
#[cfg(target_arch = "aarch64")] mod aarch64;

#[cfg(target_os = "linux")]
#[path = "sys/linux.rs"]
mod sys;

#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64.rs")]
#[cfg_attr(target_arch = "aarch64", path = "arch/aarch64.rs")]
mod arch;                            // ta sama nazwa, inny plik w zależności od celu

#[cfg(feature = "alloc")] pub mod heap;

#[doc(hidden)] pub mod internal;     // publiczny technicznie, ale ukryty w dokumentacji
#[deprecated] pub mod old {}
#[allow(dead_code)] mod wip {}

mod noisy {
    #![allow(unused)]                // atrybut wewnętrzny: dotyczy całego modułu
}


// Prelude i extern crate
// W edycji 2018+ extern crate jest zwykle zbędne dla zwykłych zależności, ale alloc i std w no_std nadal go wymagają. Prelude języka (Option, Vec, Clone...) jest wstrzykiwane automatycznie, w no_std jest to core::prelude
// Własne prelude: jedno miejsce z najczęstszymi importami
pub mod prelude {
    pub use crate::mm::{PhysAddr, VirtAddr};
    pub use crate::sync::Mutex;
}
// użycie: use mycrate::prelude::*;

#![no_std]
extern crate alloc;                  // w no_std: dostęp do alloc::vec::Vec, alloc::boxed::Box
use alloc::vec::Vec;

extern crate std;                    // jawnie, gdy potrzebne w no_std + std


// Wzorce systemowe oparte na mod
// Wzorce 1, 2 i 3 działają dzięki prywatności modułów: niebezpieczne albo niezmiennikowe pola są niewidoczne spoza modułu, więc tylko kod w nim może je złamać. Powierzchnia do przeglądu unsafe kurczy się do jednego pliku
// 1. Prywatny rdzeń z unsafe + bezpieczna fasada
pub mod ring {
    use core::cell::UnsafeCell;
    pub struct Ring<T, const N: usize> { /* pola prywatne */ }
    impl<T, const N: usize> Ring<T, N> {
        pub fn push(&self, x: T) -> Result<(), T> { /* unsafe w środku */ }
    }
}

// 2. Newtype z konstruktorem pilnującym niezmiennika
pub mod addr {
    #[derive(Clone, Copy)]
    pub struct PhysAddr(u64);               // pole prywatne
    impl PhysAddr {
        pub fn new(a: u64) -> Option<Self> { (a < (1 << 52)).then_some(Self(a)) }
        pub const fn as_u64(self) -> u64 { self.0 }
    }
}

// 3. Sealed trait
mod private { pub trait Sealed {} }
pub trait Mode: private::Sealed {}          // private niedostępny spoza crate'a
impl private::Sealed for Mapped {}
impl Mode for Mapped {}

// 4. Warstwy architektury
pub mod arch {
    #[cfg(target_arch = "x86_64")] mod x86_64;
    #[cfg(target_arch = "x86_64")] pub use self::x86_64::*;
    #[cfg(target_arch = "aarch64")] mod aarch64;
    #[cfg(target_arch = "aarch64")] pub use self::aarch64::*;
}
// reszta kernela woła arch::enable_interrupts(), nie wiedząc, która architektura

// 5. Uchwyty wewnętrzne tylko dla crate'a
pub(crate) mod raw { pub(crate) unsafe fn map(/*..*/) {} }

// 6. Fasada: płaski publiczny interfejs nad głębokim drzewem
mod drivers { pub mod uart; pub mod pci; pub mod nvme; }
pub use drivers::{uart::Uart, pci::PciBus};

// 7. Moduł jako przestrzeń nazw stałych
pub mod flags {
    pub const READ: u8 = 1;
    pub const WRITE: u8 = 2;
    pub const EXEC: u8 = 4;
}
// użycie: flags::READ | flags::WRITE


// Granice wyższego poziomu
// Poziom	
// Czym jest	
// Widoczność
// funkcja / blok	
// zakres zmiennych	
// lokalny
// mod	
// przestrzeń nazw + prywatność	
// pub, pub(crate), pub(super), pub(in ..)
// crate	
// jednostka kompilacji, ABI, wersjonowanie	
// pub = widoczne dla zależności
// workspace	
// wiele crate’ów w jednym repo	
// Cargo.toml, pub między crate’ami


// pub(crate) ma sens dopiero w bibliotece, bo dla jednego crate’a binarnego różnica między pub a pub(crate) jest tylko dokumentacyjna


// Pułapki
// mod foo; nie jest importem: deklaruje moduł (wprowadza plik do kompilacji). Żeby użyć nazwy bez ścieżki, potrzebny jest jeszcze use.
// Zapomniane mod: plik istnieje, ale kod nie jest kompilowany ani sprawdzany, bez żadnego błędu.
// Dwa pliki dla jednego modułu: net.rs i net/mod.rs naraz dają błąd.
// pub na elemencie w prywatnym module: element jest pub, ale nieosiągalny spoza (ostrzeżenie unreachable_pub przy włączeniu lintów). Dopiero pub use go ujawni.
// Prywatny typ w publicznym interfejsie: błąd lub ostrzeżenie private_interfaces.
// Cykle: moduły mogą się wzajemnie importować w obrębie crate’a (to tylko przestrzenie nazw), cykli nie ma tylko między crate’ami.
// Glob use x::*: wciąga wszystko, w tym przyszłe dodatki. Jawny import zawsze wygrywa z globem, ale dwa globy o tej samej nazwie dają błąd dopiero przy użyciu.
// use super::* w testach wciąga też prywatne elementy rodzica, co jest zamierzone, ale pamiętaj, że testy tylko wewnątrz crate’a widzą prywatne API.
// pub(in path) musi wskazywać przodka, nie dowolny moduł.
// Niezmienniki unsafe: moduł jest granicą, ale pub(crate) pozwala na złamanie niezmienników z całego crate’a. Dla naprawdę wrażliwych pól trzymaj typ i jego impl w jednym małym module.


// Porównanie z C/C++
// C/C++	
// Rust
// plik .c = jednostka kompilacji	
// crate (moduły to tylko drzewo nazw wewnątrz)
// #include "x.h" (tekstowe wklejenie)	
// mod x; + use (strukturalne, bez nagłówków)
// namespace foo { }	
// mod foo { }
// static na poziomie pliku (linkage wewnętrzny)	
// brak pub
// friend, protected, private	
// pub(super), pub(crate), pub(in ..)
// using namespace x;	
// use x::*;
// using x::y;	
// use x::y;
// include guards, forward declarations	
// niepotrzebne (kolejność deklaracji nie ma znaczenia)
// extern "C"	
// extern "C" { } w Rust to blok FFI, nie moduł
// C++20 modules	
// najbliższy odpowiednik koncepcyjny


// Największa różnica: w C/C++ prywatność jest na poziomie klasy lub pliku, a w Rust na poziomie modułu, więc kilka typów w jednym module widzi nawzajem swoje prywatne pola
// To celowe i właśnie na tym stoi wzorzec “mały moduł z unsafe i bezpieczną fasadą”
// Których modułów jest widoczny przy pub, pub(crate), pub(super) i pub(in ...), oraz co się stanie po dodaniu pub use
