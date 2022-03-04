
// fn main() {
// let mut v = vec![1, 2, 3,4 ];
// for x in &mut v {
//     //*x += 1;
//     // print!("{}", x);
// }
// println!(); 
// println!("v = {:?}", v);

// }

// fn main() {
//     let v = vec![1, 2, 3,4 ];
//     let w: Vec<i32> = v.iter().map(|x| x*x).collect();
//     println!(" w = {:?}", w);
//     println!(" w = {:?}", v);

// }

// un tableau peut parcouru avec un boucle for et avec un ref en lecture ou en écriture 



// #### Type ####
/* Caractere : char
let ch = 'c'; // char
let st = "chaine"; // &str
let a = "hello".to_string(); // string
let b = String::from("World"); // String
let mut c = String::new(); // String

let s = "chaine";
dbg!(s.len()); // s.len() = 7 Ce n'est pas la bonn méthoe pour compter le nombre de Caractere d'une chaine 
dbg!(s.chars().count());
let mut s = String::new();
s.push_str("Bon"); // s = "Bon"
s.push_str("jour"); // s = "Bonjour"
s.push("!");
dbg!(s); // s = "Bonjour!"
*/
fn main() {
    let s = "Voici mon premier message";

    println!("{}", s.contains("premier"));
    println!("{}", s.replace("nom","ton"));
    println!("{}", s.starts_with("Voici"));
    println!("{}", s.ends_with("message"));
    println!("{}", s.to_uppercase());

    println!("{:?}", s.find("mon")); // Some(6)

    match s.find("mon") {
        Some(n) => println!("position = {}",n),
        None => println!("non présent"),
    }
}
// position 6

