use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::Declaration;

#[derive(Clone)]
pub struct ScopedSymbols {
    symbols: HashMap<String, Declaration>,
    parent: Option<Rc<RefCell<ScopedSymbols>>>,
}

impl ScopedSymbols {
    pub fn new() -> Self {
        ScopedSymbols {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<ScopedSymbols>>) -> Self {
        ScopedSymbols {
            symbols: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn insert(&mut self, key: String, declaration: Declaration) {
        self.symbols.insert(key, declaration);
    }

    pub fn get(&self, key: &str) -> Option<Declaration> {
        match self.symbols.get(key) {
            Some(declaration) => Some(declaration.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(key),
                None => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Expression, Type};

    use super::*;

    #[test]
    fn test_scoped_symbols() {
        let mut symbols = ScopedSymbols::new();
        symbols.insert(
            "a".to_string(),
            Declaration {
                name: "a".to_string(),
                type_: Type::Number,
                expression: Box::new(Expression::Number(1.0)),
            },
        );

        let mut symbols2 = ScopedSymbols::with_parent(Rc::new(RefCell::new(symbols)));
        symbols2.insert(
            "b".to_string(),
            Declaration {
                name: "b".to_string(),
                type_: Type::Number,
                expression: Box::new(Expression::Number(2.0)),
            },
        );

        let a = symbols2.get("a").unwrap();
        assert_eq!(a.name, "a");
        assert_eq!(a.type_, Type::Number);
        assert_eq!(*a.expression, Expression::Number(1.0));

        let b = symbols2.get("b").unwrap();
        assert_eq!(b.name, "b");
        assert_eq!(b.type_, Type::Number);
        assert_eq!(*b.expression, Expression::Number(2.0));
    }
}
