

fn main(){
    let number = 15;
   //let name =  "Bekir";
    match number{
       1 => println!("It is one !"),
       2..=20 => println!("It is between 2 and 20 !"),
       _ => println!("It doesnt match")
    }

    /*match name {
        "Kaan" => println!("Hey Kaan, I m not looking for you");
        "Bekir" => println!("Yess, Welcome Bekir");
        _ => println!("I dont know you !");
    } */
   }