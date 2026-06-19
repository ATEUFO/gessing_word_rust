use std::io;
fn main() {
    let mut n: i32 = 5;
    let mut s: String = String::new();
    println!("Hello, world!");
    println!("La valeur de n est {}", n);
    n = n + 10;
    println!("La valeur de n est {}", n);

    io::stdin().read_line(&mut s).expect("erreur de lecture");
    println!("La valeur de s est {}", s);
}
