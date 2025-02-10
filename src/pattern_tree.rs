use crate::{simple_literal::SimpleLiteral, simple_pattern::SimplePattern};

#[derive(Debug, Clone, PartialEq)]
pub enum Leaf {
    Id(String),
    // gen [(::)]
    ArrayEclipse(String, usize),

    // gen if ==
    Literal(SimpleLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Array(Vec<PatternTree>),
    Object(Vec<(String, PatternTree)>, Option<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternTree {
    Leaf(Leaf),
    Node(Node),
}

impl PatternTree {
    pub fn from(pattern: SimplePattern) -> Self {
        match pattern {
            SimplePattern::Id(id) => PatternTree::Leaf(Leaf::Id(id)),
            SimplePattern::Literal(literal) => PatternTree::Leaf(Leaf::Literal(literal)),
            SimplePattern::Array(elems, eclipse) => {
                let len = elems.len();
                let mut elems: Vec<PatternTree> =
                    elems.into_iter().map(PatternTree::from).collect();
                if let Some(eclipse) = eclipse {
                    elems.push(PatternTree::Leaf(Leaf::ArrayEclipse(
                        eclipse,
                        len,
                    )));
                }
                PatternTree::Node(Node::Array(elems))
            }
            SimplePattern::Object(fields, eclipse) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, PatternTree::from(value)))
                    .collect();
                PatternTree::Node(Node::Object(fields, eclipse))
            }
        }
    }

    pub fn retrieve(self) -> Vec<Leaf> {
        match self {
            PatternTree::Leaf(leaf) => vec![leaf],
            PatternTree::Node(Node::Array(elems)) => {
                elems.into_iter().flat_map(PatternTree::retrieve).collect()
            }
            PatternTree::Node(Node::Object(fields, _)) => fields
                .into_iter()
                .flat_map(|(_, value)| value.retrieve())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simple_pattern::SimplePattern;

    #[test]
    fn test_retrieve() {
        let pattern = SimplePattern::Array(
            vec![
                SimplePattern::Id("a".to_string()),
                SimplePattern::Array(
                    vec![SimplePattern::Id("b".to_string())],
                    Some("c".to_string()),
                ),
            ],
            None,
        );
        let pattern_tree = PatternTree::from(pattern);
        let leaves = pattern_tree.retrieve();
        assert_eq!(leaves, vec![
            Leaf::Id("a".to_string()),
            Leaf::Id("b".to_string()),
            Leaf::ArrayEclipse("c".to_string(), 1),
        ]);
    }
}
