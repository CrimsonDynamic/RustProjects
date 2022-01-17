fn main() {
   let mut s= String::from("Hello world");
   //let mut s = "Hello World";
   let word = first_word(&s);
   
   println!("the first word is {}", word);
   s.clear();

   let a = [1, 2, 3, 4, 5];
   let slice = &a[1..3];

   assert_eq!(slice, &[2, 3])
}

fn first_word(s: &str) -> &str {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
         return &s[..i];
      }  
   }
   
   &s[..]
}