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
    pub methods: Vec<String>,
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
