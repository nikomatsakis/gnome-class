use ast::*;
use lalrpop_intern::intern;
use parser;
use quote::Tokens;
use tok::Tok;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Fields {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::*;
    use lalrpop_intern::intern;
    use parser;
    use quote::Tokens;
    use tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_26_22(Tok<'input>),
        Term_22_28_22(Tok<'input>),
        Term_22_29_22(Tok<'input>),
        Term_22_2b_22(Tok<'input>),
        Term_22_2c_22(Tok<'input>),
        Term_22_2d_3e_22(Tok<'input>),
        Term_22_3a_22(Tok<'input>),
        Term_22_3c_22(Tok<'input>),
        Term_22_3e_22(Tok<'input>),
        Term_22_5b_22(Tok<'input>),
        Term_22_5d_22(Tok<'input>),
        Term_22class_22(Tok<'input>),
        Term_22extends_22(Tok<'input>),
        Term_22fn_22(Tok<'input>),
        Term_22init_22(Tok<'input>),
        Term_22self_22(Tok<'input>),
        Term_22struct_22(Tok<'input>),
        Term_22_7b_2e_2e_7d_22(&'input str),
        TermOtherId(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, Tok<'input>, ::errors::Error>),
        Nt_28_22_2b_22_20_3cType1_3e_29(Type),
        Nt_28_22_2b_22_20_3cType1_3e_29_2b(::std::vec::Vec<Type>),
        Nt_28_22_2d_3e_22_20_3cType_3e_29(Type),
        Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(::std::option::Option<Type>),
        Nt_28_22extends_22_20_3cId_3e_29(Identifier),
        Nt_28_22extends_22_20_3cId_3e_29_3f(::std::option::Option<Identifier>),
        Nt_28_3cField_3e_20_22_2c_22_29(Field),
        Nt_28_3cField_3e_20_22_2c_22_29_2a(::std::vec::Vec<Field>),
        Nt_28_3cField_3e_20_22_2c_22_29_2b(::std::vec::Vec<Field>),
        Nt_40L(usize),
        NtClass(Class),
        NtClass_2a(::std::vec::Vec<Class>),
        NtClass_2b(::std::vec::Vec<Class>),
        NtCodeBlock(OpaqueTokens),
        NtComma_3cField_3e(Vec<Field>),
        NtField(Field),
        NtField_2a(::std::vec::Vec<Field>),
        NtField_2b(::std::vec::Vec<Field>),
        NtField_3f(::std::option::Option<Field>),
        NtFields(Vec<Field>),
        NtFnDef(FnDef),
        NtFnSig(FnSig),
        NtId(Identifier),
        NtIdStr(&'input str),
        NtInit(OpaqueTokens),
        NtMember(Member),
        NtMember_2a(::std::vec::Vec<Member>),
        NtMember_2b(::std::vec::Vec<Member>),
        NtMembers(Vec<Member>),
        NtMethod(Method),
        NtPrivateStruct(PrivateStruct),
        NtProgram(Program),
        NtReturnTy(Option<Type>),
        NtType(Type),
        NtType_2a(::std::vec::Vec<Type>),
        NtType_2b(::std::vec::Vec<Type>),
        NtType1(Type),
        Nt____Fields(Vec<Field>),
        Nt____Members(Vec<Member>),
        Nt____Program(Program),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 0, 9, 0, 0, 0, 10, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, -30, 0, -30, 0, 0, 0, -30, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 0, 9, 0, 0, 0, 10, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, -39, 0, 0, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, 0, 0, -39, 0,
        // State 6
        0, 0, 0, -41, 0, 0, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, 0, -41, 0,
        // State 7
        0, 0, 0, -42, 0, 0, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, 0, -42, 0,
        // State 8
        0, 0, 0, -40, 0, 0, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, 0, -40, 0,
        // State 9
        0, 0, 0, -43, 0, 0, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, 0, -43, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, -31, 0, -31, 0, 0, 0, -31, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 7, 8, 0, 9, 0, 0, 0, 10, 0,
        // State 12
        0, 0, 0, -66, 0, 0, 0, 17, -66, -66, -66, -66, -66, 0, -66, 0, 0, 0, -66, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, -27, 0, -27, 0, 0, 0, -27, 0,
        // State 14
        0, 0, 0, 19, 0, 0, 0, 0, -60, -60, -60, -60, -60, 0, -60, 0, 0, 0, -60, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 7, 8, 0, 9, 0, 0, 0, 10, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 23, 16, 0, 7, 8, 0, 9, 0, 0, 0, 10, 0,
        // State 17
        0, 0, 0, 24, 0, 0, 0, 0, -61, -61, -61, -61, -61, 0, -61, 0, 0, 0, -61, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 7, 8, 0, 9, 0, 0, 0, 10, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, -64, -64, 0, -64, -64, 0, -64, 0, 0, 0, -64, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 28, 16, 0, 7, 8, 0, 9, 0, 0, 0, 10, 0,
        // State 22
        0, 0, 0, -67, 0, 0, 0, 0, -67, -67, -67, -67, -67, 0, -67, 0, 0, 0, -67, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 7, 8, 0, 9, 0, 0, 0, 10, 0,
        // State 24
        0, 0, 0, -2, 0, 0, 0, 0, -2, -2, -2, -2, -2, 0, -2, 0, 0, 0, -2, 0,
        // State 25
        0, 0, 0, -69, 0, 0, 0, 0, -69, -69, -69, -69, -69, 0, -69, 0, 0, 0, -69, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, -65, -65, 0, -65, -65, 0, -65, 0, 0, 0, -65, 0,
        // State 27
        0, 0, 0, -68, 0, 0, 0, 0, -68, -68, -68, -68, -68, 0, -68, 0, 0, 0, -68, 0,
        // State 28
        0, 0, 0, -3, 0, 0, 0, 0, -3, -3, -3, -3, -3, 0, -3, 0, 0, 0, -3, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -34,
        -30,
        -35,
        -70,
        0,
        -39,
        -41,
        -42,
        -40,
        -43,
        -31,
        0,
        -66,
        -27,
        -60,
        0,
        0,
        -61,
        0,
        0,
        0,
        0,
        -67,
        0,
        -2,
        -69,
        0,
        -68,
        -3,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 4, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 15, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 22, 15, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 15, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""&""###,
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""->""###,
            r###"":""###,
            r###""<""###,
            r###"">""###,
            r###""[""###,
            r###""]""###,
            r###""class""###,
            r###""extends""###,
            r###""fn""###,
            r###""init""###,
            r###""self""###,
            r###""struct""###,
            r###""{..}""###,
            r###"OtherId"###,
        ];
        __ACTION[(__state * 20)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Fields<
        'input,
        __TOKEN: __ToTriple<'input, Error=::errors::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Vec<Field>, __lalrpop_util::ParseError<usize, Tok<'input>, ::errors::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Tok::Ampersand if true => 0,
                Tok::LeftParen if true => 1,
                Tok::RightParen if true => 2,
                Tok::Plus if true => 3,
                Tok::Comma if true => 4,
                Tok::ThinArrow if true => 5,
                Tok::Colon if true => 6,
                Tok::LessThan if true => 7,
                Tok::GreaterThan if true => 8,
                Tok::LeftBracket if true => 9,
                Tok::RightBracket if true => 10,
                Tok::Class if true => 11,
                Tok::Extends if true => 12,
                Tok::Fn if true => 13,
                Tok::Init if true => 14,
                Tok::SelfKw if true => 15,
                Tok::Struct if true => 16,
                Tok::Block(_) if true => 17,
                Tok::Id(_) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 20 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Tok::Ampersand => __Symbol::Term_22_26_22(__tok),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Tok::LeftParen => __Symbol::Term_22_28_22(__tok),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Tok::RightParen => __Symbol::Term_22_29_22(__tok),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Tok::Plus => __Symbol::Term_22_2b_22(__tok),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Tok::Comma => __Symbol::Term_22_2c_22(__tok),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Tok::ThinArrow => __Symbol::Term_22_2d_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Tok::Colon => __Symbol::Term_22_3a_22(__tok),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Tok::LessThan => __Symbol::Term_22_3c_22(__tok),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Tok::GreaterThan => __Symbol::Term_22_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Tok::LeftBracket => __Symbol::Term_22_5b_22(__tok),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            __tok @ Tok::RightBracket => __Symbol::Term_22_5d_22(__tok),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Tok::Class => __Symbol::Term_22class_22(__tok),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            __tok @ Tok::Extends => __Symbol::Term_22extends_22(__tok),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Tok::Fn => __Symbol::Term_22fn_22(__tok),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            __tok @ Tok::Init => __Symbol::Term_22init_22(__tok),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            __tok @ Tok::SelfKw => __Symbol::Term_22self_22(__tok),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            __tok @ Tok::Struct => __Symbol::Term_22struct_22(__tok),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Tok::Block(__tok0) => __Symbol::Term_22_7b_2e_2e_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Tok::Id(__tok0) => __Symbol::TermOtherId(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Field>,__lalrpop_util::ParseError<usize, Tok<'input>, ::errors::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ("+" <Type1>) = "+", Type1 => ActionFn(33);
                let __sym1 = __pop_NtType1(__symbols);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29(__nt), __end));
                0
            }
            2 => {
                // ("+" <Type1>)+ = "+", Type1 => ActionFn(63);
                let __sym1 = __pop_NtType1(__symbols);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action63::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__nt), __end));
                1
            }
            3 => {
                // ("+" <Type1>)+ = ("+" <Type1>)+, "+", Type1 => ActionFn(64);
                let __sym2 = __pop_NtType1(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__nt), __end));
                1
            }
            4 => {
                // ("->" <Type>) = "->", Type => ActionFn(38);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29(__nt), __end));
                2
            }
            5 => {
                // ("->" <Type>)? = "->", Type => ActionFn(65);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action65::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(__nt), __end));
                3
            }
            6 => {
                // ("->" <Type>)? =  => ActionFn(37);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action37::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(__nt), __end));
                3
            }
            7 => {
                // ("extends" <Id>) = "extends", Id => ActionFn(45);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29(__nt), __end));
                4
            }
            8 => {
                // ("extends" <Id>)? = "extends", Id => ActionFn(68);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action68::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__nt), __end));
                5
            }
            9 => {
                // ("extends" <Id>)? =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__nt), __end));
                5
            }
            10 => {
                // (<Field> ",") = Field, "," => ActionFn(56);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action56::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29(__nt), __end));
                6
            }
            11 => {
                // (<Field> ",")* =  => ActionFn(54);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action54::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            12 => {
                // (<Field> ",")* = (<Field> ",")+ => ActionFn(55);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action55::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            13 => {
                // (<Field> ",")+ = Field, "," => ActionFn(71);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action71::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            14 => {
                // (<Field> ",")+ = (<Field> ",")+, Field, "," => ActionFn(72);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action72::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            15 => {
                // @L =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40L(__nt), __end));
                9
            }
            16 => {
                // Class = "class", Id, "extends", Id, "{..}" => ActionFn(75);
                let __sym4 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __sym3 = __pop_NtId(__symbols);
                let __sym2 = __pop_Term_22extends_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = match super::__action75::<>(__sym0, __sym1, __sym2, __sym3, __sym4) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtClass(__nt), __end));
                10
            }
            17 => {
                // Class = "class", Id, "{..}" => ActionFn(76);
                let __sym2 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action76::<>(__sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtClass(__nt), __end));
                10
            }
            18 => {
                // Class* =  => ActionFn(46);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action46::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtClass_2a(__nt), __end));
                11
            }
            19 => {
                // Class* = Class+ => ActionFn(47);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtClass_2a(__nt), __end));
                11
            }
            20 => {
                // Class+ = Class => ActionFn(48);
                let __sym0 = __pop_NtClass(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action48::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtClass_2b(__nt), __end));
                12
            }
            21 => {
                // Class+ = Class+, Class => ActionFn(49);
                let __sym1 = __pop_NtClass(__symbols);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action49::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtClass_2b(__nt), __end));
                12
            }
            22 => {
                // CodeBlock = "{..}" => ActionFn(28);
                let __sym0 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCodeBlock(__nt), __end));
                13
            }
            23 => {
                // Comma<Field> = Field => ActionFn(82);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action82::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            24 => {
                // Comma<Field> =  => ActionFn(83);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action83::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            25 => {
                // Comma<Field> = (<Field> ",")+, Field => ActionFn(84);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action84::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            26 => {
                // Comma<Field> = (<Field> ",")+ => ActionFn(85);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action85::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            27 => {
                // Field = Id, ":", Type => ActionFn(22);
                let __sym2 = __pop_NtType(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtField(__nt), __end));
                15
            }
            28 => {
                // Field* =  => ActionFn(34);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action34::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtField_2a(__nt), __end));
                16
            }
            29 => {
                // Field* = Field+ => ActionFn(35);
                let __sym0 = __pop_NtField_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtField_2a(__nt), __end));
                16
            }
            30 => {
                // Field+ = Field => ActionFn(57);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtField_2b(__nt), __end));
                17
            }
            31 => {
                // Field+ = Field+, Field => ActionFn(58);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_NtField_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtField_2b(__nt), __end));
                17
            }
            32 => {
                // Field? = Field => ActionFn(52);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action52::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtField_3f(__nt), __end));
                18
            }
            33 => {
                // Field? =  => ActionFn(53);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action53::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtField_3f(__nt), __end));
                18
            }
            34 => {
                // Fields =  => ActionFn(80);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action80::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtFields(__nt), __end));
                19
            }
            35 => {
                // Fields = Field+ => ActionFn(81);
                let __sym0 = __pop_NtField_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action81::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFields(__nt), __end));
                19
            }
            36 => {
                // FnDef = FnSig, CodeBlock => ActionFn(17);
                let __sym1 = __pop_NtCodeBlock(__symbols);
                let __sym0 = __pop_NtFnSig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtFnDef(__nt), __end));
                20
            }
            37 => {
                // FnSig = "(", "&", "self", ")", ReturnTy => ActionFn(18);
                let __sym4 = __pop_NtReturnTy(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Term_22self_22(__symbols);
                let __sym1 = __pop_Term_22_26_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtFnSig(__nt), __end));
                21
            }
            38 => {
                // FnSig = "(", "&", "self", ",", Comma<Field>, ")", ReturnTy => ActionFn(19);
                let __sym6 = __pop_NtReturnTy(__symbols);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtComma_3cField_3e(__symbols);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_Term_22self_22(__symbols);
                let __sym1 = __pop_Term_22_26_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtFnSig(__nt), __end));
                21
            }
            39 => {
                // Id = IdStr => ActionFn(6);
                let __sym0 = __pop_NtIdStr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                22
            }
            40 => {
                // IdStr = "init" => ActionFn(7);
                let __sym0 = __pop_Term_22init_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            41 => {
                // IdStr = "class" => ActionFn(8);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            42 => {
                // IdStr = "extends" => ActionFn(9);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            43 => {
                // IdStr = OtherId => ActionFn(10);
                let __sym0 = __pop_TermOtherId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            44 => {
                // Init = "init", CodeBlock => ActionFn(15);
                let __sym1 = __pop_NtCodeBlock(__symbols);
                let __sym0 = __pop_Term_22init_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtInit(__nt), __end));
                24
            }
            45 => {
                // Member = PrivateStruct => ActionFn(11);
                let __sym0 = __pop_NtPrivateStruct(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember(__nt), __end));
                25
            }
            46 => {
                // Member = Init => ActionFn(12);
                let __sym0 = __pop_NtInit(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember(__nt), __end));
                25
            }
            47 => {
                // Member = Method => ActionFn(13);
                let __sym0 = __pop_NtMethod(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember(__nt), __end));
                25
            }
            48 => {
                // Member* =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtMember_2a(__nt), __end));
                26
            }
            49 => {
                // Member* = Member+ => ActionFn(41);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember_2a(__nt), __end));
                26
            }
            50 => {
                // Member+ = Member => ActionFn(50);
                let __sym0 = __pop_NtMember(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action50::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember_2b(__nt), __end));
                27
            }
            51 => {
                // Member+ = Member+, Member => ActionFn(51);
                let __sym1 = __pop_NtMember(__symbols);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMember_2b(__nt), __end));
                27
            }
            52 => {
                // Members =  => ActionFn(86);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action86::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtMembers(__nt), __end));
                28
            }
            53 => {
                // Members = Member+ => ActionFn(87);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action87::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMembers(__nt), __end));
                28
            }
            54 => {
                // Method = "fn", Id, FnDef => ActionFn(16);
                let __sym2 = __pop_NtFnDef(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22fn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                29
            }
            55 => {
                // PrivateStruct = "struct", Id, "{..}" => ActionFn(77);
                let __sym2 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22struct_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action77::<>(__sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPrivateStruct(__nt), __end));
                30
            }
            56 => {
                // Program =  => ActionFn(78);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action78::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                31
            }
            57 => {
                // Program = Class+ => ActionFn(79);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action79::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                31
            }
            58 => {
                // ReturnTy = "->", Type => ActionFn(66);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action66::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtReturnTy(__nt), __end));
                32
            }
            59 => {
                // ReturnTy =  => ActionFn(67);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action67::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtReturnTy(__nt), __end));
                32
            }
            60 => {
                // Type = Type1 => ActionFn(23);
                let __sym0 = __pop_NtType1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                33
            }
            61 => {
                // Type = Type1, ("+" <Type1>)+ => ActionFn(24);
                let __sym1 = __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b(__symbols);
                let __sym0 = __pop_NtType1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                33
            }
            62 => {
                // Type* =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtType_2a(__nt), __end));
                34
            }
            63 => {
                // Type* = Type+ => ActionFn(30);
                let __sym0 = __pop_NtType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType_2a(__nt), __end));
                34
            }
            64 => {
                // Type+ = Type => ActionFn(59);
                let __sym0 = __pop_NtType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType_2b(__nt), __end));
                35
            }
            65 => {
                // Type+ = Type+, Type => ActionFn(60);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_NtType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action60::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtType_2b(__nt), __end));
                35
            }
            66 => {
                // Type1 = Id => ActionFn(25);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            67 => {
                // Type1 = Id, "<", ">" => ActionFn(88);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action88::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            68 => {
                // Type1 = Id, "<", Type+, ">" => ActionFn(89);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtType_2b(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action89::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            69 => {
                // Type1 = "[", Type, "]" => ActionFn(27);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            70 => {
                // __Fields = Fields => ActionFn(2);
                let __sym0 = __pop_NtFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            71 => {
                // __Members = Members => ActionFn(1);
                let __sym0 = __pop_NtMembers(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Members(__nt), __end));
                38
            }
            72 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                39
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 40 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_26_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_26_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22class_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22class_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22extends_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22extends_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22fn_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22fn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22init_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22init_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22self_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22self_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22struct_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22struct_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_2e_2e_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_2e_2e_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermOtherId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermOtherId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Tok<'input>, ::errors::Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2b_22_20_3cType1_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2d_3e_22_20_3cType_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2d_3e_22_20_3cType_3e_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22extends_22_20_3cId_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22extends_22_20_3cId_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22extends_22_20_3cId_3e_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Field, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40L<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40L(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtClass<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Class, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtClass(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtClass_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Class>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtClass_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtClass_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Class>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtClass_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCodeBlock<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, OpaqueTokens, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCodeBlock(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cField_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cField_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Field, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFields<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFields(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFnDef<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FnDef, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFnDef(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFnSig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FnSig, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFnSig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdStr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdStr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInit<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, OpaqueTokens, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInit(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMember<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Member, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMember(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMember_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMember_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMember_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMember_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMembers<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMembers(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMethod<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Method, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMethod(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPrivateStruct<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PrivateStruct, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPrivateStruct(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtReturnTy<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtReturnTy(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Fields<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Fields(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Members<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Members(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Fields::parse_Fields;

