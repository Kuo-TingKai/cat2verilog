use std::collections::HashMap;

/// AST node representing a category theory statement
#[derive(Debug, Clone)]
pub enum Statement {
    /// Object declaration: object A
    Object(String),
    /// Morphism declaration: morphism f: A -> B
    Morphism { 
        name: String, 
        from: String, 
        to: String 
    },
    /// Commutativity assertion: assert commute: g ∘ f == h
    AssertCommute { 
        lhs: Vec<String>, 
        rhs: Vec<String> 
    },
}

/// Complete AST representing a category theory description
#[derive(Debug, Clone)]
pub struct CategoryAST {
    pub statements: Vec<Statement>,
}

impl CategoryAST {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    /// Get all object names from the AST
    pub fn get_objects(&self) -> Vec<&String> {
        self.statements
            .iter()
            .filter_map(|stmt| {
                if let Statement::Object(name) = stmt {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all morphism definitions as a map
    pub fn get_morphisms(&self) -> HashMap<&String, (&String, &String)> {
        self.statements
            .iter()
            .filter_map(|stmt| {
                if let Statement::Morphism { name, from, to } = stmt {
                    Some((name, (from, to)))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get all commutativity assertions
    pub fn get_commute_assertions(&self) -> Vec<(&Vec<String>, &Vec<String>)> {
        self.statements
            .iter()
            .filter_map(|stmt| {
                if let Statement::AssertCommute { lhs, rhs } = stmt {
                    Some((lhs, rhs))
                } else {
                    None
                }
            })
            .collect()
    }
} 