
//===============PRINTY=========================
/* 
//Przyklad 5.1
fn main(){
    let x = 5;
    let y = 6;
    println!("x+y={}", x+y);
}
*/

//=================KONWERSJA TYPOW=======================

/* Przyklad 9.1
fn main(){
    let x: i32 = 7;
    let y: i16 = 8;
    println!("x + y = {}", x+y as i32);
} 
*/

//===================PĘTLE===============================
/* 
// Przyklad 10.1
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
// Przyklad 10.2
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
// Przyklad 10.3
fn main() {
    for liczba in 1..6 {
        println!("Aktualna liczba: {}", liczba);
    }
}
*/

//==========================FUNKCJE==========================
/*
//Przyklad 11.1
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
    // Przyklad 12.1
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
    // Przyklad 13.1
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
