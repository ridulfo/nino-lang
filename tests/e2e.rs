use nino::{
    lexer::tokenize,
    parser::{parse, Expression, Type},
    virtual_machine::VirtualMachine,
};

/// Testing a declaration of factorial function and calling it
#[test]
fn test_recursion() {
    let code = "let factorial:fn = (n:num):num => n ? {
    0 => 1,
    n * factorial(n - 1)
};
let result:num = factorial(5);";
    let tokens = tokenize(code);
    let ast = parse(&tokens).unwrap();

    let mut vm = VirtualMachine::new();

    vm.run(ast);

    let function = vm.symbols.get("factorial").unwrap();
    assert_eq!(function.name, "factorial");
    assert_eq!(function.type_, Type::Function);

    let result = *vm.symbols.get("result").unwrap().expression.clone();
    assert_eq!(result, Expression::Number(120.0));
}

/// Testing decalration of arrays and concatenation
#[test]
fn test_array_operations() {
    let declare = "let array:[num] = [1, 2, 3, 4, 5];
        let array2:[num] = [6, 7, 8, 9, 10];
        let array3:[num] = array + array2;";

    let tokens = tokenize(declare);
    let ast = parse(&tokens).unwrap();

    let mut vm = VirtualMachine::new();

    vm.run(ast);

    assert!(vm.symbols.get("array").is_some());
    assert_eq!(vm.symbols.get("array").unwrap().name, "array");
    assert_eq!(
        vm.symbols.get("array").unwrap().type_,
        Type::Array(Box::new(Type::Number))
    );

    assert!(vm.symbols.get("array2").is_some());
    assert_eq!(vm.symbols.get("array2").unwrap().name, "array2");
    assert_eq!(
        vm.symbols.get("array2").unwrap().type_,
        Type::Array(Box::new(Type::Number))
    );

    assert!(vm.symbols.get("array3").is_some());
    assert_eq!(vm.symbols.get("array3").unwrap().name, "array3");
    assert_eq!(
        vm.symbols.get("array3").unwrap().type_,
        Type::Array(Box::new(Type::Number))
    );

    let result = *vm.symbols.get("array3").unwrap().expression.clone();
    assert_eq!(
        result,
        Expression::Array(
            Type::Number,
            vec![
                Expression::Number(1.0),
                Expression::Number(2.0),
                Expression::Number(3.0),
                Expression::Number(4.0),
                Expression::Number(5.0),
                Expression::Number(6.0),
                Expression::Number(7.0),
                Expression::Number(8.0),
                Expression::Number(9.0),
                Expression::Number(10.0)
            ]
        )
    );
}

/// Testing declaration of a string and concatenation
#[test]
fn test_string() {
    let declare = "let string:[char] = \"Hello\" + \", World!\";";

    let tokens = tokenize(declare);

    let program = parse(&tokens).unwrap();

    let mut vm = VirtualMachine::new();
    vm.run(program);

    let result = *vm.symbols.get("string").unwrap().expression.clone();
    assert_eq!(
        result,
        Expression::Array(
            Type::Char,
            vec![
                Expression::Char(b'H'),
                Expression::Char(b'e'),
                Expression::Char(b'l'),
                Expression::Char(b'l'),
                Expression::Char(b'o'),
                Expression::Char(b','),
                Expression::Char(b' '),
                Expression::Char(b'W'),
                Expression::Char(b'o'),
                Expression::Char(b'r'),
                Expression::Char(b'l'),
                Expression::Char(b'd'),
                Expression::Char(b'!')
            ]
        )
    );
}

#[test]
fn test_string_equality() {
    let declare = "let string:[char] = \"Hello\" == \"Hello\";";

    let tokens = tokenize(declare);

    let program = parse(&tokens).unwrap();

    let mut vm = VirtualMachine::new();
    vm.run(program);

    let result = *vm.symbols.get("string").unwrap().expression.clone();
    assert_eq!(result, Expression::Bool(true));
}

#[test]
fn test_declare_string_as_array() {
    let declare = "let string1:[char] = \"Hello\";
let string2:[char] = ['H', 'e', 'l', 'l', 'o'];
let equality:bool = string1 == string2;";

    let tokens = tokenize(declare);

    let program = parse(&tokens).unwrap();

    let mut vm = VirtualMachine::new();
    vm.run(program);

    let result = *vm.symbols.get("string1").unwrap().expression.clone();
    assert_eq!(
        result,
        Expression::Array(
            Type::Char,
            vec![
                Expression::Char(b'H'),
                Expression::Char(b'e'),
                Expression::Char(b'l'),
                Expression::Char(b'l'),
                Expression::Char(b'o'),
            ]
        )
    );

    let result = *vm.symbols.get("string2").unwrap().expression.clone();
    assert_eq!(
        result,
        Expression::Array(
            Type::Char,
            vec![
                Expression::Char(b'H'),
                Expression::Char(b'e'),
                Expression::Char(b'l'),
                Expression::Char(b'l'),
                Expression::Char(b'o'),
            ]
        )
    );

    let result = *vm.symbols.get("equality").unwrap().expression.clone();
    assert_eq!(result, Expression::Bool(true));
}

/// Testing tail call optimization
#[test]
fn tail_optimization() {
    let declare = "let increment:fn = (x:num, i:num):num => i ? {
    0 => x,
    increment(x + 1, i - 1)
};
let incremented:num = increment(0, 20000);";
    let tokens = tokenize(declare);

    let program = parse(&tokens).unwrap();

    let mut vm = VirtualMachine::new();

    vm.run(program);

    let result = *vm.symbols.get("incremented").unwrap().expression.clone();
    assert_eq!(result, Expression::Number(20000.0));
}

/// Testing scope
#[test]
fn test_scope() {
    let declare = "let x:num = 0;
let func:fn = (x:num):num=>x+1;
func(1);
";

    let tokens = tokenize(declare);

    let program = parse(&tokens).unwrap();

    let mut vm = VirtualMachine::new();

    vm.run(program);

    let result = *vm.symbols.get("x").unwrap().expression.clone();
    assert_eq!(result, Expression::Number(0.0));
}

/// Testing operator precedence
#[test]
fn test_precedence() {
    let declare = "let x:num = 1 + 2 * 3;";

    let tokens = tokenize(declare);

    let program = parse(&tokens).unwrap();

    let mut vm = VirtualMachine::new();

    vm.run(program);

    let result = *vm.symbols.get("x").unwrap().expression.clone();
    assert_eq!(result, Expression::Number(7.0));

    let declare = "let x:num = (1 * 2) + 3;";

    let tokens = tokenize(declare);

    let program = parse(&tokens).unwrap();

    let mut vm = VirtualMachine::new();

    vm.run(program);

    let result = *vm.symbols.get("x").unwrap().expression.clone();
    assert_eq!(result, Expression::Number(5.0));
}
