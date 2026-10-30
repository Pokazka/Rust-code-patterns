// Anatomia
#[atrybuty]
let [mut] WZORZEC [: TYP] [= WYRAŻENIE [else { rozbieżny blok }]];
//       │          │       │                 └ tylko dla wzorców odmowalnych
//       │          │       └ inicjalizacja (opcjonalna)
//       │          └ adnotacja typu (opcjonalna)
//       └ wzorzec, nie tylko nazwa
// let to instrukcja, nie wyrażenie: let a = (let b = 1); nie istnieje, a let a = b = 1; daje a: ()




// Podstawowe formy
let x = 5;                    // niemutowalna, typ wnioskowany (i32)
let mut y = 5;                // mutowalna
let z: u8 = 5;                // jawny typ
let w = 5u8;                  // typ w literale
let big = 1_000_000_u64;      // podkreślniki, sufiks
let h = 0xFF_u8;              // 0x, 0o, 0b
let v: Vec<_> = it.collect(); // _ = niech kompilator wywnioskuje
let n = "42".parse::<u32>().unwrap();   // turbofish zamiast adnotacji
let _ = f();                  // wynik odrzucony od razu
let _unused = 5;              // bez ostrzeżenia o nieużyciu
// Adnotacja dotyczy całego wzorca, nie pojedynczych nazw: let (a, b): (u8, u16) = ... działa, ale let (a: u8, b) = ... nie istnieje




// Inicjalizacja odroczona
let x: u32;                   // zadeklarowana, niezainicjalizowana
if cond { x = 1; } else { x = 2; }   // dokładnie jedno przypisanie na każdej ścieżce
use_it(x);                    // OK: kompilator wie, że x jest ustawione
// Zmienna nie musi być mut, jeśli przypisujesz ją raz na każdej ścieżce
// Odczyt przed pewnym przypisaniem to błąd kompilacji, nie UB. W Rust nie ma niezainicjalizowanych odczytów w bezpiecznym kodzie
// Surowy, celowo niezainicjalizowany bufor robi się przez MaybeUninit:
use core::mem::MaybeUninit;
let mut buf = MaybeUninit::<[u8; 4096]>::uninit();
unsafe { fill(buf.as_mut_ptr().cast::<u8>()); }
let buf = unsafe { buf.assume_init() };   // SAFETY: fill zapisał całość




// Przesłanianie (shadowing)
let x = 5;
let x = x * 2;                // nowa zmienna, ta sama nazwa
let x = x.to_string();        // wolno zmienić typ
{
    let x = "inner";          // tylko w tym bloku
}                             // x znowu String
// Wzorce użycia:
let data = read();                   // Vec<u8>
let data = parse(&data)?;            // Header, stary Vec nadal żyje do końca zakresu
let line = line.trim();              // zawężenie
let mut v = v; v.sort();             // dodanie mutowalności
let v = v;                           // "zamrożenie": dalej już niemutowalne
// Shadowing nie zwalnia poprzedniej wartości. Zniknie z zakresu (Drop) dopiero na końcu bloku




// Wzorce nieodmowalne (irrefutable)
// W zwykłym let wzorzec musi pasować zawsze
// Krotki
let (a, b) = (1, 2);
let (a, .., z) = t;                  // pomiń środek
let (mut a, b) = t;                  // mut per nazwa
let ((a, b), c) = ((1, 2), 3);       // zagnieżdżone
let () = unit_value;

// Struktury (pełne zasady jak wcześniej)
let Point { x, y } = p;
let Point { x: a, y: b } = p;
let Point { x, .. } = p;
let Rgb(r, g, b) = c;
let Meters(m) = d;                   // newtype: rozpakowanie
let Marker = m;

// Tablice o znanym rozmiarze
let [a, b, c] = arr;
let [first, .., last] = arr;
let [head, rest @ ..] = arr;         // rest: [T; N-1], tablica, nie slice
let [a, b] = x.to_le_bytes();        // u16 → [u8; 2]

