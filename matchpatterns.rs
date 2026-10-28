// Anatomia
let wynik = match wyrazenie {
    wzorzec1 => wartosc1,
    wzorzec2 | wzorzec3 if warunek => { instrukcje; wartosc2 }
    nazwa @ wzorzec4 => wartosc3,
    _ => domyslna,
};


// Zasady
// Ramiona są sprawdzane od góry, wygrywa pierwsze pasujące
// match musi być wyczerpujący (exhaustive): kompilator sprawdza, że każda możliwa wartość ma ramię
// To wyrażenie: wszystkie ramiona zwracają ten sam typ albo !
// Przecinek po ramieniu z blokiem {} jest opcjonalny, a przed pierwszym wzorcem wolno dać |


// Wzorce literałów i zakresów
match n {
    0 => {}
    1 | 2 | 3 => {}            // alternatywa
    4..=9 => {}                // zakres domknięty
    10..100 => {}              // zakres z wyłączonym końcem
    100.. => {}                // półotwarty: >= 100
}
match i {                      // i: i32
    ..0 => {}                  // ujemne
    0 => {}
    1.. => {}
}
match c {
    'a'..='z' | 'A'..='Z' => {}
    '0'..='9' => {}
    '\n' | '\t' => {}
    _ => {}
}
match flag { true => {} false => {} }
match s { "GET" => {} "POST" => {} _ => {} }      // s: &str
match b { b'a'..=b'z' => {} b'\n' => {} _ => {} } // b: u8


// Dla u8 pełne pokrycie bez _ jest dozwolone, bo kompilator liczy zakresy
match b { 0..=127 => {} 128..=255 => {} }


// Symbol zastępczy i wiązania
match x {
    _ => {}        // dowolna wartość, nic nie wiąże i nic nie przenosi
}
match x {
    y => {}        // wiąże całą wartość (przenosi, jeśli nie Copy)
}
match x {
    _y => {}       // też wiąże i przenosi, tylko bez ostrzeżenia o nieużyciu
}


// Krotki: dopasowanie wielu wartości naraz
// To najczęstszy sposób na tablice decyzyjne i maszyny stanów ((stan, zdarzenie))
match (a, b) {
    (0, 0) => {}
    (0, _) | (_, 0) => {}
    (x, y) if x == y => {}
    (x, y) => {}
}


// Enumy
// Porównanie: match a.cmp(&b) { Ordering::Less => .., Ordering::Equal => .., Ordering::Greater => .. }
enum Cmd { Quit, Move { x: i32, y: i32 }, Write(String), Color(u8, u8, u8) }

match cmd {
    Cmd::Quit => {}
    Cmd::Move { x, y: 0 } => {}              // stała w polu
    Cmd::Move { x, .. } => {}
    Cmd::Write(s) if s.is_empty() => {}
    Cmd::Write(s) => {}
    Cmd::Color(r, _, _) => {}
}

match opt { Some(x) => {} None => {} }
match res { Ok(v) => {} Err(e) => {} }
match opt { Some(Some(x)) => {} Some(None) => {} None => {} }   // zagnieżdżone
match res { Ok(1 | 2) => {} Ok(_) => {} Err(_) => {} }          // alternatywa w środku

use Cmd::*;     // skrócenie ścieżek (uwaga na pułapkę w sekcji 12)


// Własność w match: przenoszenie, pożyczanie, kopiowanie
// To najważniejsze miejsce dla programisty z C/C++
// Kilka zasad
// match na wyrażeniu miejsca (zmienna, pole) samo niczego nie przenosi, przenosi dopiero wiązanie przez wartość w ramieniu
// Default binding modes: gdy dopasowujesz &Option<T> wzorcem Some(x), kompilator sam robi x: &T
// W edycji 2024 w takim ergonomicznym trybie nie wolno już pisać jawnie ref, mut ani & w środku wzorca (zarezerwowane). Zapisy z sekcji powyżej z ref działają, gdy dopasowujesz wartość, a nie referencję
// Wartość Copy (liczby, PhysAddr, uchwyt) jest po prostu kopiowana i nic się nie przenosi.
let o: Option<String> = Some("abc".into());

match o { Some(s) => {} None => {} }        // s: String, przenosi, o zużyte
match &o { Some(s) => {} None => {} }       // s: &String, o nietknięte
match &mut o { Some(s) => s.push('x'), None => {} }   // s: &mut String
match o.as_ref() { Some(s) => {} None => {} }         // jak &o, jawnie
match o { Some(ref s) => {} None => {} }    // pożycza, o nietknięte
match o { Some(_) => {} None => {} }        // nie przenosi (nic nie wiąże)
match o.as_deref() {                        // Option<String> → Option<&str>
    Some("abc") => {}
    Some(_) => {}
    None => {}
}
match s.as_str() { "a" => {} _ => {} }      // String trzeba zamienić na &str


