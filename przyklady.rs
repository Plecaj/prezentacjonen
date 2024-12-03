//=================KONWERSJA TYPOW=======================
/*
fn main(){
    let x: i32 = 7;
    let y: i16 = 8;
    println!("x + y = {}", x+y as i32);
} 
*/

//===============INSTRUKCJE WARUNKOWE====================
/*
fn main() {
    // Przyklad if z przypisaniem do zmiennej
    let liczba = 5;
    let wynik = if liczba > 0 {
        "dodatnia"
    } else {
        "ujemna lub zero"
    };
    println!("Liczba jest: {}", wynik);

    // Przyklad match
    let kod = 200;
    match kod {
        200 => println!("OK"),
        404 => println!("Nie znaleziono"),
        _ => println!("Inny status"),
    }
}
*/
//===================PĘTLE===============================
/* 
fn main() {
    let mut licznik = 0;
    loop {
        println!("Licznik: {}", licznik);
        licznik += 1;
        
        if licznik == 5 {
            break;
        }
    }
}
*/ 

/* 
fn main() {
    let mut x = 3;
    while x > 0 {
        println!("x = {}", x);
        x -= 1;
    }
    println!("Koniec!");
}
*/

/* 
fn main() {
    for liczba in 1..6 {
        println!("Aktualna liczba: {}", liczba);
    }
}
*/

//==========================FUNKCJE==========================
/*
fn mnoz(x: i32, y: i32) -> i32 {
    return x * y;
}

fn main(){
    println!("{}", mnoz(3, 4));
}

*/

//===========================OWNERSHIP======================
/*
fn main() {
    let tekst = String::from("hello");
    
    let nowy_tekst = tekst; 
    // println!("{}", tekst); // blad
    println!("{}", nowy_tekst); 
    
    {
        let tymczasowy = String::from("temporary");
        println!("Wewnątrz zakresu: {}", tymczasowy);
    } 
    
    // println!("{}", tymczasowy); // blad
*/

//===========================BORROWING=====================
/*
fn main() {
    let mut data = String::from("Tak");

    let ref1 = &data;
    let ref2 = &data;
    // let ref_mut = &mut data;     // Błąd

    println!("ref1: {}, ref2: {}", ref1, ref2);

    let ref3 = &data;
    // data.push_str(" New content");  // Błąd
    println!("ref3: {}", ref3);
}
*/

//============================STRING======================
/*
fn main() {
    let mut s = String::from("Ala");
    s.push_str(" ma kota!");
    println!("Po push_str: {}", s);

    s.push(' ');
    println!("Po push: {}", s);

    let s_ref: &str = &s;
    println!("String jako &str: {}", s_ref);

    let len = s.len();
    println!("Długość stringa: {}", len);

    let sub_string = &s[0..3];
    println!("Podciągnięty fragment: {}", sub_string);

    if s.contains("kota") {
        println!("String zawiera słowo 'kota'");
    }
}
*/
//======================================TUPLE============================
/*
fn main() {
    let krotka = (42, "tekst", true);
    println!("Pierwszy: {}", krotka.0);
    
    let (liczba, tekst, flaga) = krotka;
    println!("Rozpakowane: {}, {}, {}", liczba, tekst, flaga);
}
*/

//===========================================ARRAY=============================
/*
fn main() {
    let liczby: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Trzeci element: {}", liczby[2]);
    
    // Iteracja po tablicy
    for x in liczby.iter() {
        println!("Element: {}", x);
    }
}
*/

//================================================VEC================================

/*
fn main() {
    let mut wektor = Vec::new();
    wektor.push(1);
    wektor.push(2);
    
    let mut v2 = vec![1, 2, 3];
    v2.pop();
    
    println!("Długość: {}", wektor.len());
    println!("Drugi element: {}", wektor[1]);
}
*/

//==============================================STRUCTY=================================
/*
fn main() {
    struct Osoba {
        imie: String,
        wiek: u32
    }

    impl Osoba {
        fn new(imie: &str, wiek: u32) -> Osoba {
            Osoba {
                imie: String::from(imie),
                wiek
            }
        }
        
        fn przedstaw_sie(&self) {
            println!("Cześć, mam na imię {}", self.imie);
        }
    }

    let osoba = Osoba::new("Jan", 25);
    osoba.przedstaw_sie();
}
*/