mod __parse__Members {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::*;
    use lalrpop_intern::intern;
    use parser;
    use quote::Tokens;
    use tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_26_22(Tok<'input>),
        Term_22_28_22(Tok<'input>),
        Term_22_29_22(Tok<'input>),
        Term_22_2b_22(Tok<'input>),
        Term_22_2c_22(Tok<'input>),
        Term_22_2d_3e_22(Tok<'input>),
        Term_22_3a_22(Tok<'input>),
        Term_22_3c_22(Tok<'input>),
        Term_22_3e_22(Tok<'input>),
        Term_22_5b_22(Tok<'input>),
        Term_22_5d_22(Tok<'input>),
        Term_22class_22(Tok<'input>),
        Term_22extends_22(Tok<'input>),
        Term_22fn_22(Tok<'input>),
        Term_22init_22(Tok<'input>),
        Term_22self_22(Tok<'input>),
        Term_22struct_22(Tok<'input>),
        Term_22_7b_2e_2e_7d_22(&'input str),
        TermOtherId(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, Tok<'input>, ::errors::Error>),
        Nt_28_22_2b_22_20_3cType1_3e_29(Type),
        Nt_28_22_2b_22_20_3cType1_3e_29_2b(::std::vec::Vec<Type>),
        Nt_28_22_2d_3e_22_20_3cType_3e_29(Type),
        Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(::std::option::Option<Type>),
        Nt_28_22extends_22_20_3cId_3e_29(Identifier),
        Nt_28_22extends_22_20_3cId_3e_29_3f(::std::option::Option<Identifier>),
        Nt_28_3cField_3e_20_22_2c_22_29(Field),
        Nt_28_3cField_3e_20_22_2c_22_29_2a(::std::vec::Vec<Field>),
        Nt_28_3cField_3e_20_22_2c_22_29_2b(::std::vec::Vec<Field>),
        Nt_40L(usize),
        NtClass(Class),
        NtClass_2a(::std::vec::Vec<Class>),
        NtClass_2b(::std::vec::Vec<Class>),
        NtCodeBlock(OpaqueTokens),
        NtComma_3cField_3e(Vec<Field>),
        NtField(Field),
        NtField_2a(::std::vec::Vec<Field>),
        NtField_2b(::std::vec::Vec<Field>),
        NtField_3f(::std::option::Option<Field>),
        NtFields(Vec<Field>),
        NtFnDef(FnDef),
        NtFnSig(FnSig),
        NtId(Identifier),
        NtIdStr(&'input str),
        NtInit(OpaqueTokens),
        NtMember(Member),
        NtMember_2a(::std::vec::Vec<Member>),
        NtMember_2b(::std::vec::Vec<Member>),
        NtMembers(Vec<Member>),
        NtMethod(Method),
        NtPrivateStruct(PrivateStruct),
        NtProgram(Program),
        NtReturnTy(Option<Type>),
        NtType(Type),
        NtType_2a(::std::vec::Vec<Type>),
        NtType_2b(::std::vec::Vec<Type>),
        NtType1(Type),
        Nt____Fields(Vec<Field>),
        Nt____Members(Vec<Member>),
        Nt____Program(Program),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 0, 10, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, -46, 0, -46, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, -50, 0, -50, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 0, 10, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, -47, 0, -47, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, 0, -45, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, -51, 0, -51, 0, 0, 0,
        // State 11
        0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, -39, -39, -39, -39, 0, -39, -39, -39, -39, -39, -39, -39, 0, -39, 0, 0, -39, -39, 0,
        // State 13
        0, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, -41, -41, 0, -41, 0, 0, -41, -41, 0,
        // State 14
        0, -42, -42, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, 0, -42, 0, 0, -42, -42, 0,
        // State 15
        0, -40, -40, -40, -40, 0, -40, -40, -40, -40, -40, -40, -40, 0, -40, 0, 0, -40, -40, 0,
        // State 16
        0, -43, -43, -43, -43, 0, -43, -43, -43, -43, -43, -43, -43, 0, -43, 0, 0, -43, -43, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, -44, 0, -44, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, -22, 0, -22, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, -54, 0, -54, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0,
        // State 22
        26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, -55, 0, -55, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, -36, 0, -36, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0,
        // State 26
        0, 0, 28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0,
        // State 28
        0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 31
        0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 32
        0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, -23, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, -66, -66, -66, 0, 0, 44, -66, -66, -66, -66, -66, 0, -66, 0, 0, -66, -66, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0,
        // State 37
        0, 0, -60, 46, -60, 0, 0, 0, -60, -60, -60, -60, -60, 0, -60, 0, 0, -60, -60, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 39
        0, 0, -25, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0,
        // State 41
        0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, 0, -13, 0, 0, 0, -13, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 53, 39, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 44
        0, 0, -61, 54, -61, 0, 0, 0, -61, -61, -61, -61, -61, 0, -61, 0, 0, -61, -61, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, 0, -14, 0, 0, 0, -14, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0,
        // State 49
        0, 0, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, -64, -64, 0, -64, -64, 0, -64, 0, 0, 0, -64, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 58, 39, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 52
        0, 0, -67, -67, -67, 0, 0, 0, -67, -67, -67, -67, -67, 0, -67, 0, 0, -67, -67, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 14, 15, 0, 16, 0, 0, 0, 17, 0,
        // State 54
        0, 0, -2, -2, -2, 0, 0, 0, -2, -2, -2, -2, -2, 0, -2, 0, 0, -2, -2, 0,
        // State 55
        0, 0, -69, -69, -69, 0, 0, 0, -69, -69, -69, -69, -69, 0, -69, 0, 0, -69, -69, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, -65, -65, 0, -65, -65, 0, -65, 0, 0, 0, -65, 0,
        // State 57
        0, 0, -68, -68, -68, 0, 0, 0, -68, -68, -68, -68, -68, 0, -68, 0, 0, -68, -68, 0,
        // State 58
        0, 0, -3, -3, -3, 0, 0, 0, -3, -3, -3, -3, -3, 0, -3, 0, 0, -3, -3, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -52,
        -46,
        -50,
        -53,
        -71,
        -47,
        -45,
        0,
        0,
        0,
        -51,
        0,
        0,
        0,
        0,
        0,
        0,
        -44,
        -22,
        0,
        -54,
        0,
        0,
        -55,
        -36,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 11, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 33, 34, 0, 0, 0, 0, 0, 0, 35, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 38, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 35, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 38, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 38, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 52, 38, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 38, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""&""###,
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""->""###,
            r###"":""###,
            r###""<""###,
            r###"">""###,
            r###""[""###,
            r###""]""###,
            r###""class""###,
            r###""extends""###,
            r###""fn""###,
            r###""init""###,
            r###""self""###,
            r###""struct""###,
            r###""{..}""###,
            r###"OtherId"###,
        ];
        __ACTION[(__state * 20)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Members<
        'input,
        __TOKEN: __ToTriple<'input, Error=::errors::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Vec<Member>, __lalrpop_util::ParseError<usize, Tok<'input>, ::errors::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Tok::Ampersand if true => 0,
                Tok::LeftParen if true => 1,
                Tok::RightParen if true => 2,
                Tok::Plus if true => 3,
                Tok::Comma if true => 4,
                Tok::ThinArrow if true => 5,
                Tok::Colon if true => 6,
                Tok::LessThan if true => 7,
                Tok::GreaterThan if true => 8,
                Tok::LeftBracket if true => 9,
                Tok::RightBracket if true => 10,
                Tok::Class if true => 11,
                Tok::Extends if true => 12,
                Tok::Fn if true => 13,
                Tok::Init if true => 14,
                Tok::SelfKw if true => 15,
                Tok::Struct if true => 16,
                Tok::Block(_) if true => 17,
                Tok::Id(_) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 20 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Tok::Ampersand => __Symbol::Term_22_26_22(__tok),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Tok::LeftParen => __Symbol::Term_22_28_22(__tok),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Tok::RightParen => __Symbol::Term_22_29_22(__tok),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Tok::Plus => __Symbol::Term_22_2b_22(__tok),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Tok::Comma => __Symbol::Term_22_2c_22(__tok),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Tok::ThinArrow => __Symbol::Term_22_2d_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Tok::Colon => __Symbol::Term_22_3a_22(__tok),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Tok::LessThan => __Symbol::Term_22_3c_22(__tok),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Tok::GreaterThan => __Symbol::Term_22_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Tok::LeftBracket => __Symbol::Term_22_5b_22(__tok),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            __tok @ Tok::RightBracket => __Symbol::Term_22_5d_22(__tok),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Tok::Class => __Symbol::Term_22class_22(__tok),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            __tok @ Tok::Extends => __Symbol::Term_22extends_22(__tok),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Tok::Fn => __Symbol::Term_22fn_22(__tok),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            __tok @ Tok::Init => __Symbol::Term_22init_22(__tok),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            __tok @ Tok::SelfKw => __Symbol::Term_22self_22(__tok),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            __tok @ Tok::Struct => __Symbol::Term_22struct_22(__tok),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Tok::Block(__tok0) => __Symbol::Term_22_7b_2e_2e_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Tok::Id(__tok0) => __Symbol::TermOtherId(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Member>,__lalrpop_util::ParseError<usize, Tok<'input>, ::errors::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ("+" <Type1>) = "+", Type1 => ActionFn(33);
                let __sym1 = __pop_NtType1(__symbols);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29(__nt), __end));
                0
            }
            2 => {
                // ("+" <Type1>)+ = "+", Type1 => ActionFn(63);
                let __sym1 = __pop_NtType1(__symbols);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action63::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__nt), __end));
                1
            }
            3 => {
                // ("+" <Type1>)+ = ("+" <Type1>)+, "+", Type1 => ActionFn(64);
                let __sym2 = __pop_NtType1(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__nt), __end));
                1
            }
            4 => {
                // ("->" <Type>) = "->", Type => ActionFn(38);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29(__nt), __end));
                2
            }
            5 => {
                // ("->" <Type>)? = "->", Type => ActionFn(65);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action65::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(__nt), __end));
                3
            }
            6 => {
                // ("->" <Type>)? =  => ActionFn(37);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action37::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(__nt), __end));
                3
            }
            7 => {
                // ("extends" <Id>) = "extends", Id => ActionFn(45);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29(__nt), __end));
                4
            }
            8 => {
                // ("extends" <Id>)? = "extends", Id => ActionFn(68);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action68::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__nt), __end));
                5
            }
            9 => {
                // ("extends" <Id>)? =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__nt), __end));
                5
            }
            10 => {
                // (<Field> ",") = Field, "," => ActionFn(56);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action56::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29(__nt), __end));
                6
            }
            11 => {
                // (<Field> ",")* =  => ActionFn(54);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action54::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            12 => {
                // (<Field> ",")* = (<Field> ",")+ => ActionFn(55);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action55::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            13 => {
                // (<Field> ",")+ = Field, "," => ActionFn(71);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action71::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            14 => {
                // (<Field> ",")+ = (<Field> ",")+, Field, "," => ActionFn(72);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action72::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            15 => {
                // @L =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40L(__nt), __end));
                9
            }
            16 => {
                // Class = "class", Id, "extends", Id, "{..}" => ActionFn(75);
                let __sym4 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __sym3 = __pop_NtId(__symbols);
                let __sym2 = __pop_Term_22extends_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = match super::__action75::<>(__sym0, __sym1, __sym2, __sym3, __sym4) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtClass(__nt), __end));
                10
            }
            17 => {
                // Class = "class", Id, "{..}" => ActionFn(76);
                let __sym2 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action76::<>(__sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtClass(__nt), __end));
                10
            }
            18 => {
                // Class* =  => ActionFn(46);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action46::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtClass_2a(__nt), __end));
                11
            }
            19 => {
                // Class* = Class+ => ActionFn(47);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtClass_2a(__nt), __end));
                11
            }
            20 => {
                // Class+ = Class => ActionFn(48);
                let __sym0 = __pop_NtClass(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action48::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtClass_2b(__nt), __end));
                12
            }
            21 => {
                // Class+ = Class+, Class => ActionFn(49);
                let __sym1 = __pop_NtClass(__symbols);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action49::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtClass_2b(__nt), __end));
                12
            }
            22 => {
                // CodeBlock = "{..}" => ActionFn(28);
                let __sym0 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCodeBlock(__nt), __end));
                13
            }
            23 => {
                // Comma<Field> = Field => ActionFn(82);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action82::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            24 => {
                // Comma<Field> =  => ActionFn(83);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action83::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            25 => {
                // Comma<Field> = (<Field> ",")+, Field => ActionFn(84);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action84::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            26 => {
                // Comma<Field> = (<Field> ",")+ => ActionFn(85);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action85::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            27 => {
                // Field = Id, ":", Type => ActionFn(22);
                let __sym2 = __pop_NtType(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtField(__nt), __end));
                15
            }
            28 => {
                // Field* =  => ActionFn(34);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action34::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtField_2a(__nt), __end));
                16
            }
            29 => {
                // Field* = Field+ => ActionFn(35);
                let __sym0 = __pop_NtField_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtField_2a(__nt), __end));
                16
            }
            30 => {
                // Field+ = Field => ActionFn(57);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtField_2b(__nt), __end));
                17
            }
            31 => {
                // Field+ = Field+, Field => ActionFn(58);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_NtField_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtField_2b(__nt), __end));
                17
            }
            32 => {
                // Field? = Field => ActionFn(52);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action52::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtField_3f(__nt), __end));
                18
            }
            33 => {
                // Field? =  => ActionFn(53);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action53::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtField_3f(__nt), __end));
                18
            }
            34 => {
                // Fields =  => ActionFn(80);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action80::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtFields(__nt), __end));
                19
            }
            35 => {
                // Fields = Field+ => ActionFn(81);
                let __sym0 = __pop_NtField_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action81::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFields(__nt), __end));
                19
            }
            36 => {
                // FnDef = FnSig, CodeBlock => ActionFn(17);
                let __sym1 = __pop_NtCodeBlock(__symbols);
                let __sym0 = __pop_NtFnSig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtFnDef(__nt), __end));
                20
            }
            37 => {
                // FnSig = "(", "&", "self", ")", ReturnTy => ActionFn(18);
                let __sym4 = __pop_NtReturnTy(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Term_22self_22(__symbols);
                let __sym1 = __pop_Term_22_26_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtFnSig(__nt), __end));
                21
            }
            38 => {
                // FnSig = "(", "&", "self", ",", Comma<Field>, ")", ReturnTy => ActionFn(19);
                let __sym6 = __pop_NtReturnTy(__symbols);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtComma_3cField_3e(__symbols);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_Term_22self_22(__symbols);
                let __sym1 = __pop_Term_22_26_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtFnSig(__nt), __end));
                21
            }
            39 => {
                // Id = IdStr => ActionFn(6);
                let __sym0 = __pop_NtIdStr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                22
            }
            40 => {
                // IdStr = "init" => ActionFn(7);
                let __sym0 = __pop_Term_22init_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            41 => {
                // IdStr = "class" => ActionFn(8);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            42 => {
                // IdStr = "extends" => ActionFn(9);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            43 => {
                // IdStr = OtherId => ActionFn(10);
                let __sym0 = __pop_TermOtherId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            44 => {
                // Init = "init", CodeBlock => ActionFn(15);
                let __sym1 = __pop_NtCodeBlock(__symbols);
                let __sym0 = __pop_Term_22init_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtInit(__nt), __end));
                24
            }
            45 => {
                // Member = PrivateStruct => ActionFn(11);
                let __sym0 = __pop_NtPrivateStruct(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember(__nt), __end));
                25
            }
            46 => {
                // Member = Init => ActionFn(12);
                let __sym0 = __pop_NtInit(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember(__nt), __end));
                25
            }
            47 => {
                // Member = Method => ActionFn(13);
                let __sym0 = __pop_NtMethod(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember(__nt), __end));
                25
            }
            48 => {
                // Member* =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtMember_2a(__nt), __end));
                26
            }
            49 => {
                // Member* = Member+ => ActionFn(41);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember_2a(__nt), __end));
                26
            }
            50 => {
                // Member+ = Member => ActionFn(50);
                let __sym0 = __pop_NtMember(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action50::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember_2b(__nt), __end));
                27
            }
            51 => {
                // Member+ = Member+, Member => ActionFn(51);
                let __sym1 = __pop_NtMember(__symbols);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMember_2b(__nt), __end));
                27
            }
            52 => {
                // Members =  => ActionFn(86);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action86::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtMembers(__nt), __end));
                28
            }
            53 => {
                // Members = Member+ => ActionFn(87);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action87::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMembers(__nt), __end));
                28
            }
            54 => {
                // Method = "fn", Id, FnDef => ActionFn(16);
                let __sym2 = __pop_NtFnDef(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22fn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                29
            }
            55 => {
                // PrivateStruct = "struct", Id, "{..}" => ActionFn(77);
                let __sym2 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22struct_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action77::<>(__sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPrivateStruct(__nt), __end));
                30
            }
            56 => {
                // Program =  => ActionFn(78);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action78::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                31
            }
            57 => {
                // Program = Class+ => ActionFn(79);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action79::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                31
            }
            58 => {
                // ReturnTy = "->", Type => ActionFn(66);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action66::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtReturnTy(__nt), __end));
                32
            }
            59 => {
                // ReturnTy =  => ActionFn(67);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action67::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtReturnTy(__nt), __end));
                32
            }
            60 => {
                // Type = Type1 => ActionFn(23);
                let __sym0 = __pop_NtType1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                33
            }
            61 => {
                // Type = Type1, ("+" <Type1>)+ => ActionFn(24);
                let __sym1 = __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b(__symbols);
                let __sym0 = __pop_NtType1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                33
            }
            62 => {
                // Type* =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtType_2a(__nt), __end));
                34
            }
            63 => {
                // Type* = Type+ => ActionFn(30);
                let __sym0 = __pop_NtType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType_2a(__nt), __end));
                34
            }
            64 => {
                // Type+ = Type => ActionFn(59);
                let __sym0 = __pop_NtType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType_2b(__nt), __end));
                35
            }
            65 => {
                // Type+ = Type+, Type => ActionFn(60);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_NtType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action60::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtType_2b(__nt), __end));
                35
            }
            66 => {
                // Type1 = Id => ActionFn(25);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            67 => {
                // Type1 = Id, "<", ">" => ActionFn(88);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action88::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            68 => {
                // Type1 = Id, "<", Type+, ">" => ActionFn(89);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtType_2b(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action89::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            69 => {
                // Type1 = "[", Type, "]" => ActionFn(27);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            70 => {
                // __Fields = Fields => ActionFn(2);
                let __sym0 = __pop_NtFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Fields(__nt), __end));
                37
            }
            71 => {
                // __Members = Members => ActionFn(1);
                let __sym0 = __pop_NtMembers(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            72 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                39
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 40 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_26_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_26_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22class_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22class_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22extends_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22extends_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22fn_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22fn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22init_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22init_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22self_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22self_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22struct_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22struct_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_2e_2e_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_2e_2e_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermOtherId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermOtherId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Tok<'input>, ::errors::Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2b_22_20_3cType1_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2d_3e_22_20_3cType_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2d_3e_22_20_3cType_3e_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22extends_22_20_3cId_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22extends_22_20_3cId_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22extends_22_20_3cId_3e_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Field, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40L<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40L(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtClass<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Class, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtClass(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtClass_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Class>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtClass_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtClass_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Class>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtClass_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCodeBlock<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, OpaqueTokens, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCodeBlock(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cField_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cField_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Field, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFields<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFields(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFnDef<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FnDef, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFnDef(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFnSig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FnSig, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFnSig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdStr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdStr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInit<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, OpaqueTokens, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInit(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMember<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Member, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMember(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMember_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMember_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMember_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMember_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMembers<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMembers(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMethod<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Method, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMethod(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPrivateStruct<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PrivateStruct, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPrivateStruct(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtReturnTy<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtReturnTy(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Fields<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Fields(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Members<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Members(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Members::parse_Members;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast::*;
    use lalrpop_intern::intern;
    use parser;
    use quote::Tokens;
    use tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_26_22(Tok<'input>),
        Term_22_28_22(Tok<'input>),
        Term_22_29_22(Tok<'input>),
        Term_22_2b_22(Tok<'input>),
        Term_22_2c_22(Tok<'input>),
        Term_22_2d_3e_22(Tok<'input>),
        Term_22_3a_22(Tok<'input>),
        Term_22_3c_22(Tok<'input>),
        Term_22_3e_22(Tok<'input>),
        Term_22_5b_22(Tok<'input>),
        Term_22_5d_22(Tok<'input>),
        Term_22class_22(Tok<'input>),
        Term_22extends_22(Tok<'input>),
        Term_22fn_22(Tok<'input>),
        Term_22init_22(Tok<'input>),
        Term_22self_22(Tok<'input>),
        Term_22struct_22(Tok<'input>),
        Term_22_7b_2e_2e_7d_22(&'input str),
        TermOtherId(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, Tok<'input>, ::errors::Error>),
        Nt_28_22_2b_22_20_3cType1_3e_29(Type),
        Nt_28_22_2b_22_20_3cType1_3e_29_2b(::std::vec::Vec<Type>),
        Nt_28_22_2d_3e_22_20_3cType_3e_29(Type),
        Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(::std::option::Option<Type>),
        Nt_28_22extends_22_20_3cId_3e_29(Identifier),
        Nt_28_22extends_22_20_3cId_3e_29_3f(::std::option::Option<Identifier>),
        Nt_28_3cField_3e_20_22_2c_22_29(Field),
        Nt_28_3cField_3e_20_22_2c_22_29_2a(::std::vec::Vec<Field>),
        Nt_28_3cField_3e_20_22_2c_22_29_2b(::std::vec::Vec<Field>),
        Nt_40L(usize),
        NtClass(Class),
        NtClass_2a(::std::vec::Vec<Class>),
        NtClass_2b(::std::vec::Vec<Class>),
        NtCodeBlock(OpaqueTokens),
        NtComma_3cField_3e(Vec<Field>),
        NtField(Field),
        NtField_2a(::std::vec::Vec<Field>),
        NtField_2b(::std::vec::Vec<Field>),
        NtField_3f(::std::option::Option<Field>),
        NtFields(Vec<Field>),
        NtFnDef(FnDef),
        NtFnSig(FnSig),
        NtId(Identifier),
        NtIdStr(&'input str),
        NtInit(OpaqueTokens),
        NtMember(Member),
        NtMember_2a(::std::vec::Vec<Member>),
        NtMember_2b(::std::vec::Vec<Member>),
        NtMembers(Vec<Member>),
        NtMethod(Method),
        NtPrivateStruct(PrivateStruct),
        NtProgram(Program),
        NtReturnTy(Option<Type>),
        NtType(Type),
        NtType_2a(::std::vec::Vec<Type>),
        NtType_2b(::std::vec::Vec<Type>),
        NtType1(Type),
        Nt____Fields(Vec<Field>),
        Nt____Members(Vec<Member>),
        Nt____Program(Program),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 10, 0, 11, 0, 0, 0, 12, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 14, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, -39, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, -41, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, -42, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, -40, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, -43, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 10, 0, 11, 0, 0, 0, 12, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -56,
        -20,
        -57,
        -72,
        0,
        -21,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -17,
        0,
        -16,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""&""###,
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""->""###,
            r###"":""###,
            r###""<""###,
            r###"">""###,
            r###""[""###,
            r###""]""###,
            r###""class""###,
            r###""extends""###,
            r###""fn""###,
            r###""init""###,
            r###""self""###,
            r###""struct""###,
            r###""{..}""###,
            r###"OtherId"###,
        ];
        __ACTION[(__state * 20)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Program<
        'input,
        __TOKEN: __ToTriple<'input, Error=::errors::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Program, __lalrpop_util::ParseError<usize, Tok<'input>, ::errors::Error>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Tok::Ampersand if true => 0,
                Tok::LeftParen if true => 1,
                Tok::RightParen if true => 2,
                Tok::Plus if true => 3,
                Tok::Comma if true => 4,
                Tok::ThinArrow if true => 5,
                Tok::Colon if true => 6,
                Tok::LessThan if true => 7,
                Tok::GreaterThan if true => 8,
                Tok::LeftBracket if true => 9,
                Tok::RightBracket if true => 10,
                Tok::Class if true => 11,
                Tok::Extends if true => 12,
                Tok::Fn if true => 13,
                Tok::Init if true => 14,
                Tok::SelfKw if true => 15,
                Tok::Struct if true => 16,
                Tok::Block(_) if true => 17,
                Tok::Id(_) if true => 18,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 20 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Tok::Ampersand => __Symbol::Term_22_26_22(__tok),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Tok::LeftParen => __Symbol::Term_22_28_22(__tok),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Tok::RightParen => __Symbol::Term_22_29_22(__tok),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Tok::Plus => __Symbol::Term_22_2b_22(__tok),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Tok::Comma => __Symbol::Term_22_2c_22(__tok),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Tok::ThinArrow => __Symbol::Term_22_2d_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Tok::Colon => __Symbol::Term_22_3a_22(__tok),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Tok::LessThan => __Symbol::Term_22_3c_22(__tok),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Tok::GreaterThan => __Symbol::Term_22_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Tok::LeftBracket => __Symbol::Term_22_5b_22(__tok),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            __tok @ Tok::RightBracket => __Symbol::Term_22_5d_22(__tok),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Tok::Class => __Symbol::Term_22class_22(__tok),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            __tok @ Tok::Extends => __Symbol::Term_22extends_22(__tok),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Tok::Fn => __Symbol::Term_22fn_22(__tok),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            __tok @ Tok::Init => __Symbol::Term_22init_22(__tok),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            __tok @ Tok::SelfKw => __Symbol::Term_22self_22(__tok),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            __tok @ Tok::Struct => __Symbol::Term_22struct_22(__tok),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Tok::Block(__tok0) => __Symbol::Term_22_7b_2e_2e_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Tok::Id(__tok0) => __Symbol::TermOtherId(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize, Tok<'input>, ::errors::Error>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ("+" <Type1>) = "+", Type1 => ActionFn(33);
                let __sym1 = __pop_NtType1(__symbols);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29(__nt), __end));
                0
            }
            2 => {
                // ("+" <Type1>)+ = "+", Type1 => ActionFn(63);
                let __sym1 = __pop_NtType1(__symbols);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action63::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__nt), __end));
                1
            }
            3 => {
                // ("+" <Type1>)+ = ("+" <Type1>)+, "+", Type1 => ActionFn(64);
                let __sym2 = __pop_NtType1(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__nt), __end));
                1
            }
            4 => {
                // ("->" <Type>) = "->", Type => ActionFn(38);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29(__nt), __end));
                2
            }
            5 => {
                // ("->" <Type>)? = "->", Type => ActionFn(65);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action65::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(__nt), __end));
                3
            }
            6 => {
                // ("->" <Type>)? =  => ActionFn(37);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action37::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(__nt), __end));
                3
            }
            7 => {
                // ("extends" <Id>) = "extends", Id => ActionFn(45);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29(__nt), __end));
                4
            }
            8 => {
                // ("extends" <Id>)? = "extends", Id => ActionFn(68);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action68::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__nt), __end));
                5
            }
            9 => {
                // ("extends" <Id>)? =  => ActionFn(44);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action44::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__nt), __end));
                5
            }
            10 => {
                // (<Field> ",") = Field, "," => ActionFn(56);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action56::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29(__nt), __end));
                6
            }
            11 => {
                // (<Field> ",")* =  => ActionFn(54);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action54::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            12 => {
                // (<Field> ",")* = (<Field> ",")+ => ActionFn(55);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action55::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__nt), __end));
                7
            }
            13 => {
                // (<Field> ",")+ = Field, "," => ActionFn(71);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action71::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            14 => {
                // (<Field> ",")+ = (<Field> ",")+, Field, "," => ActionFn(72);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action72::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__nt), __end));
                8
            }
            15 => {
                // @L =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_40L(__nt), __end));
                9
            }
            16 => {
                // Class = "class", Id, "extends", Id, "{..}" => ActionFn(75);
                let __sym4 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __sym3 = __pop_NtId(__symbols);
                let __sym2 = __pop_Term_22extends_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = match super::__action75::<>(__sym0, __sym1, __sym2, __sym3, __sym4) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtClass(__nt), __end));
                10
            }
            17 => {
                // Class = "class", Id, "{..}" => ActionFn(76);
                let __sym2 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action76::<>(__sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtClass(__nt), __end));
                10
            }
            18 => {
                // Class* =  => ActionFn(46);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action46::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtClass_2a(__nt), __end));
                11
            }
            19 => {
                // Class* = Class+ => ActionFn(47);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtClass_2a(__nt), __end));
                11
            }
            20 => {
                // Class+ = Class => ActionFn(48);
                let __sym0 = __pop_NtClass(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action48::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtClass_2b(__nt), __end));
                12
            }
            21 => {
                // Class+ = Class+, Class => ActionFn(49);
                let __sym1 = __pop_NtClass(__symbols);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action49::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtClass_2b(__nt), __end));
                12
            }
            22 => {
                // CodeBlock = "{..}" => ActionFn(28);
                let __sym0 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtCodeBlock(__nt), __end));
                13
            }
            23 => {
                // Comma<Field> = Field => ActionFn(82);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action82::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            24 => {
                // Comma<Field> =  => ActionFn(83);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action83::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            25 => {
                // Comma<Field> = (<Field> ",")+, Field => ActionFn(84);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action84::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            26 => {
                // Comma<Field> = (<Field> ",")+ => ActionFn(85);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action85::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                14
            }
            27 => {
                // Field = Id, ":", Type => ActionFn(22);
                let __sym2 = __pop_NtType(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtField(__nt), __end));
                15
            }
            28 => {
                // Field* =  => ActionFn(34);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action34::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtField_2a(__nt), __end));
                16
            }
            29 => {
                // Field* = Field+ => ActionFn(35);
                let __sym0 = __pop_NtField_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtField_2a(__nt), __end));
                16
            }
            30 => {
                // Field+ = Field => ActionFn(57);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action57::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtField_2b(__nt), __end));
                17
            }
            31 => {
                // Field+ = Field+, Field => ActionFn(58);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_NtField_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtField_2b(__nt), __end));
                17
            }
            32 => {
                // Field? = Field => ActionFn(52);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action52::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtField_3f(__nt), __end));
                18
            }
            33 => {
                // Field? =  => ActionFn(53);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action53::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtField_3f(__nt), __end));
                18
            }
            34 => {
                // Fields =  => ActionFn(80);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action80::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtFields(__nt), __end));
                19
            }
            35 => {
                // Fields = Field+ => ActionFn(81);
                let __sym0 = __pop_NtField_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action81::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFields(__nt), __end));
                19
            }
            36 => {
                // FnDef = FnSig, CodeBlock => ActionFn(17);
                let __sym1 = __pop_NtCodeBlock(__symbols);
                let __sym0 = __pop_NtFnSig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtFnDef(__nt), __end));
                20
            }
            37 => {
                // FnSig = "(", "&", "self", ")", ReturnTy => ActionFn(18);
                let __sym4 = __pop_NtReturnTy(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_Term_22self_22(__symbols);
                let __sym1 = __pop_Term_22_26_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action18::<>(__sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtFnSig(__nt), __end));
                21
            }
            38 => {
                // FnSig = "(", "&", "self", ",", Comma<Field>, ")", ReturnTy => ActionFn(19);
                let __sym6 = __pop_NtReturnTy(__symbols);
                let __sym5 = __pop_Term_22_29_22(__symbols);
                let __sym4 = __pop_NtComma_3cField_3e(__symbols);
                let __sym3 = __pop_Term_22_2c_22(__symbols);
                let __sym2 = __pop_Term_22self_22(__symbols);
                let __sym1 = __pop_Term_22_26_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action19::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtFnSig(__nt), __end));
                21
            }
            39 => {
                // Id = IdStr => ActionFn(6);
                let __sym0 = __pop_NtIdStr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                22
            }
            40 => {
                // IdStr = "init" => ActionFn(7);
                let __sym0 = __pop_Term_22init_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            41 => {
                // IdStr = "class" => ActionFn(8);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            42 => {
                // IdStr = "extends" => ActionFn(9);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            43 => {
                // IdStr = OtherId => ActionFn(10);
                let __sym0 = __pop_TermOtherId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdStr(__nt), __end));
                23
            }
            44 => {
                // Init = "init", CodeBlock => ActionFn(15);
                let __sym1 = __pop_NtCodeBlock(__symbols);
                let __sym0 = __pop_Term_22init_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtInit(__nt), __end));
                24
            }
            45 => {
                // Member = PrivateStruct => ActionFn(11);
                let __sym0 = __pop_NtPrivateStruct(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember(__nt), __end));
                25
            }
            46 => {
                // Member = Init => ActionFn(12);
                let __sym0 = __pop_NtInit(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember(__nt), __end));
                25
            }
            47 => {
                // Member = Method => ActionFn(13);
                let __sym0 = __pop_NtMethod(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember(__nt), __end));
                25
            }
            48 => {
                // Member* =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtMember_2a(__nt), __end));
                26
            }
            49 => {
                // Member* = Member+ => ActionFn(41);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember_2a(__nt), __end));
                26
            }
            50 => {
                // Member+ = Member => ActionFn(50);
                let __sym0 = __pop_NtMember(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action50::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember_2b(__nt), __end));
                27
            }
            51 => {
                // Member+ = Member+, Member => ActionFn(51);
                let __sym1 = __pop_NtMember(__symbols);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMember_2b(__nt), __end));
                27
            }
            52 => {
                // Members =  => ActionFn(86);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action86::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtMembers(__nt), __end));
                28
            }
            53 => {
                // Members = Member+ => ActionFn(87);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action87::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMembers(__nt), __end));
                28
            }
            54 => {
                // Method = "fn", Id, FnDef => ActionFn(16);
                let __sym2 = __pop_NtFnDef(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22fn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtMethod(__nt), __end));
                29
            }
            55 => {
                // PrivateStruct = "struct", Id, "{..}" => ActionFn(77);
                let __sym2 = __pop_Term_22_7b_2e_2e_7d_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22struct_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action77::<>(__sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPrivateStruct(__nt), __end));
                30
            }
            56 => {
                // Program =  => ActionFn(78);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action78::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                31
            }
            57 => {
                // Program = Class+ => ActionFn(79);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action79::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                31
            }
            58 => {
                // ReturnTy = "->", Type => ActionFn(66);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_2d_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action66::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtReturnTy(__nt), __end));
                32
            }
            59 => {
                // ReturnTy =  => ActionFn(67);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action67::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtReturnTy(__nt), __end));
                32
            }
            60 => {
                // Type = Type1 => ActionFn(23);
                let __sym0 = __pop_NtType1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                33
            }
            61 => {
                // Type = Type1, ("+" <Type1>)+ => ActionFn(24);
                let __sym1 = __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b(__symbols);
                let __sym0 = __pop_NtType1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action24::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                33
            }
            62 => {
                // Type* =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtType_2a(__nt), __end));
                34
            }
            63 => {
                // Type* = Type+ => ActionFn(30);
                let __sym0 = __pop_NtType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType_2a(__nt), __end));
                34
            }
            64 => {
                // Type+ = Type => ActionFn(59);
                let __sym0 = __pop_NtType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action59::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType_2b(__nt), __end));
                35
            }
            65 => {
                // Type+ = Type+, Type => ActionFn(60);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_NtType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action60::<>(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtType_2b(__nt), __end));
                35
            }
            66 => {
                // Type1 = Id => ActionFn(25);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            67 => {
                // Type1 = Id, "<", ">" => ActionFn(88);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action88::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            68 => {
                // Type1 = Id, "<", Type+, ">" => ActionFn(89);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtType_2b(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action89::<>(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            69 => {
                // Type1 = "[", Type, "]" => ActionFn(27);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                36
            }
            70 => {
                // __Fields = Fields => ActionFn(2);
                let __sym0 = __pop_NtFields(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Fields(__nt), __end));
                37
            }
            71 => {
                // __Members = Members => ActionFn(1);
                let __sym0 = __pop_NtMembers(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Members(__nt), __end));
                38
            }
            72 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 40 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_26_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_26_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22class_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22class_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22extends_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22extends_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22fn_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22fn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22init_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22init_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22self_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22self_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22struct_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Tok<'input>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22struct_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_2e_2e_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_2e_2e_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermOtherId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermOtherId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Tok<'input>, ::errors::Error>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termerror(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2b_22_20_3cType1_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2d_3e_22_20_3cType_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_2d_3e_22_20_3cType_3e_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_2d_3e_22_20_3cType_3e_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22extends_22_20_3cId_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22extends_22_20_3cId_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22extends_22_20_3cId_3e_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Identifier>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Field, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_40L<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_40L(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtClass<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Class, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtClass(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtClass_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Class>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtClass_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtClass_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Class>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtClass_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCodeBlock<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, OpaqueTokens, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCodeBlock(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cField_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cField_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Field, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtField_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtField_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFields<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFields(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFnDef<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FnDef, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFnDef(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFnSig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FnSig, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFnSig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdStr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdStr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtInit<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, OpaqueTokens, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInit(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMember<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Member, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMember(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMember_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMember_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMember_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMember_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMembers<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMembers(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMethod<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Method, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMethod(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPrivateStruct<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PrivateStruct, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPrivateStruct(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtReturnTy<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtReturnTy(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Type>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtType1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Type, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtType1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Fields<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Field>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Fields(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Members<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Member>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Members(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Program::parse_Program;

pub fn __action0<
    'input,
>(
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    (__0)
}

pub fn __action1<
    'input,
>(
    (_, __0, _): (usize, Vec<Member>, usize),
) -> Vec<Member>
{
    (__0)
}

pub fn __action2<
    'input,
>(
    (_, __0, _): (usize, Vec<Field>, usize),
) -> Vec<Field>
{
    (__0)
}

pub fn __action3<
    'input,
>(
    (_, __0, _): (usize, ::std::vec::Vec<Class>, usize),
) -> Program
{
    Program { classes: __0 }
}

pub fn __action4<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, name, _): (usize, Identifier, usize),
    (_, extends, _): (usize, ::std::option::Option<Identifier>, usize),
    (_, b, _): (usize, usize, usize),
    (_, blk, _): (usize, &'input str, usize),
) -> Result<Class,__lalrpop_util::ParseError<usize,Tok<'input>,::errors::Error>>
{
    {
        let members = parser::parse_members(blk, b)?;
        Ok(Class { name, extends, members })
    }
}

pub fn __action5<
    'input,
>(
    (_, __0, _): (usize, ::std::vec::Vec<Member>, usize),
) -> Vec<Member>
{
    (__0)
}

pub fn __action6<
    'input,
>(
    (_, __0, _): (usize, &'input str, usize),
) -> Identifier
{
    Identifier { str: intern(__0) }
}

pub fn __action7<
    'input,
>(
    (_, __0, _): (usize, Tok<'input>, usize),
) -> &'input str
{
    "init"
}

pub fn __action8<
    'input,
>(
    (_, __0, _): (usize, Tok<'input>, usize),
) -> &'input str
{
    "class"
}

pub fn __action9<
    'input,
>(
    (_, __0, _): (usize, Tok<'input>, usize),
) -> &'input str
{
    "extends"
}

pub fn __action10<
    'input,
>(
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

pub fn __action11<
    'input,
>(
    (_, __0, _): (usize, PrivateStruct, usize),
) -> Member
{
    Member::PrivateStruct(__0)
}

pub fn __action12<
    'input,
>(
    (_, __0, _): (usize, OpaqueTokens, usize),
) -> Member
{
    Member::Init(__0)
}

pub fn __action13<
    'input,
>(
    (_, __0, _): (usize, Method, usize),
) -> Member
{
    Member::Method(__0)
}

pub fn __action14<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, name, _): (usize, Identifier, usize),
    (_, s, _): (usize, usize, usize),
    (_, fields, _): (usize, &'input str, usize),
) -> Result<PrivateStruct,__lalrpop_util::ParseError<usize,Tok<'input>,::errors::Error>>
{
    {
        let fields = parser::parse_fields(fields, s)?;
        Ok(PrivateStruct { name, fields })
    }
}

pub fn __action15<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, OpaqueTokens, usize),
) -> OpaqueTokens
{
    (__0)
}

pub fn __action16<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, name, _): (usize, Identifier, usize),
    (_, fn_def, _): (usize, FnDef, usize),
) -> Method
{
    {
        Method { name, fn_def }
    }
}

pub fn __action17<
    'input,
>(
    (_, sig, _): (usize, FnSig, usize),
    (_, code, _): (usize, OpaqueTokens, usize),
) -> FnDef
{
    FnDef { sig, code }
}

pub fn __action18<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, return_ty, _): (usize, Option<Type>, usize),
) -> FnSig
{
    {
        FnSig { args: vec![], return_ty }
    }
}

pub fn __action19<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, args, _): (usize, Vec<Field>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, return_ty, _): (usize, Option<Type>, usize),
) -> FnSig
{
    {
        FnSig { args, return_ty }
    }
}

pub fn __action20<
    'input,
>(
    (_, __0, _): (usize, ::std::option::Option<Type>, usize),
) -> Option<Type>
{
    (__0)
}

pub fn __action21<
    'input,
>(
    (_, __0, _): (usize, ::std::vec::Vec<Field>, usize),
) -> Vec<Field>
{
    (__0)
}

pub fn __action22<
    'input,
>(
    (_, name, _): (usize, Identifier, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, ty, _): (usize, Type, usize),
) -> Field
{
    Field { name, ty }
}

pub fn __action23<
    'input,
>(
    (_, __0, _): (usize, Type, usize),
) -> Type
{
    (__0)
}

pub fn __action24<
    'input,
>(
    (_, h, _): (usize, Type, usize),
    (_, t, _): (usize, ::std::vec::Vec<Type>, usize),
) -> Type
{
    Type::Sum(Some(h).into_iter().chain(t).collect())
}

pub fn __action25<
    'input,
>(
    (_, __0, _): (usize, Identifier, usize),
) -> Type
{
    Type::Name(__0)
}

pub fn __action26<
    'input,
>(
    (_, __0, _): (usize, Identifier, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __1, _): (usize, ::std::vec::Vec<Type>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Type
{
    Type::Args(__0, __1)
}

pub fn __action27<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Type, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Type
{
    Type::Array(Box::new(__0))
}

pub fn __action28<
    'input,
>(
    (_, __0, _): (usize, &'input str, usize),
) -> OpaqueTokens
{
    {
        let mut tokens = Tokens::new();
        tokens.append(__0);
        OpaqueTokens { tokens }
    }
}

pub fn __action29<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Type>
{
    vec![]
}

pub fn __action30<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Type>, usize),
) -> ::std::vec::Vec<Type>
{
    v
}

pub fn __action31<
    'input,
>(
    (_, __0, _): (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    vec![__0]
}

pub fn __action32<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Type>, usize),
    (_, e, _): (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action33<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Type, usize),
) -> Type
{
    (__0)
}

pub fn __action34<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Field>
{
    vec![]
}

pub fn __action35<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Field>, usize),
) -> ::std::vec::Vec<Field>
{
    v
}

pub fn __action36<
    'input,
>(
    (_, __0, _): (usize, Type, usize),
) -> ::std::option::Option<Type>
{
    Some(__0)
}

pub fn __action37<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Type>
{
    None
}

pub fn __action38<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Type, usize),
) -> Type
{
    (__0)
}

pub fn __action39<
    'input,
>(
    (_, h, _): (usize, ::std::vec::Vec<Field>, usize),
    (_, t, _): (usize, ::std::option::Option<Field>, usize),
) -> Vec<Field>
{
    {
        let mut h = h;
        h.extend(t);
        h
    }
}

pub fn __action40<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Member>
{
    vec![]
}

pub fn __action41<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Member>, usize),
) -> ::std::vec::Vec<Member>
{
    v
}

pub fn __action42<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

pub fn __action43<
    'input,
>(
    (_, __0, _): (usize, Identifier, usize),
) -> ::std::option::Option<Identifier>
{
    Some(__0)
}

pub fn __action44<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Identifier>
{
    None
}

pub fn __action45<
    'input,
>(
    (_, _, _): (usize, Tok<'input>, usize),
    (_, __0, _): (usize, Identifier, usize),
) -> Identifier
{
    (__0)
}

pub fn __action46<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Class>
{
    vec![]
}

pub fn __action47<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Class>, usize),
) -> ::std::vec::Vec<Class>
{
    v
}

pub fn __action48<
    'input,
>(
    (_, __0, _): (usize, Class, usize),
) -> ::std::vec::Vec<Class>
{
    vec![__0]
}

pub fn __action49<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Class>, usize),
    (_, e, _): (usize, Class, usize),
) -> ::std::vec::Vec<Class>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action50<
    'input,
