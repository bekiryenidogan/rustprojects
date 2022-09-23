

fn largest <T: PartialOrd + Copy>(list:&[T]) -> T {
   let mut largest = list[0];

   for &item in list {
      if item > largest {
         largest = item;
      }
   }
   largest
}
fn main(){
   let number_list = vec![12,231,1251,634,143,33];

   let result = largest(&number_list);
   println!("The largest number is {}", result);

   let char_list = vec!['b','e','k','i','r'];

   let result = largest(&char_list);
   println!("The largest char is {}", result);
}
