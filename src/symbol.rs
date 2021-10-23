use crate::{contract::ContractStatus, filter::*};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Symbol {
    /// Symbol name
    pub name: String,
    /// Symbol alias
    pub alias: String,
    /// Symbol status
    pub status: ContractStatus,
    /// Base currency
    pub base_currency: String,
    /// Quote currency
    pub quote_currency: String,
    /// Price scale (number of decimal places)
    pub price_scale: i64,
    /// Taker fee
    pub taker_fee: String,
    /// Maker fee
    pub maker_fee: String,
    /// Leverage filter
    pub leverage_filter: LeverageFilter,
    /// Price filter
    pub price_filter: PriceFilter,
    /// Lot size filter
    pub lot_size_filter: LotSizeFilter,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct Symbols(Vec<Symbol>);

impl Symbols {
    /// Get the symbol with the given name.
    /// * `name` - The name of the symbol to find.
    pub fn get(&self, name: &str) -> Option<Symbol> {
        let pos = self.0.iter().position(|symbol| symbol.name == name)?;
        Some(self.0[pos].clone())
    }

    /// Returns an iterator over the symbols.
    pub fn symbols(&self) -> std::slice::Iter<'_, Symbol> {
        self.0.iter()
    }
}

impl Symbol {
    /// Create a symbol with the given name.
    /// * `name` - The name of the symbol.
    pub fn new(name: &str) -> Self {
        Symbol {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_symbol() {
        let symbols = Symbols(vec![Symbol::new("BTC"), Symbol::new("ETH")]);
        assert!(symbols.get("BTC").is_some());
        assert!(symbols.get("LTC").is_none());
        assert!(symbols.get("ETH").is_some());
    }
}
