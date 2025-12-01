fn main(){
   let a = 6;

   match a {
      1 => println!("monday"),
      2 => println!("tuesday"),
      3 => println!("wednesday"),
      4 => println!("thursday"),
      5 => println!("friday"),
      6 => println!("saturday"),
      7 => println!("sunday"),
      _ => println!("its not a number")
   }

   let mut count = 1;

   loop{
      println!("hello world");

      if count == 3{
         break;
      }

      count+= 1;
   }
}