>(
    (_, __0, _): (usize, Member, usize),
) -> ::std::vec::Vec<Member>
{
    vec![__0]
}

pub fn __action51<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Member>, usize),
    (_, e, _): (usize, Member, usize),
) -> ::std::vec::Vec<Member>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action52<
    'input,
>(
    (_, __0, _): (usize, Field, usize),
) -> ::std::option::Option<Field>
{
    Some(__0)
}

pub fn __action53<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Field>
{
    None
}

pub fn __action54<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Field>
{
    vec![]
}

pub fn __action55<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Field>, usize),
) -> ::std::vec::Vec<Field>
{
    v
}

pub fn __action56<
    'input,
>(
    (_, __0, _): (usize, Field, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Field
{
    (__0)
}

pub fn __action57<
    'input,
>(
    (_, __0, _): (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    vec![__0]
}

pub fn __action58<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Field>, usize),
    (_, e, _): (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action59<
    'input,
>(
    (_, __0, _): (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    vec![__0]
}

pub fn __action60<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Type>, usize),
    (_, e, _): (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action61<
    'input,
>(
    (_, __0, _): (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    vec![__0]
}

pub fn __action62<
    'input,
>(
    (_, v, _): (usize, ::std::vec::Vec<Field>, usize),
    (_, e, _): (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action63<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action33(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        __temp0,
    )
}

pub fn __action64<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Type>, usize),
    __1: (usize, Tok<'input>, usize),
    __2: (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action33(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        __0,
        __temp0,
    )
}

pub fn __action65<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Type, usize),
) -> ::std::option::Option<Type>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __temp0,
    )
}

pub fn __action66<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Type, usize),
) -> Option<Type>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action65(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        __temp0,
    )
}

