// enum Result< T,E>
//      Ok(T),
//      Err(e);

fn main() {
    let s1 = "123";
    let s2 = "hello";

    let res1 = s1.parse::<i32>();
    let res2 = s2.parse::<i32>();

   match res2 {
       Ok(r) => println!(" Résultat : {}", r),
       Err(e) => println!(" Erreur : {}", e),
   }
}

// pas d'object exeption 
// result<T, E> 
// retour de fn potenciellement en erreur
// parttern maching ()