// Wzorce tablic i slice’ów
match slice {
    [] => {}                              // pusty
    [x] => {}                             // dokładnie jeden
    [a, b] => {}                          // dokładnie dwa
    [first, .., last] => {}               // co najmniej dwa
    [first, rest @ ..] => {}              // głowa i ogon (rest: &[T])
    [.., last] => {}
    [a, b, rest @ ..] => {}
    [0, ..] => {}                         // zaczyna się od zera
    [.., 0xff] => {}
}

let arr = [1, 2, 3];
match arr { [a, b, c] => {} }             // tablica o stałym rozmiarze: pełne pokrycie

match v.as_slice() { [] => {} [x, ..] => {} }   // Vec<T> najpierw na slice


// To pozwala na rekurencję po slice’ach bez indeksów
fn sum(s: &[u32]) -> u32 { match s { [] => 0, [h, t @ ..] => h + sum(t) } }


// Wiązanie @ (nazwa i test jednocześnie)
match n {
    small @ 0..=9 => {}                   // small to wartość, a zakres to warunek
    x @ (10 | 20 | 30) => {}              // alternatywa w nawiasach
    other => {}
}
match opt { whole @ Some(_) => {} None => {} }
match opt { Some(n @ 1..=5) => {} _ => {} }
match p { whole @ Point { x, .. } => {} }   // cała struktura i jej pole (stabilne od 1.56)


// Strażnicy (if)
// Co trzeba wiedzieć
// Strażnik wykonuje się dopiero po dopasowaniu wzorca i może używać zmiennych z tego wzorca
// Kompilator nie bierze strażników pod uwagę przy sprawdzaniu wyczerpywalności. Ramiona ze strażnikami nie liczą się jako pokrycie, więc potrzebujesz ramienia bez strażnika (zwykle _)
// W strażniku wiązania są dostępne tylko jako referencje współdzielone, nie wolno w nim przenosić
// if let w strażniku (Some(x) if let Ok(y) = f(x) =>) był przez długi czas tylko w nightly. Sprawdź, czy twój kompilator już to stabilizuje
match x {
    Some(n) if n > 0 => {}
    Some(n) if n < 0 => {}
    Some(_) => {}
    None => {}
}

match v {
    1 | 2 if flag => {}                   // strażnik dotyczy OBU alternatyw
    _ => {}
}


// Stałe i ścieżki we wzorcach
// Stała we wzorcu musi mieć typ, który wspiera dopasowanie strukturalne (liczby, &str, enumy, krotki i struktury z derive(PartialEq, Eq)). Zmiennoprzecinkowych lepiej unikać
const PAGE: usize = 4096;
const NULL: u32 = 0;

match n {
    PAGE => {}                            // porównanie ze stałą
    NULL => {}
    _ => {}
}

match ev { Event::Key(Key::Esc) => {} Event::Key(_) => {} _ => {} }   // ścieżka do wariantu
match x { u32::MAX => {} _ => {} }        // stała skojarzona


// Wyczerpywalność i typy specjalne
// W twoim kodzie _ => bywa niebezpieczne: gdy dodasz wariant do enuma, kompilator nie zgłosi braku obsługi. Przy własnych enumach lepiej wypisać warianty jawnie, żeby zmiana wymusiła przegląd
// Typ pusty: match bez ramion
enum Void {}
fn absurd(v: Void) -> ! { match v {} }

// Result z niemożliwym błędem: wystarczy ramię Ok
fn f(r: Result<u32, core::convert::Infallible>) -> u32 {
    match r { Ok(v) => v }
}

// #[non_exhaustive]: enum z innego crate'a wymaga ramienia domyślnego
match err.kind() {
    ErrorKind::NotFound => {}
    _ => {}                               // wymagane, bo przyszłe warianty
}


// Sterowanie w ramionach
// Ramię może mieć typ !, więc wolno w nim zrobić wyjście
let n = match s.parse::<u32>() {
    Ok(n) => n,
    Err(_) => return Err(Bad),            // wyjście z funkcji
};
let x = match it.next() { Some(x) => x, None => break };
let y = match opt { Some(y) => y, None => continue };
let z = match r { Ok(z) => z, Err(e) => panic!("{e}") };
match state { S::Done => unreachable!(), S::Todo => todo!(), _ => {} }


