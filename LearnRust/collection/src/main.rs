


/*
Vec<T> : Tableau ou pile (LIFO)
VecDeque<T> : file (FIFO) ou pile (LIFO)
HashMap : 
BtreeMap< K,V>: dictionnaires déja trié


lifo : Dernier entré dernier sortie
fifo : Premier entré premier sortie
*/


// vec!
fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    v.pop();
    v.insert(0, 5);
}


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






