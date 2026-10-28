// Są dwie rodziny:
// impl Typ (metody własne)
// impl Trait for Typ (implementacja kontraktu)


// impl własny (inherent)
// Atrybuty i modyfikatory przy funkcjach: pub, pub(crate), unsafe fn, const fn, async fn, extern "C" fn, #[inline], #[must_use], #[cfg(...)]
struct Counter { n: u32 }

impl Counter {
    const MAX: u32 = 100;                         // stała skojarzona

    fn new() -> Self { Self { n: 0 } }            // funkcja skojarzona (bez self)
    const fn zero() -> Self { Self { n: 0 } }     // const fn: działa w czasie kompilacji

    fn get(&self) -> u32 { self.n }               // odczyt
    fn inc(&mut self) { self.n += 1 }             // zapis
    fn into_inner(self) -> u32 { self.n }         // konsumuje obiekt
    fn boxed(self: Box<Self>) -> u32 { self.n }   // inne formy odbiorcy
    fn pinned(self: Pin<&mut Self>) {}            // dla async i intrusive
}

impl Counter { /* można mieć wiele bloków impl dla jednego typu */ }


// impl z generykami
// To samo co w struct: <T> po impl deklaruje parametr, a Wrapper<T> go używa. Konkretny typ (Wrapper<u32>) nie deklaruje nic
struct Wrapper<T> { inner: T }

impl<T> Wrapper<T> {                              // dla każdego T
    fn new(inner: T) -> Self { Self { inner } }
}

impl<T: Copy> Wrapper<T> {                        // tylko gdy T: Copy
    fn get(&self) -> T { self.inner }
}

impl Wrapper<u32> {                               // tylko dla konkretnego typu
    fn double(&self) -> u32 { self.inner * 2 }
}

impl<T> Wrapper<T> where T: Display + Clone { }   // ograniczenia w where

impl<'a, T> View<'a, T> { }                       // lifetime w impl
impl<T, const N: usize> Ring<T, N> {              // stała generyczna
    const CAP: usize = N;
}
impl<T> Wrapper<Vec<T>> { }                       // wzorzec w typie
impl<T> Wrapper<&T> { }                           // dla referencji
impl<T, E> Wrapper<Result<T, E>> { }


// impl Trait for Typ
impl Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.n)
    }
}


// Na jakie typy można implementować
impl Trait for Counter {}              // struktura
impl Trait for u32 {}                  // typ prymitywny (jeśli trait jest twój)
impl Trait for &Counter {}             // referencja współdzielona
impl Trait for &mut Counter {}         // referencja mutowalna
impl<T: Trait> Trait for Box<T> {}     // pudełko
impl<T: Trait> Trait for Vec<T> {}
impl<T> Trait for [T] {}               // slice (DST)
impl<T, const N: usize> Trait for [T; N] {}   // tablica
impl<A, B> Trait for (A, B) {}         // krotka
impl Trait for fn(u32) -> u32 {}       // wskaźnik na funkcję
impl Trait for () {}                   // unit
impl Trait for dyn Other {}            // obiekt trait
impl Trait for *const u8 {}            // surowy wskaźnik


// Blanket impl (dla wszystkich typów spełniających warunek)
impl<T: Display> MyTrait for T {
    fn describe(&self) -> String { format!("<{}>", self) }
}
// tak działa np. ToString: impl<T: Display> ToString for T


// Trait generyczny i jego warianty
// Ten sam trait z innym parametrem to osobne impl. To w Rust zastępuje przeciążanie
impl From<u32> for Counter { fn from(n: u32) -> Self { Self { n } } }
impl From<Counter> for u32 { fn from(c: Counter) -> u32 { c.n } }
impl PartialEq<u32> for Counter { fn eq(&self, o: &u32) -> bool { self.n == *o } }
impl Add<Counter> for Counter { type Output = Counter; fn add(self, o: Counter) -> Counter { /*..*/ } }
impl<'a> Add<&'a Counter> for &'a Counter { type Output = Counter; /*..*/ }


// Elementy wewnątrz impl Trait for
impl Iterator for Counter {
    type Item = u32;                                   // typ skojarzony
    fn next(&mut self) -> Option<u32> { /*..*/ None }  // wymagana metoda
    // metody domyślne (map, filter, ...) dostajesz za darmo
}

trait Shape { const SIDES: u32; }
impl Shape for Square { const SIDES: u32 = 4; }       // stała skojarzona