// Pokrewne konstrukcje (ten sam język wzorców)
if let Some(x) = opt { } else { }                         // jeden wzorzec
if let Some(x) = a { } else if let Some(y) = b { } else { }
let Some(x) = opt else { return };                        // let else: wzorzec odmowalny + wyjście
while let Some(x) = stack.pop() { }
let ok = matches!(v, Some(1 | 2));                        // bool bez wiązań
let ok = matches!(v, Some(n) if n > 0);                   // ze strażnikiem
if let Some(x) = opt && x > 0 { }                         // let chains (edycja 2024)
let (a, b) = pair;                                        // wzorzec nieodmowalny w let


// Wzorce systemowe
// Dekoder opcodów
match opcode {
    0x00 => nop(),
    0x01..=0x0f => alu(opcode & 0x0f),
    0x10 | 0x11 => jump(),
    op if op & 0x80 != 0 => extended(op),
    _ => illegal(),
}

// Wektor przerwania
match vector {
    0..=31 => exception(vector),
    32..=47 => irq(vector - 32),
    0x80 => syscall(),
    _ => spurious(),
}

// Syscall z walidacją numeru (TryFrom robi "adres → typ")
match Syscall::try_from(nr) {
    Ok(Syscall::Read)  => sys_read(a0, a1, a2),
    Ok(Syscall::Write) => sys_write(a0, a1, a2),
    Err(_) => -ENOSYS,
}

// Page fault: krotka bitów z kodu błędu
match (err.present(), err.write(), err.user()) {
    (false, _, _)        => demand_page(addr),
    (true, true, _)      => cow_or_segv(addr),
    (true, false, true)  => segv(addr),
    (true, false, false) => kernel_bug(addr),
}

// Parser protokołu na bajtach
match buf {
    [b'G', b'E', b'T', b' ', rest @ ..] => get(rest),
    [b'P', b'O', b'S', b'T', b' ', rest @ ..] => post(rest),
    [] => Incomplete,
    _ => BadRequest,
}

// Uchwyt z generacją
match slots.get(h.idx as usize) {
    Some(Slot::Used { gen, val }) if *gen == h.gen => Some(val),
    _ => None,                            // pusty slot, zły indeks albo stara generacja
}

// Pętla CAS
match head.compare_exchange_weak(cur, new, AcqRel, Relaxed) {
    Ok(_) => break,
    Err(actual) => cur = actual,
}

// Maszyna stanów
state = match (state, event) {
    (State::Idle, Event::Start) => State::Running,
    (State::Running, Event::Done) => State::Idle,
    (s, _) => s,
};

// Błędy z filtrowaniem
match res {
    Err(e) if e.kind() == ErrorKind::Interrupted => retry(),
    Err(e) => return Err(e),
    Ok(v) => v,
}


// Pułapki
// Literówka w wariancie: gdy zapomnisz use Ordering::*, to Less => .. staje się zmienną, która łapie wszystko. Kompilator ostrzega (unreachable pattern, nazwa nie w snake_case), więc nie ignoruj ostrzeżeń w match
// Kolejność: ogólne ramię przed szczegółowym zasłania je. Jest ostrzeżenie unreachable_patterns
// Alternatywa musi wiązać te same nazwy tego samego typu: Ok(n) | Err(n) działa tylko, gdy oba n mają jeden typ
// _y kontra _: _y przenosi wartość, _ nie
// Przeniesienie w ramieniu: po match o { Some(s) => .. } zmienna o jest częściowo zużyta. Jeśli potrzebujesz jej dalej, dopasuj &o lub o.as_ref()
// Strażnik nie zapewnia pokrycia: brak końcowego _ skończy się błędem, mimo że “logicznie” wszystko pokryłeś
// Wiązania @ z podwzorcami wymagają, żeby wartość była Copy albo pożyczona, gdy podwzorzec też wiąże


// Porównanie z switch z C/C++
// C/C++ switch
// Rust match
// tylko liczby całkowite i enumy
// dowolny typ: struktury, krotki, slice, napisy, referencje
// break lub fallthrough
// brak fallthrough, ramię to jedno wyrażenie
// default opcjonalny
// wyczerpywalność wymuszona przez kompilator
// instrukcja
// wyrażenie zwracające wartość
// case 1: case 2:
// 1 | 2 =>
// brak rozbierania danych
// wzorce wiążą pola, zakresy i @
// brak strażników
// if w ramieniu


// Rust dodatkowo z każdej struktury i enuma robi to, co w C trzeba rozpisać ręcznie: switch (tag) plus rzutowanie unii
// Tutaj sprawdzenie tagu i dostęp do danych jest jedną operacją, więc nie da się odczytać pola nieaktywnego wariantu
// Które ramię wygrywa, dlaczego wcześniejsze odpadły (wzorzec, zakres czy strażnik) i co zostaje przeniesione lub pożyczone
