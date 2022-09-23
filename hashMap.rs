use std::collections::HashMap;

fn main() {
   let mut marks = HashMap::new();

   // Add values
   marks.insert("Rust Programming", 23);
   marks.insert("Java Programming", 85);
   marks.insert("Solidity Programming", 75);
   marks.insert("Javascript Programming", 55);
   marks.insert("SQL Programming", 80);


   // Length Of HashMap
   println!("How many subjects have you studied ? {}", marks.len());

   // Get Single Value
   match marks.get("Java Programming") {
       Some(mark) => println!("You got %{} for Java Programming!",mark),
       None => println!(" You didnt study ")
   }

   // Remove a Value
   marks.remove("SQL Programming");

   //Loop through HashMap
   for (subject,mark) in &marks {
       println!("For {} you got %{} !", subject, mark);
   }

   // Check For Value
   println!("Did you study C++ ? {} ", marks.contains_key("C++ Programming"));

}