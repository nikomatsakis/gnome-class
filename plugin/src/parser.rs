use lalrpop_intern::intern;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use lalrpop_intern::intern;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22class_22(&'input str),
        Term_22extends_22(&'input str),
        Term_22struct_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_5ba_2dzA_2dZ___5d_5b_5b_3aword_3a_5d_5d_2a_22_23(&'input str),
        Termerror(__lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>),
        Nt_28_22_2b_22_20_3cType1_3e_29(Type),
        Nt_28_22_2b_22_20_3cType1_3e_29_2b(::std::vec::Vec<Type>),
        Nt_28_22extends_22_20_3cId_3e_29(InternedString),
        Nt_28_22extends_22_20_3cId_3e_29_3f(::std::option::Option<InternedString>),
        Nt_28_3cField_3e_20_22_2c_22_29(Field),
        Nt_28_3cField_3e_20_22_2c_22_29_2a(::std::vec::Vec<Field>),
        Nt_28_3cField_3e_20_22_2c_22_29_2b(::std::vec::Vec<Field>),
        NtClass(Class),
        NtClass_2a(::std::vec::Vec<Class>),
        NtClass_2b(::std::vec::Vec<Class>),
        NtComma_3cField_3e(Vec<Field>),
        NtField(Field),
        NtField_3f(::std::option::Option<Field>),
        NtId(InternedString),
        NtMember(Member),
        NtMember_2a(::std::vec::Vec<Member>),
        NtMember_2b(::std::vec::Vec<Member>),
        NtPrivateStruct(PrivateStruct),
        NtProgram(Program),
        NtType(Type),
        NtType_2a(::std::vec::Vec<Type>),
        NtType_2b(::std::vec::Vec<Type>),
        NtType1(Type),
        Nt____Program(Program),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 10, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, -27, 0, -27, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 17, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, -31, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 20, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, -28, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 23, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, -32, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 25, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 30, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 30, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0,
        // State 27
        0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0,
        // State 28
        0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, -33, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, -10, 0,
        // State 33
        0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 40, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, -11, 0,
        // State 35
        -42, -42, 0, 41, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0,
        // State 36
        0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0,
        // State 37
        43, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 48, 0,
        // State 39
        -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0,
        // State 40
        0, 0, 0, 0, 53, 54, 0, 0, 0, 0, 0, 0, 55, 0,
        // State 41
        56, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 40, 0,
        // State 43
        -42, 0, 0, 58, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        61, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 48, 0,
        // State 47
        -27, 0, 0, -27, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        -42, 0, 0, 63, -42, -42, 0, 0, 0, 0, 0, 0, -42, 0,
        // State 49
        0, 0, 0, 0, -40, -40, 0, 0, 0, 0, 0, 0, -40, 0,
        // State 50
        0, 0, 0, 0, 65, 54, 0, 0, 0, 0, 0, 0, 55, 0,
        // State 51
        67, 0, 0, 0, -36, -36, 0, 0, 0, 0, 0, 0, -36, 0,
        // State 52
        -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 48, 0,
        // State 54
        -27, 0, 0, -27, -27, -27, 0, 0, 0, 0, 0, 0, -27, 0,
        // State 55
        0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 40, 0,
        // State 56
        -2, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0,
        // State 57
        0, 0, 0, 0, 71, 54, 0, 0, 0, 0, 0, 0, 55, 0,
        // State 58
        -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0,
        // State 59
        72, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 48, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 76, 54, 0, 0, 0, 0, 0, 0, 55, 0,
        // State 63
        0, 0, 0, 0, -41, -41, 0, 0, 0, 0, 0, 0, -41, 0,
        // State 64
        -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0,
        // State 65
        77, 0, 0, 0, -37, -37, 0, 0, 0, 0, 0, 0, -37, 0,
        // State 66
        0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 55, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        -3, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0,
        // State 69
        0, 0, 0, 0, 80, 54, 0, 0, 0, 0, 0, 0, 55, 0,
        // State 70
        -43, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 48, 0,
        // State 72
        -2, 0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        -45, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 82, 54, 0, 0, 0, 0, 0, 0, 55, 0,
        // State 75
        -43, 0, 0, 0, -43, -43, 0, 0, 0, 0, 0, 0, -43, 0,
        // State 76
        0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 55, 0,
        // State 77
        -2, 0, 0, 0, -2, -2, 0, 0, 0, 0, 0, 0, -2, 0,
        // State 78
        -45, 0, 0, 0, -45, -45, 0, 0, 0, 0, 0, 0, -45, 0,
        // State 79
        -44, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        -3, 0, 0, 0, 0, 0, -3, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        -44, 0, 0, 0, -44, -44, 0, 0, 0, 0, 0, 0, -44, 0,
        // State 82
        -3, 0, 0, 0, -3, -3, 0, 0, 0, 0, 0, 0, -3, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -34,
        -18,
        -35,
        -46,
        0,
        -19,
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
        -14,
        0,
        0,
        -15,
        0,
        0,
        -12,
        0,
        -13,
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
        0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 15, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 22, 15, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 15, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 27, 28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 37, 0, 0, 38, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 45, 0, 0, 46, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 50, 0, 51, 52, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 62, 0, 0, 46, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 64, 0, 0, 52, 0,
        // State 51
        0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 68, 0, 0, 46, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 50, 0, 70, 52, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 73, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 50, 0, 75, 52, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 78, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 64, 0, 0, 52, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 81, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 64, 0, 0, 52, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 76
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0,
        // State 77
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""+""###,
            r###"",""###,
            r###"":""###,
            r###""<""###,
            r###"">""###,
            r###""[""###,
            r###""]""###,
            r###""class""###,
            r###""extends""###,
            r###""struct""###,
            r###""{""###,
            r###""}""###,
            r###"r#"[a-zA-Z_][[:word:]]*"#"###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Program<
        'input,
    >(
        input: &'input str,
    ) -> Result<Program, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
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
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_2b_22(__tok0),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_2c_22(__tok0),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_3a_22(__tok0),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_3c_22(__tok0),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_3e_22(__tok0),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_5b_22(__tok0),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_5d_22(__tok0),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22class_22(__tok0),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22extends_22(__tok0),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22struct_22(__tok0),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_7b_22(__tok0),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_7d_22(__tok0),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ___5d_5b_5b_3aword_3a_5d_5d_2a_22_23(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ("+" <Type1>) = "+", Type1 => ActionFn(16);
                let __sym1 = __pop_NtType1(__symbols);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29(__nt), __end));
                0
            }
            2 => {
                // ("+" <Type1>)+ = "+", Type1 => ActionFn(38);
                let __sym1 = __pop_NtType1(__symbols);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__nt), __end));
                1
            }
            3 => {
                // ("+" <Type1>)+ = ("+" <Type1>)+, "+", Type1 => ActionFn(39);
                let __sym2 = __pop_NtType1(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22_2b_22_20_3cType1_3e_29_2b(__nt), __end));
                1
            }
            4 => {
                // ("extends" <Id>) = "extends", Id => ActionFn(22);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29(__nt), __end));
                2
            }
            5 => {
                // ("extends" <Id>)? = "extends", Id => ActionFn(40);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22extends_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action40::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__nt), __end));
                3
            }
            6 => {
                // ("extends" <Id>)? =  => ActionFn(21);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action21::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22extends_22_20_3cId_3e_29_3f(__nt), __end));
                3
            }
            7 => {
                // (<Field> ",") = Field, "," => ActionFn(33);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29(__nt), __end));
                4
            }
            8 => {
                // (<Field> ",")* =  => ActionFn(31);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action31::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__nt), __end));
                5
            }
            9 => {
                // (<Field> ",")* = (<Field> ",")+ => ActionFn(32);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2a(__nt), __end));
                5
            }
            10 => {
                // (<Field> ",")+ = Field, "," => ActionFn(43);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action43::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__nt), __end));
                6
            }
            11 => {
                // (<Field> ",")+ = (<Field> ",")+, Field, "," => ActionFn(44);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action44::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cField_3e_20_22_2c_22_29_2b(__nt), __end));
                6
            }
            12 => {
                // Class = "class", Id, "extends", Id, "{", "}" => ActionFn(53);
                let __sym5 = __pop_Term_22_7d_22(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtId(__symbols);
                let __sym2 = __pop_Term_22extends_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action53::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtClass(__nt), __end));
                7
            }
            13 => {
                // Class = "class", Id, "extends", Id, "{", Member+, "}" => ActionFn(54);
                let __sym6 = __pop_Term_22_7d_22(__symbols);
                let __sym5 = __pop_NtMember_2b(__symbols);
                let __sym4 = __pop_Term_22_7b_22(__symbols);
                let __sym3 = __pop_NtId(__symbols);
                let __sym2 = __pop_Term_22extends_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action54::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtClass(__nt), __end));
                7
            }
            14 => {
                // Class = "class", Id, "{", "}" => ActionFn(55);
                let __sym3 = __pop_Term_22_7d_22(__symbols);
                let __sym2 = __pop_Term_22_7b_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action55::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtClass(__nt), __end));
                7
            }
            15 => {
                // Class = "class", Id, "{", Member+, "}" => ActionFn(56);
                let __sym4 = __pop_Term_22_7d_22(__symbols);
                let __sym3 = __pop_NtMember_2b(__symbols);
                let __sym2 = __pop_Term_22_7b_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22class_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action56::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtClass(__nt), __end));
                7
            }
            16 => {
                // Class* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtClass_2a(__nt), __end));
                8
            }
            17 => {
                // Class* = Class+ => ActionFn(24);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtClass_2a(__nt), __end));
                8
            }
            18 => {
                // Class+ = Class => ActionFn(25);
                let __sym0 = __pop_NtClass(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtClass_2b(__nt), __end));
                9
            }
            19 => {
                // Class+ = Class+, Class => ActionFn(26);
                let __sym1 = __pop_NtClass(__symbols);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtClass_2b(__nt), __end));
                9
            }
            20 => {
                // Comma<Field> = Field => ActionFn(49);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action49::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                10
            }
            21 => {
                // Comma<Field> =  => ActionFn(50);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action50::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                10
            }
            22 => {
                // Comma<Field> = (<Field> ",")+, Field => ActionFn(51);
                let __sym1 = __pop_NtField(__symbols);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action51::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                10
            }
            23 => {
                // Comma<Field> = (<Field> ",")+ => ActionFn(52);
                let __sym0 = __pop_Nt_28_3cField_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action52::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cField_3e(__nt), __end));
                10
            }
            24 => {
                // Field = Id, ":", Type => ActionFn(6);
                let __sym2 = __pop_NtType(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtField(__nt), __end));
                11
            }
            25 => {
                // Field? = Field => ActionFn(29);
                let __sym0 = __pop_NtField(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtField_3f(__nt), __end));
                12
            }
            26 => {
                // Field? =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtField_3f(__nt), __end));
                12
            }
            27 => {
                // Id = r#"[a-zA-Z_][[:word:]]*"# => ActionFn(3);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ___5d_5b_5b_3aword_3a_5d_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                13
            }
            28 => {
                // Member = PrivateStruct => ActionFn(4);
                let __sym0 = __pop_NtPrivateStruct(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember(__nt), __end));
                14
            }
            29 => {
                // Member* =  => ActionFn(18);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action18::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtMember_2a(__nt), __end));
                15
            }
            30 => {
                // Member* = Member+ => ActionFn(19);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember_2a(__nt), __end));
                15
            }
            31 => {
                // Member+ = Member => ActionFn(27);
                let __sym0 = __pop_NtMember(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMember_2b(__nt), __end));
                16
            }
            32 => {
                // Member+ = Member+, Member => ActionFn(28);
                let __sym1 = __pop_NtMember(__symbols);
                let __sym0 = __pop_NtMember_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtMember_2b(__nt), __end));
                16
            }
            33 => {
                // PrivateStruct = "struct", Id, "{", Comma<Field>, "}" => ActionFn(5);
                let __sym4 = __pop_Term_22_7d_22(__symbols);
                let __sym3 = __pop_NtComma_3cField_3e(__symbols);
                let __sym2 = __pop_Term_22_7b_22(__symbols);
                let __sym1 = __pop_NtId(__symbols);
                let __sym0 = __pop_Term_22struct_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtPrivateStruct(__nt), __end));
                17
            }
            34 => {
                // Program =  => ActionFn(47);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action47::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                18
            }
            35 => {
                // Program = Class+ => ActionFn(48);
                let __sym0 = __pop_NtClass_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action48::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                18
            }
            36 => {
                // Type = Type1 => ActionFn(7);
                let __sym0 = __pop_NtType1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                19
            }
            37 => {
                // Type = Type1, ("+" <Type1>)+ => ActionFn(8);
                let __sym1 = __pop_Nt_28_22_2b_22_20_3cType1_3e_29_2b(__symbols);
                let __sym0 = __pop_NtType1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtType(__nt), __end));
                19
            }
            38 => {
                // Type* =  => ActionFn(12);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action12::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtType_2a(__nt), __end));
                20
            }
            39 => {
                // Type* = Type+ => ActionFn(13);
                let __sym0 = __pop_NtType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType_2a(__nt), __end));
                20
            }
            40 => {
                // Type+ = Type => ActionFn(34);
                let __sym0 = __pop_NtType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType_2b(__nt), __end));
                21
            }
            41 => {
                // Type+ = Type+, Type => ActionFn(35);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_NtType_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtType_2b(__nt), __end));
                21
            }
            42 => {
                // Type1 = Id => ActionFn(9);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                22
            }
            43 => {
                // Type1 = Id, "<", ">" => ActionFn(57);
                let __sym2 = __pop_Term_22_3e_22(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action57::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                22
            }
            44 => {
                // Type1 = Id, "<", Type+, ">" => ActionFn(58);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtType_2b(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action58::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                22
            }
            45 => {
                // Type1 = "[", Type, "]" => ActionFn(11);
                let __sym2 = __pop_Term_22_5d_22(__symbols);
                let __sym1 = __pop_NtType(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtType1(__nt), __end));
                22
            }
            46 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 24 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22class_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22class_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22extends_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22extends_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22struct_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22struct_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ___5d_5b_5b_3aword_3a_5d_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ___5d_5b_5b_3aword_3a_5d_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termerror<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, (usize, &'input str), ()>, usize) {
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
    fn __pop_Nt_28_22extends_22_20_3cId_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, InternedString, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22extends_22_20_3cId_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22extends_22_20_3cId_3e_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<InternedString>, usize) {
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
    fn __pop_NtId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, InternedString, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
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
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        43 => /* '+' */ {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        44 => /* ',' */ {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        58 => /* ':' */ {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        60 => /* '<' */ {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        62 => /* '>' */ {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        91 => /* '[' */ {
                            __current_match = Some((5, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        93 => /* ']' */ {
                            __current_match = Some((6, __index + 1));
                            __current_state = 8;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 9;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 10;
                            continue;
                        }
                        102 ... 114 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 11;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 6;
                            continue;
                        }
                        123 => /* '{' */ {
                            __current_match = Some((10, __index + 1));
                            __current_state = 12;
                            continue;
                        }
                        125 => /* '}' */ {
                            __current_match = Some((11, __index + 1));
                            __current_state = 13;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                9 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 107 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        108 => /* 'l' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 16;
                            continue;
                        }
                        109 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                10 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 119 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        120 => /* 'x' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 17;
                            continue;
                        }
                        121 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                11 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 18;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                12 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                13 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                14 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                15 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                16 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 => /* 'a' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 19;
                            continue;
                        }
                        98 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                17 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 20;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                18 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 113 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        114 => /* 'r' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 21;
                            continue;
                        }
                        115 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                19 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 22;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                20 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 100 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        101 => /* 'e' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 23;
                            continue;
                        }
                        102 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                21 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 116 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        117 => /* 'u' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 24;
                            continue;
                        }
                        118 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                22 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((7, __index + 1));
                            __current_state = 25;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                23 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 109 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        110 => /* 'n' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 26;
                            continue;
                        }
                        111 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                24 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 98 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        99 => /* 'c' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 27;
                            continue;
                        }
                        100 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                25 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                26 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 99 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        100 => /* 'd' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 28;
                            continue;
                        }
                        101 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                27 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 115 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        116 => /* 't' */ {
                            __current_match = Some((9, __index + 1));
                            __current_state = 29;
                            continue;
                        }
                        117 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                28 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 114 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        115 => /* 's' */ {
                            __current_match = Some((8, __index + 1));
                            __current_state = 30;
                            continue;
                        }
                        116 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                29 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                30 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch as u32 {
                        48 ... 57 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        65 ... 90 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        95 => /* '_' */ {
                            __current_match = Some((12, __index + 1));
                            __current_state = 15;
                            continue;
                        }
                        97 ... 122 => {
                            __current_match = Some((12, __index + __ch.len_utf8()));
                            __current_state = 15;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__lalrpop_util::ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Class>, usize),
) -> Program
{
    Program { classes: __0 }
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, InternedString, usize),
    (_, super, _): (usize, ::std::option::Option<InternedString>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, members, _): (usize, ::std::vec::Vec<Member>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Class
{
    {
        Class { name, extends, members }
    }
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> InternedString
{
    intern(__0)
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, PrivateStruct, usize),
) -> Member
{
    Member::PrivateStruct(__0)
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, InternedString, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, fields, _): (usize, Vec<Field>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> PrivateStruct
{
    {
        Member::PrivateStruct { name, fields }
    }
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, name, _): (usize, InternedString, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, Type, usize),
) -> Field
{
    Field { name, ty }
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Type, usize),
) -> Type
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, h, _): (usize, Type, usize),
    (_, t, _): (usize, ::std::vec::Vec<Type>, usize),
) -> Type
{
    Type::Sum(Some(h).into_iter().chain(t).collect())
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, InternedString, usize),
) -> Type
{
    Type::Name(__0)
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, InternedString, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, ::std::vec::Vec<Type>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Type
{
    Type::Name(__0, __1)
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Type, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Type
{
    Type::Array(Box::new(__0))
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Type>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Type>, usize),
) -> ::std::vec::Vec<Type>
{
    v
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Type>, usize),
    (_, e, _): (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Type, usize),
) -> Type
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
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

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Member>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Member>, usize),
) -> ::std::vec::Vec<Member>
{
    v
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, InternedString, usize),
) -> ::std::option::Option<InternedString>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<InternedString>
{
    None
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, InternedString, usize),
) -> InternedString
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Class>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Class>, usize),
) -> ::std::vec::Vec<Class>
{
    v
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Class, usize),
) -> ::std::vec::Vec<Class>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Class>, usize),
    (_, e, _): (usize, Class, usize),
) -> ::std::vec::Vec<Class>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Member, usize),
) -> ::std::vec::Vec<Member>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Member>, usize),
    (_, e, _): (usize, Member, usize),
) -> ::std::vec::Vec<Member>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
) -> ::std::option::Option<Field>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Field>
{
    None
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Field>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Field>, usize),
) -> ::std::vec::Vec<Field>
{
    v
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Field
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Type>, usize),
    (_, e, _): (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Field>, usize),
    (_, e, _): (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action38<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action16(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action39<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Type>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Type, usize),
) -> ::std::vec::Vec<Type>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action16(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action40<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, InternedString, usize),
) -> ::std::option::Option<InternedString>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action22(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action41<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, InternedString, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, InternedString, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, ::std::vec::Vec<Member>, usize),
    __6: (usize, &'input str, usize),
) -> Class
{
    let __start0 = __2.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action40(
        input,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __1,
        __temp0,
        __4,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action42<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, InternedString, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<Member>, usize),
    __4: (usize, &'input str, usize),
) -> Class
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action21(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __1,
        __temp0,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action43<
    'input,
>(
    input: &'input str,
    __0: (usize, Field, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action33(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action44<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Field>, usize),
    __1: (usize, Field, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Field>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action33(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action45<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action31(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action46<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Field>, usize),
    __1: (usize, ::std::option::Option<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action32(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action47<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Program
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action23(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action48<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Class>, usize),
) -> Program
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action24(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action49<
    'input,
>(
    input: &'input str,
    __0: (usize, Field, usize),
) -> Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action29(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action50<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Field>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action30(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action51<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Field>, usize),
    __1: (usize, Field, usize),
) -> Vec<Field>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action29(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action52<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Field>, usize),
) -> Vec<Field>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action30(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action53<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, InternedString, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, InternedString, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
) -> Class
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
    )
}

#[allow(unused_variables)]
pub fn __action54<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, InternedString, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, InternedString, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, ::std::vec::Vec<Member>, usize),
    __6: (usize, &'input str, usize),
) -> Class
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action19(
        input,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
    )
}

#[allow(unused_variables)]
pub fn __action55<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, InternedString, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, &'input str, usize),
) -> Class
{
    let __start0 = __2.2.clone();
    let __end0 = __3.0.clone();
    let __temp0 = __action18(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
pub fn __action56<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, InternedString, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<Member>, usize),
    __4: (usize, &'input str, usize),
) -> Class
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action19(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        input,
        __0,
        __1,
        __2,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
pub fn __action57<
    'input,
>(
    input: &'input str,
    __0: (usize, InternedString, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> Type
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action12(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
pub fn __action58<
    'input,
>(
    input: &'input str,
    __0: (usize, InternedString, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<Type>, usize),
    __3: (usize, &'input str, usize),
) -> Type
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action13(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
