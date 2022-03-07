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
// fn main() {
//     let s = "Voici mon premier message";

//     println!("{}", s.contains("premier"));
//     println!("{}", s.replace("nom","ton"));
//     println!("{}", s.starts_with("Voici"));
//     println!("{}", s.ends_with("message"));
//     println!("{}", s.to_uppercase());

//     println!("{:?}", s.find("mon")); // Some(6)

//     match s.find("mon") {
//         Some(n) => println!("position = {}",n),
//         None => println!("non présent"),
//     }
// }
// position 6


// fn add(a: &String, b: &String) -> String { // on peut aussi remplacer String par str

//     format!(" {} {}", a, b)
// }

// fn main() {
//     let h = String::from("Hello");
//     let w = String::from("World");

//     println!(" h + w = {}", add(&h, &w));
//     print!("h = {}", h)
//  }

// fn push(a: &mut String, b:&str) {
//     a.push_str(" ");
//     a.push_str(b);
// }
// fn main(){
//     let mut h = String::from("hello");
//     let w = String::from("World");

//     push(&mut h, &w);
//     println!("h = {}", h);
// }
   
/*
Valeur : T
Object : T
Ref immuable : &T
Ref modifiable &mutT
*/

// Tuple 

// fn main() {
//     let t = ("Paris", 19);
//     println!("{} {} °C", t.0, t.1);
// }
// fn main() {
//     let (add, mul) = add_mul(3, 4);
//     println!("Add : {}", add);
//     println!("Mul : {}", mul);
// }

// fn add_mul(a: i32, b: i32) -> (i32, i32) {
//     (a + b, a * b)

// }
// let s ("Paris",); // tuple d'un element doit comporter un virgule
// mot clé nom définition
// struct Position(i32, i32);

// fn main(){
//     let p = Position(1, 1);
//     let x = p.0;
//     let y = p.1;
//     let Position(x, y ) = p;
//     print!("({}, {})", x, y);
// }

// struct Position { // mot clé  nom
//     //definition
//     x: i32, // attribut, type
//     y: i32, 
// }

// fn main() {
//     let p = Position{ x: 1,y: 1 };
//     //récupération champ par champ
//     let x = p.x;
//     let y = p.y;
//     // recupération par pattern matching
//     let Position {x: x, y: y} = p;
//     let Position { x, y } = p;
//     //affichage
//     println!("x={}, y={}",x , y); //
// }

// struct Personne {
//     nom: String,
//     prenom: String,
// }

// fn main() {
//     let p = Personne{
//         nom: "Block".to_string(),
//         prenom: "Karl".to_string(),
//     };
//     println!("{} {}", p.prenom, p.nom); // jeff BEZOS
// }
/*

tuple (val1, val2, val3)
struct tuple struct nom (type1, type2,...)
Struct Struct Nom {att1: type, ..}

// énumération 

*/

// enum Feu {
//     Rouge,
//     Orange,
//     Vert
// }

// fn main() {
//     let f = Feu::Rouge;
//     match f {
//         Feu::Vert => print!("Avancer"),
//         Feu::Rouge|Feu::Orange => println!("Arreter"),
//     };
// }

// enum Forme {
//     Point, // pas de dimension
//     Cercle (i64), // rayon
//     Ellipse (i64,i64), // rayon1, rayon2
//     Rectangle { longueur: i64, largeur:i64}
// }

// let formes =  [
//     Forme::Point;
//     Forme::Cercle (5);
//     Forme::Ellipse (4, 6);
//     Forme::Rectangle { longueur: 9, largeur: 6};
// ];

// fn main(){
//     for f in &formes {
//         match f {
//             Forme::Cercle (r) => println!("Cercle : rayon = {}", r),
//             Forme::Rectangle { longueur, largeur } =>
//                 println!("Rectangle : longueur = {}, largeur = {}",
//                     longueur, largeur),
//             _ => println!("cas non traité"),
//         };
//     }
// }

