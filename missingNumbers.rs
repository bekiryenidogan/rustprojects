use std::collections::{HashSet};

fn main () {
   let numbers = vec![2,3,4,6,8,9];

   let mut number_hashset = HashSet::new();

   for number in numbers {
      number_hashset.insert(number);
   }

   let hashset_length = number_hashset.len();
   println!("There are {} numbers, so we are missing {}. ", hashset_length, 10 - hashset_length);


   let mut missing_vec = vec![];

   for number in 0..10 {
     if number_hashset.get(&number).is_none()  {
          missing_vec.push(number);
     } 
   
   }

   print!("It doesnt contain : ");
   for number in missing_vec  {
       print!("{} ", number);
   }
}