impl LendingIter for X { type Item<'a> = &'a u8 where Self: 'a; /*..*/ }  // GAT


// unsafe impl i traity markerowe
// unsafe impl oznacza: kompilator nie jest w stanie tego sprawdzić, więc dowodem jesteś ty. Dla Send i Sync to klucz przy własnych typach z surowymi wskaźnikami
unsafe impl Send for MyPtr {}      // obiecujesz, że wolno przenosić między wątkami
unsafe impl Sync for MyPtr {}      // obiecujesz, że wolno współdzielić
unsafe impl GlobalAlloc for Bump { /* alloc, dealloc */ }

unsafe trait Zeroable {}           // trait, którego implementacja jest obietnicą
unsafe impl Zeroable for u32 {}

impl Copy for Foo {}               // marker (wymaga Clone)
impl Eq for Foo {}


// impl na obiektach trait
impl dyn Shape {                       // metody własne dla dyn Shape
    fn describe(&self) -> String { /*..*/ }
}
impl dyn Shape + Send { }              // z dodatkowym ograniczeniem
impl<T: Shape + ?Sized> Shape for Box<T> { }   // ?Sized: dopuszcza DST


// Najważniejsze traity systemowe do implementowania
// Trait
// Po co
// Wzorzec
// Drop
// sprzątanie przy wyjściu z zakresu
// RAII guard, unmap, unlock
// Deref / DerefMut
// autodereferencja do wnętrza
// newtype, smart pointer, MutexGuard
// Index / IndexMut
// x[i]
// własne tablice i bufory
// Add, BitOr, Shl...
// operatory
// PhysAddr + offset, flagi uprawnień
// From / TryFrom
// konwersje
// walidowane adresy i uchwyty
// Iterator / IntoIterator
// pętle for
// przechodzenie struktur nieliniowych
// Default
// wartość domyślna	
// ..Default::default()
// Clone / Copy
// kopiowanie
// Copy dla adresów i uchwytów
// AsRef / Borrow
// tanie widoki
// &[u8] z różnych buforów
// Future
// async bez std
// executor w kernelu
// Send / Sync	
// granice wątków
// dane współdzielone między rdzeniami
// GlobalAlloc / Allocator
// alokatory
// bump, slab
// Write (fmt / io)
// wyjście
// konsola UART, logger
//
impl Drop for MapGuard {
    fn drop(&mut self) { unsafe { unmap(self.base, self.len) } }
}
impl Deref for Handle { type Target = Inner; fn deref(&self) -> &Inner { /*..*/ } }


// Słowo impl poza blokami (inne znaczenie)
// Tu impl nie definiuje niczego, tylko mówi “jakiś typ implementujący ten trait”
fn f(x: impl Display) {}              // w argumencie: anonimowy generyk
fn g() -> impl Iterator<Item = u32> {} // w zwrocie: ukryty konkretny typ
trait T { fn h(&self) -> impl Display; }  // w traicie (RPITIT)
type Alias = impl Display;             // type alias impl trait (unstable)


// Wzorce systemowe z impl
// Typestate: różne metody w zależności od stanu
struct Page<S> { addr: u64, _s: PhantomData<S> }
impl Page<Unmapped> { fn map(self, f: Flags) -> Page<Mapped> { /*..*/ } }
impl Page<Mapped>   { fn unmap(self) -> Page<Unmapped> { /*..*/ } fn write(&mut self, o: usize, v: u8) { /*..*/ } }
impl<S> Page<S>     { fn addr(&self) -> u64 { self.addr } }   // wspólne dla obu

// Sealed trait: nikt poza modułem nie zaimplementuje
mod private { pub trait Sealed {} }
pub trait Mode: private::Sealed {}
impl private::Sealed for Mapped {}
impl Mode for Mapped {}

// MMIO: metody na rejestrze
impl Volatile<u32> {
    fn read(&self) -> u32 { unsafe { ptr::read_volatile(self.0.get()) } }
    fn write(&self, v: u32) { unsafe { ptr::write_volatile(self.0.get(), v) } }
}


// Ograniczenia
// Reguła osieroconych (orphan rule): impl Trait for Typ wolno napisać, tylko jeśli trait albo typ jest z twojego crate’a. Nie zaimplementujesz Display dla Vec<u8>. Obejście: newtype
// Brak nakładających się impl (koherencja): dwa impl nie mogą pasować do tego samego typu. Specjalizacja (default fn) istnieje tylko w nightly
// Brak dziedziczenia: zamiast tego traity z domyślnymi metodami i kompozycja
// Brak przeciążeń metod o tej samej nazwie w jednym typie. Zastępują je generyki, traity i From
// Brak pól w traitach. Trait opisuje zachowanie, dane trzyma struktura
// impl przejdzie regułę osieroconych
// którą wersję impl kompilator wybierze (własny, blanket czy dla referencji)
