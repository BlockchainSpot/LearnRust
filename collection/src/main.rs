


/*
Vec<T> : Tableau ou pile (LIFO)
VecDeque<T> : file (FIFO) ou pile (LIFO)
HashMap : 
BtreeMap< K,V>: dictionnaires déja trié


lifo : Dernier entré dernier sortie
fifo : Premier entré premier sortie
*/


// vec!
// fn main() {
//     let mut v = vec![1, 2, 3, 4];
//     v.push(5);
//     v.pop();
//     v.insert(0, 5);
// }


// use std::collections::VecDeque;

//vecDeque est un tableau circulaire 

// fn main() {
//     let mut v = VecDeque::new();
//     v.extend([1, 2, 3]); // ajouter éléments
//     v.pop_front(); // retirer
//     v.push_back(4); // ajouter à la fin 
//     v.pop_front(); 
//     v.extend([5, 6, 7, 8, 9]);

//     println!("{:?}", v)
// }


// c'est un dictionnaire comme en python // <Key , Valeur>
// use std::collections::HashMap;

// fn main() {
//     let mut h = HashMap::new();
//     h.insert("thomas", 25);
//     h.insert("Harry", 50);
//     println!("{:?}", h);
//     //h["harry"];
//     h.contains_key("Sally");
//     h.get("Harry");
//     h.remove("Thomas");
//     *(h.entry("Harry").or_insert(25)) += 25;
//     *(h.entry("Karl").or_insert(25)) += 25;
// }

// use std::collections::BTreeMap;

// fn main() {
//     let word = "abracadabra";
//     let mut h: BTreeMap<char, i32> = BTreeMap::new();
//     for c in word.chars() {
//         *(h.entry(c).or_insert(0)) += 1;
//     }
//     println!("{:?}", h);
// }


// Collection LinedList
// fonctionne comme un vecdeque mais ce n'est pas linéaire 

// use std::collections::LinkedList;
// fn main() {
//     let mut l = LinkedList::new();
//     l.extend([1, 2, 3]);
//     l.pop_front();
//     l.push_back(4);
//     l.push_front(5);
//     l.push_front(6);
//     println!("{:?}", l);
// }

// hashSet est comme hashmap
// mais la valeur est également la KEY  V = K

// use std::collections::HashSet;
// fn main() {
//     let mut hs = HashSet::new();
//     hs.insert("Bob");

//     hs.extend(["Peter","Bob","Peter","Sally","Bob"]);
//     println!("{:?}", hs);

//     hs.remove("Peter");
//     println!("{:?}", hs);
// }

// BinaryHeap comme une file 

// use std::collections::BinaryHeap;
// fn main() { 
//     let mut bh = BinaryHeap::new();
//     bh.push((8, "Bob"));
//     bh.extend([(2, "Al"),(7, "Zoe"),(8, "Bob"), (4, "Al")]);
    
//     while let Some((priority, name)) = bh.pop(){
//         print!("{}:{} ", priority, name);
//     }
// }

// link list  : pile ou fil sans indexation
// hashset ensemble
// file de priorité

// function anonymes

// ###### methode normal ######
// fn main() {
//     fn add(a:i32, b:i32) -> i32 {
//         a + b
//     }
//     print!("1 + 1 = {}", add(1, 1));
// }

// ####  methode courte  ######

// fn main() {
//     let add = |a, b| a + b;
//     print!("1 + 1 = {}", add(1, 1));
// }

// Methode Classique :
// fn print(x: i32) {
//     println!("{}", x);
// }

// fn main() {
//     let v = [1, 2, 3, 4];
//     v.into_iter().for_each(print);
// }

// methode simplifier

// fn main() {
//     let v = [1, 2, 3, 4];
//     v.into_iter().for_each(|x| print!("{} ", x));
// }


// fn main() {
//     let v = [1, 2, 3, 4];


//     let a = 10;
//     let print = |x| print!("{} ", x * a);
//     v.into_iter().for_each(print);
// }

// Closure ( capable de capture une variable de sont envi)
// fn main() {
//     let v = [1, 2, 3, 4];
//     let mut a = 0;
//     let print = |x| {
//         a += 10;
//         print!("{}:{} ", a, x);

//     };
//     v.into_iter().for_each(print);
// }

// Move utilisation pour multy threading
// fn main() {
//     let v = [1, 2, 3, 4];
//     let print = {
//         let mut a = 0;
//         move |x|  {
//             a += 10;
//             print!("{}:{} ",a , x);
//         }
//     };
//     v.into_iter().for_each(print);
// }
//autre méthode par un changement de propriétaire
//  le block de déclaration dans le passage de parametre 

// fn main() {
//     let v = [1, 2, 3, 4];
//     v.into_iter().for_each({
//         let mut a = 0;
//         move |x|  {
//             a += 10;
//             print!("{}:{} ",a , x);
//         }
//     });  
// }

// retourner une functon à partir d'une autre function
// fn main() {
//     let create_counter = |i| {
//         let mut a = 0;
//         move || {a += i; a }
//     };
//     let mut count = create_counter(10);
//     print!("{} ", count());
// }


/*
a suivre...
*/
