use crate::types::trade::Trade;

#[derive(Debug, Clone, PartialEq)]
pub enum Transaction {
    Trade(Box<Trade>),
}