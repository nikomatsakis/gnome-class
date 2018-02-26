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
    let num_instance_private_items =
        class
        .items
        .iter()
        .fold(0,
              |p, item| {
                  match *item {
                      ClassItem::InstancePrivate(_) => p + 1,
                  }
              });

    if num_instance_private_items > 1 {
        bail!(ErrorKind::InstancePrivateError(format!("found {} InstancePrivate type declarations",
                                                      num_instance_private_items)));
    }

    Ok(())
}

pub mod tests {
    use super::*;
    use proc_macro::TokenStream;
    use syn::synom::Synom;
    use syn::buffer::TokenBuffer;

    use ast;

    pub fn run() {
        checks_empty_class();
        checks_class_with_instance_private();
        catches_several_instance_private_items();
    }

    fn checks_empty_class() {
        let raw = "class Foo {}";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = TokenBuffer::new(token_stream);
        let cursor = buffer.begin();

        let program = ast::Program::parse(cursor).unwrap().0;

        assert!(check_program(&program).is_ok());
    }

    fn checks_class_with_instance_private() {
        let raw = "class Foo {
                       type InstancePrivate = FooPrivate;
                   }";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = TokenBuffer::new(token_stream);
        let cursor = buffer.begin();

        let program = ast::Program::parse(cursor).unwrap().0;

        assert!(check_program(&program).is_ok());
    }

    fn catches_several_instance_private_items() {
        let raw = "class Foo {
                       type InstancePrivate = FooPriv;
                       type InstancePrivate = BarPriv;
                   }";

        let token_stream = raw.parse::<TokenStream>().unwrap();

        let buffer = TokenBuffer::new(token_stream);
        let cursor = buffer.begin();

        let program = ast::Program::parse(cursor).unwrap().0;

        match check_program(&program) {
            Err(Error(ErrorKind::InstancePrivateError(_), _ )) => (),
            _ => unreachable!()
        }
    }
}