// Referencje
let &x = &5;                         // rozpakowanie: x: i32
let &(a, b) = &(1, 2);
let (ref a, ref b) = t;              // pożyczka zamiast przeniesienia
let (ref mut a, _) = t;
let (a, b) = &t;                     // default binding modes: a, b to &T
let (a, b) = &mut t;                 // a, b to &mut T

// Z wiązaniem @ (nazwa całości i części; bez testu zakresu, bo ten byłby odmowalny)
let whole @ Point { x, .. } = p;     // wymaga Copy lub pożyczki dla pól wiązanych dwa razy

// Typy puste
let Ok(v) = r;                       // r: Result<T, Infallible>; stabilne od Rust 1.82
// Czego nie wolno w zwykłym let:
let Some(x) = opt;                   // błąd E0005: wzorzec odmowalny
let 1 = n;                           // błąd
let [a, b] = slice;                  // błąd, gdy slice: &[T]; długość nie jest w typie




// let else: wzorzec odmowalny z wyjściem
let Some(x) = opt else { return None };
let Ok(n) = s.parse::<u32>() else { return Err(Bad) };
let [a, b, c, d] = *chunk else { break };       // chunk: &[u8; 4]? lub slice → patrz niżej
let &[tag, ref rest @ ..] = buf else { return Err(Short) };   // klasyczny parser
let Point { x: 0, y } = p else { continue };
let Cmd::Move { x, .. } = cmd else { panic!("zła komenda") };
// Zasady:
// Blok else musi się rozbiegać (typ !): return, break, continue, panic!, loop {}, unreachable!(), wywołanie funkcji -> !
// Wiązania są widoczne po instrukcji, na tym samym poziomie zakresu. Tym let else różni się od if let, gdzie zmienne żyją tylko w bloku, i dlatego unika “piramidy zagnieżdżeń”
// Wyrażenie po = nie może kończyć się na } (np. gołe if ... {} else {}) ani być wyrażeniem && / || na najwyższym poziomie. Owiń w nawiasy lub wyciągnij do osobnej zmiennej
// bez let else
let x = match opt { Some(x) => x, None => return };
// z let else
let Some(x) = opt else { return };
// Wzorzec do parsowania bajtów:
let [kind, len, rest @ ..] = buf else { return Err(Short) };   // buf: &[u8]
// kind, len: &u8; rest: &[u8]




// Wyrażenia po prawej stronie
// Prawie wszystko jest wyrażeniem, więc let może inicjalizować z bloków:
let a = { let t = f(); t * 2 };               // blok: ostatnie wyrażenie bez średnika
let b = if cond { 1 } else { 2 };
let c = match s { "a" => 1, _ => 0 };
let d = loop { if done() { break 42; } };     // break z wartością
let e = 'blk: { if x { break 'blk 1; } 2 };   // etykietowany blok
let f = unsafe { *ptr };
let g = async { work().await };               // future, jeszcze nie wykonane
let h = |x: u32| x + 1;                       // domknięcie
let i = move || use_it(owned);                // przenoszące domknięcie
let j: fn(u32) -> u32 = |x| x + 1;            // domknięcie bez przechwytywania → wskaźnik na funkcję
let k = &v[1..3];                             // slice
let l = &mut *p;                              // reborrow
let m = &raw mut field;                       // surowy wskaźnik bez referencji pośredniej
let n = f()?;                                 // wczesne wyjście z Err/None
let o = [0u8; 64];                            // tablica wypełniona
let p = <u32 as Default>::default();
let q = some_struct_value.field;              // przenosi lub kopiuje pole
// Domknięcie ma typ ustalany po pierwszym użyciu: let add = |a, b| a + b; ze add(1, 2) zablokuje typ na liczby całkowite




