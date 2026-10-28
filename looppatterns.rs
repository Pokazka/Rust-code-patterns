// loop: pętla nieskończona
loop {
    // wykonuje się, dopóki nie ma break, return, ? albo panic
}


// Typ wyrażenia loop bez break to ! (nigdy nie wraca). Dlatego pasuje do funkcji, które nie kończą pracy
fn kernel_main() -> ! {
    init();
    loop { core::hint::spin_loop(); }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! { loop {} }


// break z wartością (tylko loop to potrafi)
let mut n = 0;
let result = loop {
    n += 1;
    if n == 10 { break n * 2; }     // loop jest wyrażeniem, wartość = 20
};


// Emulacja do { } while (Rust nie ma takiej pętli)
loop {
    krok();
    if !warunek() { break; }        // ciało wykona się co najmniej raz
}


// while: pętla z warunkiem
// while zawsze zwraca (), więc break wartość w nim nie działa
while i < 10 { i += 1; }
while !flag.load(Ordering::Acquire) { core::hint::spin_loop(); }   // spinlock
while let Some(x) = stack.pop() { /* ... */ }                       // patrz niżej


// while let: pętla ze wzorcem
// Działa, dopóki wzorzec pasuje
while let Some(top) = stack.pop() { }                  // stos, DFS
while let Some(node) = cur { cur = node.next; }        // lista
while let Ok(msg) = rx.try_recv() { }                  // kolejka komunikatów
while let Some(&b) = it.peek() { it.next(); }          // parser
while let Some(x) = it.next() { }                      // ręczne for

// Z dodatkowym warunkiem (let chains, edycja 2024)
while let Some(x) = stack.pop() && x > 0 { }


// for: iteracja po iteratorze
// Nie ma pętli w stylu C for (i = 0; i < n; i++). Zastępuje ją for i in 0..n albo while
for x in 0..10 { }                    // zakres [0, 10)
for x in 0..=10 { }                   // zakres domknięty
for x in (0..10).rev() { }            // od końca
for x in (0..100).step_by(10) { }

// Trzy tryby dostępu do kolekcji
for x in &v { }                       // x: &T, v zostaje
for x in &mut v { *x += 1; }          // x: &mut T
for x in v { }                        // x: T, v przeniesione (zużyte)

// Wzorce w nagłówku (to zwykłe wzorce, jak w let)
for (i, x) in v.iter().enumerate() { }
for (a, b) in xs.iter().zip(ys.iter()) { }
for &(x, y) in &points { }
for Point { x, y } in pts { }
for (k, v) in &map { }
for _ in 0..n { }                     // licznik bez zmiennej

// Okna i porcje
for w in v.windows(3) { }             // nakładające się, &[T] długości 3
for c in v.chunks(4) { }              // rozłączne porcje
for c in v.chunks_exact_mut(8) { }


// Czym for jest naprawdę (desugaring)
// Własny typ działa w for, jeśli implementujesz Iterator albo IntoIterator (patrz impl wyżej)
let mut it = IntoIterator::into_iter(coll);
loop {
    match it.next() {
        Some(x) => { /* ciało */ }
        None => break,
    }
}


// Sterowanie: break, continue, etykiety
for x in v {
    if x == 0 { continue; }           // następna iteracja
    if x > 100 { break; }             // wyjście z pętli
}

// Etykiety: wyjście z zagnieżdżonych pętli
'outer: for i in 0..10 {
    for j in 0..10 {
        if i * j > 50 { break 'outer; }
        if j == 3 { continue 'outer; }
    }
}

// break z wartością i etykietą
let found = 'search: loop {
    for x in &v {
        if pred(x) { break 'search Some(*x); }
    }
    break None;
};

// Etykietowany blok (bez pętli): wczesne wyjście z wartością
let r = 'blk: {
    if a { break 'blk 1; }
    if b { break 'blk 2; }
    3
};


// Wzorce systemowe oparte na pętlach
// Pętla zdarzeń / dispatcher
loop {
    while let Some(ev) = queue.pop() { handle(ev); }
    wait_for_interrupt();             // hlt / wfi
}

// Spinlock z backoffem
while lock.compare_exchange_weak(false, true, Acquire, Relaxed).is_err() {
    while lock.load(Relaxed) { core::hint::spin_loop(); }   // test-and-test-and-set
}

// Pętla CAS (lock-free): wczytaj, policz, spróbuj zapisać, powtórz
let mut cur = head.load(Relaxed);
loop {
    let new = f(cur);
    match head.compare_exchange_weak(cur, new, AcqRel, Relaxed) {
        Ok(_) => break,
        Err(actual) => cur = actual,  // ktoś nas ubiegł, spróbuj z nową wartością
    }
}

// Maszyna stanów
let mut state = State::Start;
loop {
    state = match state {
        State::Start => State::Run,
        State::Run if done() => break,
        State::Run => State::Run,
    };
}

// Odpytywanie urządzenia (polling) z limitem
let mut tries = 0;
while regs.status.read() & READY == 0 {
    tries += 1;
    if tries > MAX { return Err(Timeout); }
}

// Ring buffer: opróżnianie
while let Some(x) = ring.pop() { process(x); }

// Parser / lexer
loop {
    match bytes.get(pos) {
        Some(b' ') => pos += 1,
        Some(_) => break,
        None => return Eof,
    }
}


// Pętle w kontekstach specjalnych
// const fn: loop i while wolno, for nie
const fn sum(n: u32) -> u32 {
    let mut i = 0; let mut s = 0;
    while i < n { s += i; i += 1; }
    s
}

// async: pętla z await jako punktem oddania sterowania
async fn worker(mut rx: Receiver<Job>) {
    while let Some(job) = rx.recv().await { job.run().await; }
}

// let else wewnątrz pętli
loop {
    let Some(x) = it.next() else { break };
    // ...
}

// ? w pętli
for line in lines {
    let n: u32 = line.parse()?;
}


// Alternatywy iteratorowe (zamiast ręcznej pętli)
// W kodzie krytycznym dla wydajności iteratory zwykle kompilują się do tego samego, co ręczna pętla, a często lepiej, bo znika kontrola zakresu indeksu
let s: u32 = v.iter().sum();
let m = v.iter().max();
let v2: Vec<_> = v.iter().map(|x| x * 2).filter(|x| x % 3 == 0).collect();
v.iter().for_each(|x| println!("{x}"));
let found = v.iter().find(|&&x| x == 5);
let any = v.iter().any(|&x| x > 0);
let acc = v.iter().fold(0, |a, &x| a + x);

// Generatory iteratorów
std::iter::repeat(0u8).take(16);
std::iter::from_fn(|| next_value());
std::iter::successors(Some(1u32), |&x| x.checked_mul(2));   // 1, 2, 4, 8, ...


// Pułapki
// loop bez break w funkcji z -> T: kompilator zaakceptuje, bo typ to !, ale ciało nigdy nie zwróci wartości
// Mutowanie kolekcji w trakcie for x in &v: borrow checker odrzuci (iterator trzyma pożyczkę). Rozwiązania: indeksy, drain, retain, while let Some(x) = v.pop()
// for x in v zużywa v: dalej nie możesz go użyć. Pożycz przez &v
// Przepełnienie licznika: for i in 0..=255u8 jest OK (zakres domknięty obsługuje to specjalnie), ale w ręcznym while i <= 255 na u8 pętla nigdy się nie skończy
// Aktywne czekanie: loop {} bez spin_loop lub hlt pali rdzeń. W kernelu zwykle hlt, wfi albo oddanie CPU schedulerowi
// Etykiety: nazwy zaczynają się od apostrofu ('outer), a break bez etykiety dotyczy zawsze najbliższej pętli


// Porównanie z C/C++
// C/C++
// Rust
// for (;;), while (1)
// loop
// while (cond)
// while cond
// do { } while (cond)
// loop { ...; if !cond { break } }
// for (i=0;i<n;i++)
// for i in 0..n
// for (auto& x : v)
// for x in &v / &mut v
// goto done; z zagnieżdżonych
// break 'etykieta
// brak odpowiednika
// break wartość, while let, bloki z etykietą


// Wybierasz pętlę (for x in v, &v, &mut v, while let, loop z CAS)
// Co dzieje się z własnością, pożyczką i iteratorem w każdej iteracji
