pub mod cancel_all_orders;
pub mod cancel_open_trigger_order;
pub mod cancel_order;
pub mod change_account_leverage;
pub mod change_subaccount_name;
pub mod create_saved_addresses;
pub mod create_subaccount;
pub mod delete_saved_address;
pub mod delete_subaccount;
pub mod get_account_information;
pub mod get_airdrops;
pub mod get_all_subaccounts;
pub mod get_balances;
pub mod get_balances_of_all_accounts;
pub mod get_coins;
pub mod get_deposit_address;
pub mod get_deposit_address_list;
pub mod get_deposit_history;
pub mod get_expired_futures;
pub mod get_fills;
pub mod get_funding_rates;
pub mod get_future;
pub mod get_future_stats;
pub mod get_historical_balances_and_positions_snapshot;
pub mod get_historical_index;
pub mod get_historical_prices;
pub mod get_index_constituents;
pub mod get_index_weights;
pub mod get_markets;
pub mod get_open_orders;
pub mod get_open_trigger_orders;
pub mod get_order_book;
pub mod get_order_history;
pub mod get_positions;
pub mod get_saved_addresses;
pub mod get_single_market;
pub mod get_subaccount_balances;
pub mod get_trades;
pub mod get_trigger_order_history;
pub mod get_trigger_order_triggers;
pub mod get_withdrawal_fees;
pub mod get_withdrawal_history;
pub mod list_all_futures;
pub mod place_order;
pub mod place_trigger_order;
pub mod request_historical_balances_and_positions_snapshot;
pub mod request_withdrawal;
pub mod test_utils;
pub mod transfer_between_subaccounts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum QueryOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}
