use super::{Id, Identify};

use std::fmt::Display;

use crate::error::Q3Error;
use crate::expand::Expand;
use crate::parser::Q3Ast;
use crate::{parse_query, Q3Components, QStore};

/// Represents a query
#[derive(Debug, Clone, PartialEq)]
pub enum Query {
    /// A query that as not been expanded yet
    Raw {
        id: Id,
        query: String,
        tokens: Vec<Q3Ast>,
    },
    /// A query that as been expanded. All nested queries have been expanded.
    Expanded {
        id: Id,
        tokens: Vec<Q3Ast>,
        query: String,
    },
}

impl Query {
    pub fn new<S: Into<String>>(id: S, query: S) -> Result<Self, Q3Error> {
        let query: String = query.into();
        let id: Id = Id(id.into());
        let query_components = parse_query(query.as_ref())?;

        let sub_queries = query_components
            .iter()
            .flat_map(|component| match component {
                Q3Ast::Id(_id) => Some(()),
                _ => None,
            })
            .collect::<Vec<()>>();

        if sub_queries.is_empty() {
            Ok(Self::Expanded {
                id,
                query,
                tokens: query_components,
            })
        } else {
            Ok(Self::Raw {
                id,
                query,
                tokens: query_components,
            })
        }
    }
}

impl Identify for Query {
    fn get_id(&self) -> &Id {
        match self {
            Self::Raw { id, .. } => id,
            Self::Expanded { id, .. } => id,
        }
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expanded { tokens, .. } => {
                let result: String = tokens.iter().map(|token| token.to_string()).collect();

                write!(f, "{}", result)
            }
            Self::Raw { query, .. } => {
                write!(f, "{}", query)
            }
        }
    }
}

impl Expand for Query {
    type State = QStore;

    fn expand(&mut self, mut state: Self::State) -> Result<Self::State, Q3Error> {
        match self {
            Query::Raw {
                id, ref mut tokens, ..
            } => {
                state.set_failed_expansion(id.clone())?;
                state = tokens.expand(state)?;

                if tokens.iter().all(|token| matches!(token, Q3Ast::Other(_))) {
                    state.remove_failed_expansion(id);

                    *self = Query::Expanded {
                        id: id.to_owned(),
                        tokens: tokens.to_vec(),
                        query: tokens
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<String>>()
                            .join(""),
                    }
                }
            }
            _ => state.remove_failed_expansion(self.get_id()),
        }

        Ok(state)
    }
}

impl Expand for Vec<Q3Ast> {
    type State = QStore;

    fn expand(&mut self, mut state: Self::State) -> Result<Self::State, Q3Error> {
        for token in self.iter_mut() {
            match token {
                Q3Ast::Other(_) => (),
                Q3Ast::Id(id) => match state.get(id.to_string()) {
                    Some(Q3Components::Query(expanded_query @ Query::Expanded { .. })) => {
                        *token = Q3Ast::Other(expanded_query.to_string())
                    }
                    Some(Q3Components::Query(raw_query @ Query::Raw { .. })) => {
                        state = raw_query.clone().expand(state)?;
                    }
                    Some(Q3Components::List(mut list)) => {
                        list.expand(state.clone())?;

                        *token = Q3Ast::Other(list.to_string())
                    }
                    Some(Q3Components::Generator(mut generator)) => {
                        generator.expand(state.clone())?;
                        *token = Q3Ast::Other(generator.to_string())
                    }
                    _ => return Err(Q3Error::IdNotFound(id.to_string())),
                },
            }
        }

        Ok(state)
    }
}

#[test]
fn test_tokens_expansion() {
    let mut store = QStore::new();

    let q1 = Query::new("q1", "lorem ipsum").unwrap();
    let q2 = Query::new("q2", "dolor #{q1} #{q1}").unwrap();

    store.insert(Q3Components::Query(q1));
    store.insert(Q3Components::Query(q2));

    match store.get("q2") {
        Some(Q3Components::Query(Query::Raw { ref mut tokens, .. })) => {
            tokens.expand(store).unwrap();

            assert_eq!(
                tokens.to_vec(),
                vec![
                    Q3Ast::Other("dolor ".into()),
                    Q3Ast::Other("lorem ipsum".into()),
                    Q3Ast::Other(" ".into()),
                    Q3Ast::Other("lorem ipsum".into()),
                ]
            )
        }
        _ => (),
    }
}

#[test]
fn test_query_expansion() {
    let mut store = QStore::new();

    let q1 = Query::new("q1", "lorem ipsum").unwrap();
    let q2 = Query::new("q2", "dolor #{q1}").unwrap();

    store.insert(Q3Components::Query(q1));
    store.insert(Q3Components::Query(q2));

    let mut expected_store = QStore::new();

    let expected_q1 = Query::new("q1", "lorem ipsum").unwrap();
    let expected_q2 = Query::new("q2", "dolor lorem ipsum").unwrap();

    expected_store.insert(Q3Components::Query(expected_q1));
    expected_store.insert(Q3Components::Query(expected_q2));

    assert_eq!(
        store.expand().unwrap().get("q2").unwrap().to_string(),
        expected_store.get("q2").unwrap().to_string()
    )
}

#[test]
fn test_query_multi_level() {
    let mut store = QStore::new();

    let q1 = Query::new("q1", "q1").unwrap();
    let q2 = Query::new("q2", "q2 #{q1}").unwrap();
    let q3 = Query::new("q3", "q3 #{q1} #{q2}").unwrap();

    store.insert(Q3Components::Query(q1));
    store.insert(Q3Components::Query(q2));
    store.insert(Q3Components::Query(q3));

    let mut expected_store = QStore::new();

    let expected_q1 = Query::new("q1", "q1").unwrap();
    let expected_q2 = Query::new("q2", "q2 q1").unwrap();
    let expected_q3 = Query::new("q3", "q3 q1 q2 q1").unwrap();

    expected_store.insert(Q3Components::Query(expected_q1));
    expected_store.insert(Q3Components::Query(expected_q2));
    expected_store.insert(Q3Components::Query(expected_q3));

    assert_eq!(
        store.expand().unwrap().get("q2").unwrap().to_string(),
        expected_store.get("q2").unwrap().to_string()
    );

    assert_eq!(
        store.expand().unwrap().get("q3").unwrap().to_string(),
        expected_store.get("q3").unwrap().to_string()
    )
}

#[test]
fn test_query_expansion_cycle() {
    let mut store = QStore::new();

    let q1 = Query::new("q1", "lorem #{q2} ipsum").unwrap();
    let q2 = Query::new("q2", "dolor #{q1}").unwrap();

    store.insert(Q3Components::Query(q1));
    store.insert(Q3Components::Query(q2));

    assert!(store.expand().is_err())
}
