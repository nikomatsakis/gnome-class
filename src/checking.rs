use ast::*;
use errors::*;

pub fn check_program(program: &Program) -> Result<()> {
    for class in get_program_classes(program) {
        check_class(class)?;
    }
    Ok(())
}

fn check_class(class: &Class) -> Result<()> {
    Ok(check_class_items(class)?)
}

fn check_class_items(class: &Class) -> Result<()> {
    Ok(check_private_struct(class)?)
}

fn check_private_struct(class: &Class) -> Result<()> {
    let (num_instance_private_items, num_private_structs, num_private_inits) =
        class
        .items
        .iter()
        .fold((0, 0, 0),
              |(p, s, i), item| {
                  match *item {
                      ClassItem::InstancePrivate(_) => (p + 1, s, i),
                      ClassItem::PrivateStruct(_)   => (p, s + 1, i),
                      ClassItem::PrivateInit(_)     => (p, s, i + 1),
//                      _                        => (s, i)
                  }
              });

    if num_instance_private_items > 1 {
        bail!(ErrorKind::InstancePrivateError(format!("found {} InstancePrivate type declarations",
                                                      num_instance_private_items)));
    }

    // FIXME: use the spans to provide exact locations of the errors
    if num_private_structs != 1 {
        bail!(ErrorKind::OnePrivateStructError(format!("found {} private structs", num_private_structs)));
    }

    // FIXME: zero private_init functions are allowed if we provide a Default initializer
    if num_private_inits != 1 {
        bail!(ErrorKind::OnePrivateInitError(format!("found {} private_init items", num_private_inits)));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::{TokenStream};
    use synom::{Synom, SynomBuffer};

    use ast;

    #[test]
    fn checks_good_class() {
        let raw = "class Foo {
                       struct FooPrivate {
                           foo: u32,
                           bar: String
                       }

                       private_init () -> FooPrivate {
                           FooPrivate {
                               foo: 42,
                               bar: \"hello\".to_string()
                           }
                       }
                   }";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();

        let program = ast::Program::parse(cursor).unwrap().1;

        assert!(check_program(&program).is_ok());
    }

    #[test]
    fn catches_several_instance_private_items() {
        let raw = "class Foo {
                       type InstancePrivate = FooPriv;
                       type InstancePrivate = BarPriv;

                       struct FooPrivate {
                           foo: u32,
                           bar: String
                       }

                       private_init () -> FooPrivate {
                           FooPrivate {
                               foo: 42,
                               bar: \"hello\".to_string()
                           }
                       }
                   }";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();

        let program = ast::Program::parse(cursor).unwrap().1;

        match check_program(&program) {
            Err(Error(ErrorKind::InstancePrivateError(_), _ )) => (),
            _ => unreachable!()
        }
    }

    #[test]
    fn catches_no_private_struct() {
        let raw = "class Foo {
                   }";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();

        let program = ast::Program::parse(cursor).unwrap().1;

        match check_program(&program) {
            Err(Error(ErrorKind::OnePrivateStructError(_), _ )) => (),
            _ => unreachable!()
        }
    }

    #[test]
    fn catches_multiple_private_structs() {
        let raw = "class Foo {
                       struct FooPrivate {
                           foo: u32,
                           bar: String
                       }

                       struct BarPrivate {
                           foo: u32,
                           bar: String
                       }
                   }";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();

        let program = ast::Program::parse(cursor).unwrap().1;

        match check_program(&program) {
            Err(Error(ErrorKind::OnePrivateStructError(_), _ )) => (),
            _ => unreachable!()
        }
    }

    #[test]
    fn catches_no_private_init() {
        let raw = "class Foo {
                       struct FooPrivate {
                           foo: u32,
                           bar: String
                       }
                   }";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();

        let program = ast::Program::parse(cursor).unwrap().1;

        match check_program(&program) {
            Err(Error(ErrorKind::OnePrivateInitError(_), _ )) => (),
            _ => unreachable!()
        }
    }

    #[test]
    fn catches_multiple_private_inits() {
        let raw = "class Foo {
                       struct FooPrivate {
                           foo: u32,
                           bar: String
                       }

                       private_init () -> FooPrivate {
                           FooPrivate {
                               foo: 42,
                               bar: \"hello\".to_string()
                           }
                       }

                       private_init () -> FooPrivate {
                           FooPrivate {
                               foo: 42,
                               bar: \"hello\".to_string()
                           }
                       }
                   }";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = SynomBuffer::new(token_stream);
        let cursor = buffer.begin();

        let program = ast::Program::parse(cursor).unwrap().1;

        match check_program(&program) {
            Err(Error(ErrorKind::OnePrivateInitError(_), _ )) => (),
            _ => unreachable!()
        }
    }
}
