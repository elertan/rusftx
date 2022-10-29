use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub id: String,
    pub name: String,
    pub collateral: bool,
    pub usd_fungible: bool,
    pub is_etf: Option<bool>,
    pub is_token: bool,
    pub hidden: Option<bool>,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub can_convert: bool,
    pub has_tag: bool,
    pub collateral_weight: Decimal,
    pub initial_collateral_weight: Option<Decimal>,
    pub imf_weight: Option<Decimal>,
    pub mmf_weight: Option<Decimal>,
    pub spot_margin_imf: Option<Decimal>,
    pub spot_margin_imf_factor: Option<Decimal>,
    pub fiat: bool,
    pub methods: Vec<Method>,
    pub erc_20_contract: Option<String>,
    pub bep_2_asset: Option<String>,
    pub trc_20_contract: Option<String>,
    pub spl_mint: Option<String>,
    pub credit_to: Option<String>,
    pub spot_margin: Option<bool>,
    pub nft_quote_currency_eligible: Option<bool>,
    pub index_price: Option<Decimal>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(from = "String")]
pub enum Method {
    #[serde(rename = "erc20")]
    ERC20,
    #[serde(rename = "trx")]
    TRC20,
    #[serde(rename = "sol")]
    SPL,
    #[serde(rename = "omni")]
    Omni,
    #[serde(rename = "bep2")]
    BEP2,
    #[serde(rename = "bsc")]
    BinanceSmartChain,
    #[serde(rename = "ftm")]
    Fantom,
    #[serde(rename = "avax")]
    Avax,
    #[serde(rename = "matic")]
    Matic,
    // #[serde(rename = "algo")]
    // Algorand,
    // #[serde(rename = "btc")]
    // Bitcoin,
    // #[serde(rename = "eth")]
    // Ethereum,
    // #[serde(rename = "arbitrum")]
    // Arbitrum,
    // #[serde(rename = "bch")]
    // BitcoinCash,
    // #[serde(rename = "ltc")]
    // Litecoin,
    // #[serde(rename = "xrp")]
    // XRP,
    // #[serde(rename = "heco")]
    // Heco,
    // #[serde(rename = "mob")]
    // Mobius,
    // #[serde(rename = "doge")]
    // Dogecoin,
    Other(String),
}

impl From<String> for Method {
    fn from(s: String) -> Self {
        match s.as_str() {
            "erc20" => Self::ERC20,
            "trx" => Self::TRC20,
            "sol" => Self::SPL,
            "omni" => Self::Omni,
            "bep2" => Self::BEP2,
            "bsc" => Self::BinanceSmartChain,
            "ftm" => Self::Fantom,
            "avax" => Self::Avax,
            "matic" => Self::Matic,
            // "algo" => Self::Algorand,
            // "btc" => Self::Bitcoin,
            // "eth" => Self::Ethereum,
            // "arbitrum" => Self::Arbitrum,
            // "bch" => Self::BitcoinCash,
            // "ltc" => Self::Litecoin,
            // "xrp" => Self::XRP,
            // "heco" => Self::Heco,
            // "mob" => Self::Mobius,
            // "doge" => Self::Dogecoin,
            _ => Self::Other(s),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rest::model::coin::Coin;

    #[test]
    fn deserialize_from_json() {
        let json = include_str!("../../../tests/data/coins.json");
        serde_json::from_str::<Vec<Coin>>(json).unwrap();
    }
}