pub fn __action67<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Option<Type>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action37(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        __temp0,
    )
}

pub fn __action68<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Identifier, usize),
) -> ::std::option::Option<Identifier>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action45(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        __temp0,
    )
}

pub fn __action69<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Identifier, usize),
    __2: (usize, Tok<'input>, usize),
    __3: (usize, Identifier, usize),
    __4: (usize, usize, usize),
    __5: (usize, &'input str, usize),
) -> Result<Class,__lalrpop_util::ParseError<usize,Tok<'input>,::errors::Error>>
{
    let __start0 = __2.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action68(
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __0,
        __1,
        __temp0,
        __4,
        __5,
    )
}

pub fn __action70<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Identifier, usize),
    __2: (usize, usize, usize),
    __3: (usize, &'input str, usize),
) -> Result<Class,__lalrpop_util::ParseError<usize,Tok<'input>,::errors::Error>>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action44(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __0,
        __1,
        __temp0,
        __2,
        __3,
    )
}

pub fn __action71<
    'input,
>(
    __0: (usize, Field, usize),
    __1: (usize, Tok<'input>, usize),
) -> ::std::vec::Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action56(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        __temp0,
    )
}

pub fn __action72<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Field>, usize),
    __1: (usize, Field, usize),
    __2: (usize, Tok<'input>, usize),
) -> ::std::vec::Vec<Field>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action56(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        __0,
        __temp0,
    )
}

