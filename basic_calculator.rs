use ::std::io;

enum Operation {
   Add(f64, f64),
   Subtract(f64, f64),
   Multiply(f64, f64),
   Divide(f64, f64),
}
fn calculate (op: Operation) ->f64 {
   match op {
      Operation::Add(a, b) => a+b,
      Operation::Subtract( a, b) => a - b,
      Operation::Multiply(a, b) => a * b,
      Operation::Divide(a, b) => a / b,
   }
}

fn main(){

   println!("Enter the first number : ");
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read input");
   let a: f64 = input.trim().parse().expect("Failed to read First Number");

  println!("Enter the operation (+, -, *, /):");
  input.clear();
  io::stdin().read_line(&mut input).expect("Failed to read input");
  let operation:char = input.trim().chars().next().expect("Invalid Operator");
 
   println!("Enter the Second Number : ");
   input.clear();
   io::stdin().read_line(&mut input).expect("Failed to read input");
   let b:f64 = input.trim().parse().expect("Failed to read Second Number");

   let operation_match = match operation {
       '+' => Operation::Add(a, b),
       '-' => Operation::Subtract(a, b),
       '*' => Operation::Multiply(a, b),
       '/' => Operation::Divide(a,b),
       _ => {
         println!("Invalid Operation");
         return;
       }
   };
   let result = calculate(operation_match);

   println!("Result: {}", result);

  }
