extern crate nom;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};
use crate::model::module::Module;

#[allow(dead_code)]
fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

#[allow(dead_code)]
fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

#[allow(dead_code)]
fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

#[allow(dead_code)]
fn hex_color(input: &str) -> IResult<&str, Module> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Module::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(
            hex_color("#2F14DF"),
            Ok((
                "",
                Color {
                    red: 47,
                    green: 20,
                    blue: 223,
                }
            ))
        );
    }

    #[test]
    fn pares_obj_key() {
        let obj_key = r#"module svr-obj {
  yang-version 1;
  namespace "hyperion:rules:yang:service";
  prefix "service";

  import obj-key {
      prefix obj-key;
      revision-date "2020-01-15";
  }

  organization "tencent";

  description "ip address object.";

  revision "2020-01-15" {
      description "Initial revision of ip address object model";
  }
  
  typedef num-set {
      type union {
          type uint32;
          type string;
      }
  }

  grouping port-pair {
      leaf-list source {
          type num-set;
      }
      leaf-list dest {
          type num-set;
      }
  }

  grouping control-messages {
      leaf typ {
          type uint8;
      }
      leaf code {
          type num-set;
      }
  }

  grouping svr-obj {
      uses obj-key:obj-key;
      list protocols {
          choice protocol {
              case ip {
                  leaf number {
                      type uint32;
                  }
              }
              case tcp {
                  container tcp {
                      uses port-pair;
                  }
              }
              case udp {
                  container udp {
                      uses port-pair;
                  }
              }
              case icmp {
                  container icmp {
                      uses control-messages;
                  }
                  
              }
              case icmpv6 {
                  container icmpv6 {
                      uses control-messages;
                  }
              }
          }
      }
      list children {
          uses obj-key:obj-key;
      }
  }

    grouping svr-objs {
      list svr-objs {
          uses svr-obj;
      }
  }

  container svr-obj-container {
      uses svr-objs;
  }

  rpc get {
      output {
          uses svr-objs;
      }
  }

  rpc edit {
      input {
          uses svr-objs;
      }
  }
}      
        "#;
        println!("{}", obj_key)
    }
}