// Własność i czas życia przy let
let b = a;                  // Copy: kopia; w przeciwnym razie przeniesienie (a zużyte)
let b = &a;                 // pożyczka
let b = &mut a;             // pożyczka wyłączna
let b = a.clone();          // jawna kopia głęboka
let b = *r;                 // przeniesienie z referencji: błąd, chyba że Copy
// Tymczasowe wartości i wydłużanie życia
let r = &String::from("x");           // OK: temporary lifetime extension, żyje do końca zakresu
let s = String::from("x").as_str();   // BŁĄD: tymczasowy String znika na końcu instrukcji
let len = make().len();               // OK: tymczasowa wartość żyje do końca instrukcji
// Reguła: referencja bezpośrednio po let x = &wyrażenie; wydłuża życie tymczasowej wartości. Wywołanie metody po drodze już nie
// _ kontra _nazwa (ważne przy RAII)
let _ = mutex.lock();           // guard zwolniony OD RAZU (nic go nie wiąże)
let _guard = mutex.lock();      // guard żyje do końca bloku
let _ = place;                  // sama zmienna NIE jest przenoszona ani zwalniana
let _ = value();                // wartość tymczasowa: Drop wykonuje się na końcu instrukcji
let _g = place;                 // przenosi place do _g
// To jedna z najczęstszych pułapek w kodzie systemowym: let _ = lock.lock(); kompiluje się bez ostrzeżenia, ale nie chroni sekcji krytycznej




// Pokrewne konstrukcje (ten sam język wzorców)
if let Some(x) = opt { } else { }
if let Some(x) = a { } else if let Some(y) = b { } else { }
while let Some(x) = it.next() { }
let ok = matches!(v, Some(1 | 2));

// let chains (edycja 2024, od Rust 1.88)
if let Some(x) = a && x > 0 && let Ok(y) = f(x) { }
while let Some(x) = stack.pop() && x != 0 { }

// Te same wzorce w parametrach i pętlach
fn f((a, b): (u32, u32), &c: &u32, Point { x, .. }: Point) {}
for (i, &x) in v.iter().enumerate() {}
let g = |(a, b): (u32, u32)| a + b;
// Przypisanie z rozbiciem (bez let, na istniejących zmiennych)
let (mut a, mut b) = (1, 2);
(a, b) = (b, a);                    // swap bez zmiennej tymczasowej
[a, b] = [b, a];
Point { x: a, y: b } = p;
(a, _) = f();
(a, ..) = t;
_ = f();                            // odrzucenie wyniku
// Lewa strona to miejsca (istniejące, mut), a nie wzorzec z let: nie można tu zmienić typu ani zadeklarować nowych nazw




// let kontra const i static
const PAGE: usize = 4096;           // typ obowiązkowy, wartość w czasie kompilacji, brak wzorców
static mut COUNTER: u32 = 0;        // globalna (unsafe przy dostępie); typ obowiązkowy
static NAMES: [&str; 2] = ["a", "b"];
const _: () = assert!(PAGE.is_power_of_two());   // jedyny przypadek, gdzie const dopuszcza `_`

const fn f() -> u32 {
    let mut i = 0;                  // let wolno w const fn
    while i < 10 { i += 1; }
    i
}
// let jest tylko lokalne (na stosie, w rejestrze albo optymalizowane do niczego). Zmienna globalna to static




// Atrybuty na let
#[allow(unused_variables)] let x = 1;
#[cfg(debug_assertions)] let t = Instant::now();
#[cfg(not(debug_assertions))] let t = ();
#[rustfmt::skip] let m = [1,0,0, 0,1,0, 0,0,1];




// Wzorce systemowe z let
// Parser nagłówka
let [kind, len, rest @ ..] = buf else { return Err(Short) };
let Header { magic: MAGIC, version, .. } = parse(buf)? else { return Err(BadMagic) };

// Bajty ↔ liczby
let [lo, hi] = 0x1234u16.to_le_bytes();
let n = u32::from_le_bytes(buf[0..4].try_into().unwrap());
let (head, tail) = buf.split_at(4);
let (hi, lo) = (x >> 16, x & 0xffff);

