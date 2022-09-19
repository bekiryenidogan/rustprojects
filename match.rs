

fn main(){
    let number = 15;
   
    match number{
       1 => println!("It is one !"),
       2..=20 => println!("It is between 2 and 20 !"),
       _ => println!("It doesnt match")
    }

   
   }
   /*

    let my_vec = vec![1,2,3,4];
   for index in 0..10 {
    match my_vec.get(index){
       Some(number)=> println!("The number is : {}",number),
       None =>{}
      }
  }
 */
 /*let name =  "Bekir";
    match name {
        "Kaan" => println!("Hey Kaan, I m not looking for you");
        "Bekir" => println!("Yess, Welcome Bekir");
        _ => println!("I dont know you !");
    } 
*/