pub fn __action73<
    'input,
>(
    __0: (usize, ::std::option::Option<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action54(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        __temp0,
        __0,
    )
}

pub fn __action74<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Field>, usize),
    __1: (usize, ::std::option::Option<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action55(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        __temp0,
        __1,
    )
}

pub fn __action75<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Identifier, usize),
    __2: (usize, Tok<'input>, usize),
    __3: (usize, Identifier, usize),
    __4: (usize, &'input str, usize),
) -> Result<Class,__lalrpop_util::ParseError<usize,Tok<'input>,::errors::Error>>
{
    let __start0 = __3.2.clone();
    let __end0 = __4.0.clone();
    let __temp0 = __action42(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        __0,
        __1,
        __2,
        __3,
        __temp0,
        __4,
    )
}

pub fn __action76<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Identifier, usize),
    __2: (usize, &'input str, usize),
) -> Result<Class,__lalrpop_util::ParseError<usize,Tok<'input>,::errors::Error>>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action42(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action70(
        __0,
        __1,
        __temp0,
        __2,
    )
}

pub fn __action77<
    'input,
>(
    __0: (usize, Tok<'input>, usize),
    __1: (usize, Identifier, usize),
    __2: (usize, &'input str, usize),
) -> Result<PrivateStruct,__lalrpop_util::ParseError<usize,Tok<'input>,::errors::Error>>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action42(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        __0,
        __1,
        __temp0,
        __2,
    )
}

