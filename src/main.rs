use std::io;
fn main() {
    let mut n: i32 = 5;
    // let mut s: String = String::new();

    println!("Hello, world!");
    println!("La valeur de n est {}", n);

    n = n + 10;
    println!("La valeur de n est {}", n);
    let n :i64 = 10;
    println!("La valeur de n est {}", n);

    const P : u32 = 100;
    println!("La valeur de P est {}", P);

    let decision:bool =true;
    if decision
    {
        println!("La valeur de decision est {}", decision);
    }
    // char test
    let lettre = 'T';
    println!("{}", lettre);
    // io::stdin().read_line(&mut s).expect("erreur de lecture");
    // println!("La valeur de s est {}", s);
    //trying the tuple
    let tup:(i64, &str, bool) = (12, "dad", true);
    println!("la valeur de tup 1 est {:?}",tup);
    //testing the array
    let arr = [1,2,3];
    println!("{}",arr[2]);
    //récupération des élements saisit par l'utilisateur
    // let mut input = String::new();

    // io::stdin().read_line(&mut input).expect("erreur de lecture");
    // println!("votre nombre est {}", input);

    //adding and combining many type of variable
    let a = 110_i32;
    let b = 10_i64;
    let c = (a as i64)/b;
    println!("{}",c);
    //converting and string to a number
    let text:String=String::new();
    // récupéront un texte entré par l'utilisateur
    // io::stdin().read_line(&mut text).expect("erreur de lecture");
    println!("votre nombre est {}",text );
    // conversion en entier de la valeur saisit
    // let nombre :i64 = text.trim().parse::<i64>().unwrap();
    
    // println!("La valeur convertie est {}",nombre );
    // opérateur de comparaison
    let comparaisons = 10.20 ==(10 as f32);
    println!("la valeur de comparaison est {}",comparaisons);
    //je suis un dev rust

    // teston la condition if
    // let mut food = String::new();
    // io::stdin().read_line(&mut food).expect("error");

    // if food.trim() == "cookie"
    // {
    //     print!("this is a cookie '{}' et food.trim '{}' ", food, food.trim());
    // }
    // else 
    // {
    //     println!("this is not a cookie");
    // }
    let result = add_number(20, 30);
    println!("la valeur esr {}",result);
    // //testing the match 
    // enum coins{
    //     Dollar,
    //     Euro,
    //     Penny,
    // }
    // match coins{
    //     coins::Penny=>println!("c'est un penny"),
    //     coins::Euro=>println!("c'est un euro"),
    //     coins::Dollar=>println!("c'est un dollar"),
    // }

    // let coins::Penny;

    //learning the loops 
    let mut compter:u32 = 0;
    loop {
        compter +=1;
        println!("hello arthur");
        if compter>=10 {
            break;
        }
    }
    //for loop 
    let a = [1,2,3,4];
    for i in a.iter()
    {
        println!("voici {}", i);
    }
    // for loop white range
    for i in 1..10
    {
        println!("this is ->{}",i);
    }
    //check 6
}
// check


//testin the fonction 

fn add_number(n :i32,i :i32)->i32
{
    n + i
}
