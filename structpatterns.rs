// Trzy rodzaje definicji
// Nazwane pola
struct Point { x: i32, y: i32 }

// Krotkowa (pola po indeksach)
struct Meters(f64);
struct Rgb(u8, u8, u8);

// Jednostkowa (zero rozmiaru, sam typ jako znacznik)
struct Marker;


// Warianty definicji
// Generyki, lifetime, stałe, ograniczenia
struct Ring<'a, T: Copy, const N: usize> where T: Default {
    buf: [T; N],
    name: &'a str,
}

// Widoczność per pole
pub struct Cfg { pub a: u32, pub(crate) b: u32, c: u32 }

// PhantomData: parametr typu bez danych (typestate, jednostki, adresy)
struct Page<S> { addr: u64, _s: PhantomData<S> }

// Newtype: typ opakowujący jedno pole
#[repr(transparent)]
struct PhysAddr(u64);

// DST: ostatnie pole może mieć nieznany rozmiar
struct Packet { hdr: u32, data: [u8] }

// Pole z wartością generyczną w typie funkcji
struct Handler<F: Fn(u32)> { f: F }


// Atrybuty układu w pamięci (ważne dla systemów)
#[repr(C)]              // układ jak w C, stabilny ABI
#[repr(transparent)]    // identyczny jak jedyne pole z rozmiarem
#[repr(packed)]         // bez paddingu (uwaga: referencje do pól są UB)
#[repr(align(64))]      // wyrównanie, np. linia cache
#[repr(C, align(4096))] // kombinacje
struct S { a: u8, b: u32 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]


// Union (struktura z nakładającymi się polami)
#[repr(C)]
union Reg { raw: u32, bytes: [u8; 4] }
// odczyt pola wymaga unsafe


// Tworzenie wartości
let p = Point { x: 1, y: 2 };

let x = 1; let y = 2;
let p = Point { x, y };                 // skrót, gdy nazwa zmiennej = nazwa pola

let p2 = Point { x: 5, ..p };           // struct update: reszta z p (może przenieść pola!)
let p3 = Point { y: 9, ..Default::default() };

let m = Meters(3.5);                    // krotkowa jak wywołanie funkcji
let k = Marker;                         // jednostkowa

let r = Rgb { 0: 255, 1: 0, 2: 0 };     // krotkowa też przyjmuje składnię z nazwami

// Alias Self wewnątrz impl
impl Point { fn origin() -> Self { Self { x: 0, y: 0 } } }


// Wzorce rozbierające
// Wzorce działają w let, match, if let, while let, let else, parametrach funkcji i domknięć oraz for.
let p = Point { x: 1, y: 2 };

// Pełne rozbicie
let Point { x, y } = p;

// Zmiana nazwy
let Point { x: a, y: b } = p;

// Pominięcie reszty pól
let Point { x, .. } = p;

// Dopasowanie do stałej w polu
let Point { x: 0, y } = p;              // odmowalne (refutable), tylko w match/if let

// Zakres, alternatywa
match p {
    Point { x: 0..=10, y } => {}
    Point { x: 1 | 2, .. } => {}
    _ => {}
}

// Guard (warunek dodatkowy)
match p {
    Point { x, y } if x == y => {}
    _ => {}
}

// Wiązanie @ (nazwa + test jednocześnie)
match p {
    Point { x: lo @ 0..=9, .. } => { /* lo to wartość x */ }
    whole @ Point { y: 0, .. }  => { /* cała struktura */ }
    _ => {}
}

// Krotkowa
let Rgb(r, g, b) = c;
let Rgb(r, ..) = c;
let Rgb(r, _, _) = c;
let Meters(m) = dist;

// Jednostkowa
let Marker = k;

// Zagnieżdżone
struct Line { a: Point, b: Point }
let Line { a: Point { x: x1, .. }, b: Point { x: x2, .. } } = l;

// Struktura w enumie (wariant strukturalny)
enum Msg { Move { x: i32, y: i32 }, Quit }
match msg {
    Msg::Move { x, y: 0 } => {}
    Msg::Move { x, .. }   => {}
    Msg::Quit             => {}
}


// Referencje we wzorcach
// Ważne przy typach bez Copy: rozbicie przenosi (move) pola, więc częściowo „zużyta“ struktura nie może już być użyta w całości, chyba że rozbijasz przez referencję.
let p = Point { x: 1, y: 2 };
let r = &p;
let Point { x, y } = r;             // x i y to &i32 (default binding modes)
let &Point { x, y } = r;            // jawnie: x i y to i32 (Copy)

