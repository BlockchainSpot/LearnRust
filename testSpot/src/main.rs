mod random_info;
use random_info::*;

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

#[allow(dead_code)] 
#[derive(Debug)]




// dont use it on prod but for test i dont want warning 
struct DougsData {
   some_int: i32,
   some_float: f64,
   some_bool: bool,
   random: RandomInfo,
}

impl SomeTrait for DougsData {
   fn is_valid(&self) -> bool {
      true
   }
}

fn print_if_is_valid(check_me: &dyn SomeTrait) {
   if check_me.is_valid() {
      println!("Yay!");
   }
}

impl Default for DougsData {
   fn default() -> Self {
      Self {
         some_bool: true,
         some_float: 10.3,
         some_int: 80,
         random: RandomInfo::new(true),
      }
   }
}

fn main() {
   let random_info_var = RandomInfo { 
      some_bool: true,
      some_int: 10,
      call_count: 0,
   };

 
   let dougs_var = DougsData {
      some_bool: true,
      some_float: 10.3,
      some_int: 80,
      random: RandomInfo::new(true),
   };

   let dougs_var = DougsData::default();

   println!("{:?}",dougs_var);

   print_if_is_valid(&random_info_var);
   print_if_is_valid(&dougs_var);

}