pub fn __action78<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Program
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        __temp0,
    )
}

pub fn __action79<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Class>, usize),
) -> Program
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        __temp0,
    )
}

pub fn __action80<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Field>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action34(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        __temp0,
    )
}

pub fn __action81<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        __temp0,
    )
}

pub fn __action82<
    'input,
>(
    __0: (usize, Field, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action52(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        __temp0,
    )
}

pub fn __action83<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Field>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action53(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action73(
        __temp0,
    )
}

pub fn __action84<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Field>, usize),
    __1: (usize, Field, usize),
) -> Vec<Field>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action52(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        __0,
        __temp0,
    )
}

pub fn __action85<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action53(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action74(
        __0,
        __temp0,
    )
}

pub fn __action86<
    'input,
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Member>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        __temp0,
    )
}

pub fn __action87<
    'input,
>(
    __0: (usize, ::std::vec::Vec<Member>, usize),
) -> Vec<Member>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action41(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        __temp0,
    )
}

pub fn __action88<
    'input,
>(
    __0: (usize, Identifier, usize),
    __1: (usize, Tok<'input>, usize),
    __2: (usize, Tok<'input>, usize),
) -> Type
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action29(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        __0,
        __1,
        __temp0,
        __2,
    )
}

pub fn __action89<
    'input,
>(
    __0: (usize, Identifier, usize),
    __1: (usize, Tok<'input>, usize),
    __2: (usize, ::std::vec::Vec<Type>, usize),
    __3: (usize, Tok<'input>, usize),
) -> Type
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action30(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        __0,
        __1,
        __temp0,
        __3,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Tok<'input>, usize) {
    type Error = ::errors::Error;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),::errors::Error> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Tok<'input>, usize),::errors::Error> {
    type Error = ::errors::Error;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),::errors::Error> {
        value
    }
}