// Tablica stron: walidacja przez let else
let Some(entry) = table.get(idx) else { return Err(Fault::OutOfRange) };
let Entry::Present { frame, flags } = *entry else { return Err(Fault::NotPresent) };

// RAII: guard trzymany do końca bloku
let _irq = IrqGuard::disable();
let mut guard = LOCK.lock();

// Kanały i pary
let (tx, rx) = channel();
let (producer, consumer) = ring.split();      // SPSC: rozdzielenie na dwa końce

// unsafe: wprowadzenie referencji z surowego wskaźnika
let r: &Page = unsafe { &*ptr };              // SAFETY: ptr wyrównany, żywy, bez aliasingu
let v = unsafe { ptr.read_volatile() };       // odczyt rejestru MMIO

// Warunkowa inicjalizacja stałej na ścieżkach
let level;
if cfg!(debug_assertions) { level = 3; } else { level = 1; }

// Typestate: każdy let zużywa poprzedni stan
let page = Page::<Unmapped>::new(addr);
let page = page.map(Flags::RW);               // shadowing: stary stan przeniesiony i niedostępny
// Ostatni wzorzec jest wart zapamiętania: shadowing plus przeniesienie daje liniowe typy w praktyce: po let page = page.map(...) stary stan nie istnieje




// Pułapki
// Brak mut: let v = Vec::new(); v.push(1); to błąd. mut dotyczy zmiennej, nie typu, a let mut na wzorcu działa per nazwa
// Wnioskowanie: let v = Vec::new(); bez żadnego użycia nie skompiluje się (typ elementu nieznany). Liczba bez ograniczeń domyślnie jest i32 (f64 dla literałów ułamkowych)
// Częściowe przeniesienie: let Point { name, .. } = p; (gdy name: String) przenosi tylko name, p jako całość jest już nieużyteczne, ale p.id nadal czytelne
// let _ = ... kontra let _x = ...: patrz sekcja 8, to różnica między zwolnieniem natychmiast a na końcu bloku
// ref kontra &: let ref x = v; robi to samo co let x = &v;, ale let &x = &v; robi odwrotność (rozpakowuje)
// Wzorzec w let nie może być odmowalny, a let else wymaga rozbieżnego bloku. Błąd E0308 w bloku else zwykle znaczy “ten blok jeszcze wraca do kodu”
// Przesłonięta zmienna nadal żyje: guard lub bufor po let x = x.something() nie zostaje zwolniony wcześniej
// Edycja 2024: w ergonomicznych trybach wiązania nie wolno pisać jawnie ref, mut, & w środku wzorca, gdy dopasowujesz referencję. let (ref a, b) = &t; w tym przypadku jest błędem
// Szeroki zakres mut: preferuj małe zakresy i shadowing zamiast długo żyjących mut




// Porównanie z C/C++
// C/C++
// Rust
// int x = 5;
// let x: i32 = 5;
// const int x = 5;
// let x = 5; (niemutowalne domyślnie)
// int x; (niezainicjalizowane, UB przy odczycie)
// let x: i32; (kompilator pilnuje przypisania przed odczytem)
// auto x = f();
// let x = f();
// auto [a, b] = pair; (structured binding, C++17)
// let (a, b) = pair; (ogólne wzorce, w tym zagnieżdżone)
// const auto& r = f(); (przedłużenie życia)
// let r = &f();
// std::tie(a, b) = t;
// (a, b) = t;
// if (auto p = f(); p) {}
// if let Some(p) = f() {}
// auto p = f(); if (!p) return;
// let Some(p) = f() else { return };
// static int x;
// static X: i32 (nie let)
// zmienna deklarowana w bloku, bez przesłaniania typu
// shadowing z możliwą zmianą typu




// które nazwy powstają, z jakimi typami, co zostaje przeniesione lub pożyczone i kiedy wykona się Drop.
let (a, b) = &t;, let [h, rest @ ..] = arr;, let _ = lock();, let Some(x) = opt else { return };
