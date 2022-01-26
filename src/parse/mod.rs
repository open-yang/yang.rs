extern crate nom;

use crate::model::module::Module;
use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    sequence::tuple,
    IResult,
    error::ParseError,
    InputTakeAtPosition,
    AsChar
};

pub fn variable<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsChar,
{
  input.split_at_position_complete(|item| -> bool {
      let c = item.as_char();
      return !c.is_alpha() && !c.is_dec_digit() && c != '-' && c != '_'
})
}

#[allow(dead_code)]
fn module(input: &str) -> IResult<&str, Module> {
    let (input, (_, _, _, name, _, _)) =
        tuple((multispace0, tag("module"), multispace0, variable, multispace0, tag("{")))(input)?;
    println!("name: {}", name);
    println!("input: {}", input);
    Ok((input, Module::default()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_get_file() -> std::io::Result<()> {
        let mut file = File::open("yang/acl.yang")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        match module(contents.as_str()) {
            Ok((i, m)) => println!("input {}, module {}", i, m),
            Err(e) =>  println!("error {}", e),
        }
        Ok(())
    }
}
