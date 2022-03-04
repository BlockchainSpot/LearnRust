
fn main() {
let mut v = vec![1, 2, 3];
let a = [1, 2, 3, 4];
let b = &a[1..=2];
let c = &a[1..2];
let d = &a[..=2];
let e = &a[1..];
let b = &a[1];

println!(" voici la répond {:?}!", b, c, d);
let mut w = vec![4, 5, 6];
v.append(&mut w);

    println!("Voici l'index 1{:?}", v);
}
