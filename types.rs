
type MyString = String;
fn main(){
    let x= String::from("Lorem ipsum");
    let y= MyString::from("Lorem ipsum");
    println!("{}", x==y);
}