let Point { ref x, ref y } = p;     // pożycz pola, nie przenoś
let Point { ref mut x, .. } = pm;   // pożycz mutowalnie

let Point { x, y } = &mut pm;       // x, y to &mut i32
*x += 1;


// Miejsca użycia wzorca
// Parametry funkcji
fn dist(Point { x, y }: Point) -> i32 { x + y }
fn dist2(&Point { x, y }: &Point) -> i32 { x + y }

// Domknięcia
pts.iter().map(|&Point { x, y }| x * y);

// for
for Point { x, y } in pts { }

// let else (wzorzec odmowalny + wyjście)
let Point { x: 1, y } = p else { return };

// if let / while let
if let Point { x: 0, y } = p { }

// matches! (sprawdza, bez wiązań)
let ok = matches!(p, Point { x: 0, .. });


// Metody i self
impl Point {
    fn new(x: i32, y: i32) -> Self { Self { x, y } }    // konstruktor (konwencja)
    fn len2(&self) -> i32 { self.x * self.x + self.y * self.y }
    fn shift(&mut self, d: i32) { self.x += d; }
    fn into_tuple(self) -> (i32, i32) { let Self { x, y } = self; (x, y) }
}

// Dostęp: p.x, p.0 (krotkowa); przez referencję auto-dereferencja: r.x


// Czego nie ma
// Dziedziczenia struktur (zamiast tego kompozycja i traity).
// Przeciążania konstruktorów (zamiast tego kilka funkcji new_* lub From).
// Pól bitowych jak unsigned a : 3 w C. Robi się je przez bitflags lub własne gettery/settery na masce.
// Wartości domyślnych pól w definicji (jest Default i ..Default::default()).


// Wzorce projektowe oparte na struct (systemowe)
// Wzorzec
// Zapis
// Po co
// Newtype
// struct PhysAddr(u64)
// nie pomylisz typów adresów
// Typestate
// struct Page<S>(.., PhantomData<S>)
// niedozwolone przejścia nie kompilują się
// RAII guard
// struct Guard<'a> + impl Drop
// automatyczne zwolnienie locka lub mapowania
// Builder
// Cfg::new().a(1).b(2).build()
// brak argumentów domyślnych
// Uchwyt
// struct Handle { idx: u32, gen: u32 }
// bezpieczny „wskaźnik“
// Rejestr MMIO
// #[repr(C)] struct Regs { ctrl: Volatile<u32>, .. }
// układ jak w sprzęcie
// Intrusive node
// struct Node { link: ListLink, data: T }
// lista w obiekcie
// Marker ZST
// struct Token;
// dowód uprawnienia (capability)


// W C struct to tylko układ pamięci
// a w Rust to układ plus system własności
// Wzorzec let Point { x, .. } = p ma w C odpowiednik tylko w ręcznym p.x
// natomiast Rust dodaje do tego kontrolę, czy pole jest przeniesione, pożyczone czy skopiowane


// Pola są przenoszone, pożyczane lub ignorowane oraz jak #[repr] zmienia układ bajtów (offsety, padding)
// Domyślny
// repr(C)
// repr(C, packed)
// repr(C, align(64))
#[repr(C)]
struct S {
    a: u8,
    b: u32,
    c: u16,
    d: u8,
}
// kopia / przeniesione / &T / &mutT / nietknięte
struct Packet { id: u32, name: String, data: Vec<u8> }
let Packet { name: _n, .. } = p;
// Częściowo przeniesione: p.id nadal można czytać (Copy), ale p jako całość jest już nieużyteczna.
// p w pełni użyteczne. u32 jest Copy, a reszta nie została dotknięta.
// Częściowo przeniesione: p.id i p.data nadal użyteczne, p.name i całe p nie.
// p w pełni użyteczne. name pożycza p.name do ostatniego użycia.
// p nietknięte. Domyślny tryb wiązania zamienia każde pole w referencję współdzieloną.
// p pożyczone wyłącznie do ostatniego użycia wiązań. W tym czasie nikt inny go nie odczyta.
// Nic nie przeniesione: samo _ niczego nie wiąże, więc p zostaje całe.
// Pułapka: _n to prawdziwe wiązanie (z podkreślnikiem w nazwie) i przenosi p.name.
