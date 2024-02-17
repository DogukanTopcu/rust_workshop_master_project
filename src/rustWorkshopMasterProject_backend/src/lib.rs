use std::io;
use ic_cdk::export::candid::{candid, Nat};

fn main() {
    loop {
        println!("Lütfen bir seçenek girin:");
        println!("1. Sayının faktöriyelini hesapla");
        println!("2. Bir dizi sayının toplamını hesapla");
        println!("3. Virgülle ayrılmış sayıları küçükten büyüğe sırala");
        println!("4. Çıkış");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Okuma hatası");

        match choice.trim().parse() {
            Ok(1) => calculate_factorial(),
            Ok(2) => calculate_sum(),
            Ok(3) => sort_numbers(),
            Ok(4) => {
                println!("Programdan çıkılıyor...");
                break;
            }
            _ => println!("Geçersiz seçenek!"),
        }
    }
}

fn calculate_factorial() {
    println!("Lütfen bir sayı girin:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let number: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Geçerli bir sayı girilmedi!");
            return;
        }
    };

    let result = factorial(number);
    println!("{} sayısının faktöriyeli: {}", number, result);
}

fn calculate_sum() {
    println!("Lütfen bir dizi sayı girin (boşluklarla ayırın):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Geçerli bir sayı değil!"))
        .collect();

    let sum = calculate_even_sum(&numbers);
    println!("Girilen sayıların toplamı: {}", sum);
}

fn sort_numbers() {
    println!("Lütfen bir dizi sayı girin (virgülle ayırın):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Okuma hatası");

    let mut numbers: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.trim().parse().expect("Geçerli bir sayı değil!"))
        .collect();

    numbers.sort();
    println!("Girilen sayıların küçükten büyüğe sıralı hali: {:?}", numbers);
}

fn calculate_even_sum(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}

fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}


#[candid_method]
fn calculate_factorial(n: Nat) -> Nat {
    factorial(n)
}

#[candid_method]
fn calculate_sum(numbers: Vec<Nat>) -> Nat {
    numbers.iter().sum()
}

#[candid_method]
fn sort_numbers(mut numbers: Vec<Nat>) -> Vec<Nat> {
    numbers.sort();
    numbers
}

fn calculate_even_sum(numbers: &Vec<Nat>) -> Nat {
    numbers.iter().sum()
}

fn factorial(n: Nat) -> Nat {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}