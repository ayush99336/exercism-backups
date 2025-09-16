pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
  let mut result = String::new();
    for charecter in input.chars().rev(){
        result.push(charecter)
    }
    return result
   }

