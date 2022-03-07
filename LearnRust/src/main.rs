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
<<<<<<< HEAD
//     println!("{}", s.replace("mon","ton"));
=======
//     println!("{}", s.replace("nom","ton"));
>>>>>>> bd3b8760a4e3d4f488018a0ac71dcfe3cb29bcbb
//     println!("{}", s.starts_with("Voici"));
//     println!("{}", s.ends_with("message"));
//     println!("{}", s.to_uppercase());

//     println!("{:?}", s.find("mon")); // Some(6)

//     match s.find("mon") {
//         Some(n) => println!("position = {}",n),
//         None => println!("non présent"),
//     }
<<<<<<< HEAD

//     if let Some(n) = s.find("mon"){
//         println!("position = {}", n)
//     } else {
//         println!("non présent");
//     }
// }
// position 6

// les deux types de mémoire
// La pile => Stack
// la pile stock les variables simples du contexte & appel de fonction & les varibles du contexte de la function appelé  

// le Tas => Heap
// Stocké des objets plus complexe ! mais si la mémoire est mal géré on peut avoir des fuites de mémoire 
// rust a un compilateur qui test l'allocation de mémoire 
// respecter la proprieté et l'emprunt


// la Proriété contrôle la désallocation des données
// L'Emprunt contrôle le partage des données 
// chaque valeur en rust a une variable que l'on appel son proriétaire
// il ne peut y avoir que un proprietaire
//  lorsque le prorietaire sort de sont contexte, la valeur est détruite

// fn main() {
//     let x = String::from("hello, world");
//     // let a = x; ne compilera pas l'appel si on laisse ça comme ça 
//     let a = x.clone(); // on peut clonner avec x.clone(); si on veut garder x
//     // ou on peut l'emprunter
//     let a = &x;
//     dbg!(a);
//     dbg!(x);
// }

// #### L'emprunt #####
// & référence partageable (en lecture seul)
// &mut référence exclusive (en écriture)
// fn main() {
// let mut x = String::from("hello,");
// let a = &x;
// dbg!(a);
// {
//     let b = &mut x;
//     b.push_str("world !");
//     dbg!(b);
// }
// //dbg!(a); ne compilera pas 
// dbg!(x);
// }

// Les valeurs on un seul est unique propriétaire qui sert à identifer quand la mémoire doit être libéré 
// il peut y avoir pluisieur ref en lecteur mais lorsqu'il y a une réf en récriture, les ref en lecture comme en écriture ne sont plus utilisable
// au final les ref doivent avoir une durée de vie moins longues que les propriétaires
// Rust vérifie tout ça à la compilation

// fn add(a: i32, b:i32) -> i32 {
//      a + b
// }

// fn main() {

//     println!(" 1 + 1 = {}",add(1,1));
// }


fn add (a: String, b: String) -> String {
   format!("{} {}", a, b)
}

fn main() {
    let h = String::from("Hello");
    let w = String::from("World");

    println!("h + w = {}", add(h, w));
}
=======
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
>>>>>>> bd3b8760a4e3d4f488018a0ac71dcfe3cb29bcbb

