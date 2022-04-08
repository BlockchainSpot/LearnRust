
#[allow(unused_variables)]
#[allow(unused_assignments)]

// Mon apprentissage Rust 
#[allow(non_snake_case)]

fn main() {

   let some_bool = true;
   let some_int = 30;
   let some_int2 = 50;

   if some_bool == true ||  some_int > 100 && some_int2 == 200 {
      println!("Hit if branch")

   } else if  some_int == 30 {
      println!(" hit Else IF branch")
   } else {
      println!(" Hit else branch")
   }

   match some_int{
      0 => println!("Hi 0 branch"),
      1..=100 => println!("Between 1 and 100 branch"),
      _ => println!("Else Branch"),
   }

   let var_from_match = match some_bool { true => 10, false => 20};

   let var_from_match2 = match some_int {
      0 => 0,
      1 => 100,
      _ => 200,
   };

}