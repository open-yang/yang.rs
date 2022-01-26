extern crate nom;

use nom::IResult;
use crate::model::module::Module;

#[allow(dead_code)]
fn module(input: &str) -> IResult<&str, Module> {
   Ok((input, Module::default()))
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_hex_primary()-> std::io::Result<()> {
        let mut file = File::open("yang/acl.yang")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        println!("{}", contents);
        Ok(())
    }
}
