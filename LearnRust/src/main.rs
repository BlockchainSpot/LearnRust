
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
//     println!("{}", s.replace("mon","ton"));
//     println!("{}", s.starts_with("Voici"));
//     println!("{}", s.ends_with("message"));
//     println!("{}", s.to_uppercase());

//     println!("{:?}", s.find("mon")); // Some(6)

//     match s.find("mon") {
//         Some(n) => println!("position = {}",n),
//         None => println!("non présent"),
//     }

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

