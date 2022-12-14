# RusFTX

## FTX has gone bankrupt right after publishing RusFTX, and therefore RusFTX sadly serves no real purpose anymore.

### Rust bindings for the FTX REST and Websocket API

[//]: # ([![Build Status]&#40;https://travis-ci.com/elertan/rusftx.svg?branch=master&#41;]&#40;https://travis-ci.com/rusftx/rusftx&#41;)
[![Crates.io](https://img.shields.io/crates/v/rusftx.svg)](https://crates.io/crates/rusftx)
[![Docs.rs](https://docs.rs/rusftx/badge.svg)](https://docs.rs/rusftx)
![License](https://img.shields.io/badge/license-MIT-blue)

[FTX API Documentation](https://docs.ftx.com/#overview)

Not all APIs are implemented yet. If you need a specific API, please open an issue, or consider contributing by creating a pull request.

RusFTX has taken inspiration from the existing [ftx crate](https://github.com/fabianboesiger/ftx) by Fabian Boesiger.

- All bindings follow the naming convention of the FTX API documentation whenever possible, making it easy to find the corresponding function.
- Calls to REST APIs that require authentication are compile-time checked 

### Authentication
RusFTX supports both API key and subaccount authentication, as well as having the ability to provide a custom endpoint (which could be used for proxies for example).
RusFTX has implemented endpoints for both FTX (.com) and FTX-US (.us) which can be configured using `EndpointCom` and `EndpointUs`.
```rust
let api_key = "YOUR_FTX_API_KEY";
let secret = "YOUR_FTX_SECRET";

let rest_api = RestApiWithAuthenticationBuilder::new()
    .endpoint(EndpointCom)
    .authentication(api_key, secret)
    .build();
```

You can also use the Rustftx without authentication for endpoints that support it.
```rust
let rest_api = RestApi::new(EndpointCom);
```

### Pagination
Pagination can be performed using the `start_time` and `end_time` parameters on all endpoints that support it by providing a chrono `DateTime<Utc>`.

### Implemented REST API bindings

- [x] Authentication
- [x] Rate limits
- [x] Pagination
- [x] Subaccounts
  - [x] Get all subaccounts
  - [x] Create subaccount
  - [x] Change subaccount name
  - [x] Delete subaccount
  - [x] Get subaccount balances
  - [x] Transfer between subaccounts
- [x] Markets
  - [x] Get markets
  - [x] Get single market
  - [x] Get orderbook
  - [x] Get trades
  - [x] Get historical prices
- [x] Futures
  - [x] List all futures
  - [x] Get future
  - [x] Get future stats
  - [x] Get funding rates
  - [x] Get index weights
  - [x] Get expired futures
  - [x] Get historical index
  - [x] Get index constituents
- [x] Account
  - [x] Get account information
  - [x] Request historical balances and positions snapshot
  - [x] Get historical balances and positions snapshot
  - [x] Get all historical balances and positions snapshots
  - [x] Get positions
  - [x] Change account leverage
- [ ] Wallet
  - [x] Get coins
  - [x] Get balances
  - [x] Get balances of all accounts
  - [x] Get deposit address
  - [x] Get deposit address list
  - [x] Get deposit history
  - [x] Get withdrawal history
  - [x] Request withdrawal
  - [x] Get airdrops
  - [x] Get withdrawal fees
  - [x] Get saved addresses
  - [x] Create saved addresses
  - [x] Delete saved addresses
  - [ ] Register a SEN deposit
  - [ ] Request a SEN withdrawal
  - [ ] Register a Signet deposit
  - [ ] Request a Signet withdrawal
- [ ] Orders
  - [x] Get open orders
  - [x] Get order history
  - [x] Get open trigger orders
  - [x] Get trigger order triggers
  - [x] Get trigger order history
  - [ ] Get TWAP orders
  - [ ] Get TWAP order executions
  - [x] Place order
  - [x] Place trigger order
  - [ ] Place TWAP order
  - [ ] Modify order
  - [ ] Modify order by client ID
  - [ ] Modify trigger order
  - [ ] Get order status
  - [ ] Get order status by client ID
  - [x] Cancel order
  - [ ] Cancel TWAP order
  - [ ] Cancel order by client ID
  - [x] Cancel open trigger order
  - [x] Cancel all orders
  - [ ] Bulk cancel orders
  - [ ] Bulk cancel orders by client id
  - [x] Get fills
- [ ] Funding Payments
- [ ] Leveraged Tokens
  - [ ] List leveraged tokens
  - [ ] Get token info
  - [ ] Get leveraged token balances
  - [ ] List leveraged token creation requests
  - [ ] Request leveraged token creation
  - [ ] List leveraged token redemption requests
  - [ ] Request leveraged token redemption
  - [ ] Request ETF rebalance info
- [ ] Options
  - [ ] List quote requests
  - [ ] Your quote requests
  - [ ] Create quote request
  - [ ] Cancel quote request
  - [ ] Get quotes for your quote request
  - [ ] Create quote
  - [ ] Get my quotes
  - [ ] Cancel quote
  - [ ] Accept options quote
  - [ ] Get account options info
  - [ ] Get options positions
  - [ ] Get public options trades
  - [ ] Get options fills
  - [ ] Get 24h option volume
  - [ ] Get option historical volumes
  - [ ] Get option open interest
  - [ ] Get historical option open interest
- [ ] Staking
  - [ ] Get stakes
  - [ ] Unstake request
  - [ ] Get stake balances
  - [ ] Cancel unstake request
  - [ ] Get staking rewards
  - [ ] Stake request
- [ ] Convert
  - [ ] Request quote
  - [ ] Get quote status
  - [ ] Accept quote
- [ ] Spot Margin
  - [ ] Get lending history
  - [ ] Get borrow rates
  - [ ] Get lending rates
  - [ ] Get daily borrowed amounts
  - [ ] Get market info
  - [ ] Get my borrow history
  - [ ] Get my lending history
  - [ ] Get lending offers
  - [ ] Get lending info
  - [ ] Submit lending offer
- [ ] NFTs
  - [ ] List NFTs
  - [ ] Get NFT info
  - [ ] Get NFT trades
  - [ ] Get all NFT trades
  - [ ] Get NFT account info
  - [ ] Get all NFT collections
  - [ ] Get NFT balances
  - [ ] Make NFT offer
  - [ ] Buy NFT
  - [ ] Create Auction
  - [ ] Edit Auction
  - [ ] Cancel Auction
  - [ ] Get bids
  - [ ] Place bid
  - [ ] Get NFT deposits
  - [ ] Get NFT withdrawals
  - [ ] Get NFT fills
  - [ ] Redeem NFT
  - [ ] Get NFT gallery
  - [ ] Get gallery settings
  - [ ] Edit gallery settings
- [ ] Latency statistics
- [ ] Optimized access
  - [ ] Add IP
  - [ ] Get IPs
  - [ ] Enable IP
  - [ ] Disable IP
  - [ ] Delete IP
- [ ] Support tickets
  - [ ] Get all support tickets
  - [ ] Get support ticket messages
  - [ ] Send support ticket messages
  - [ ] Send a support message
  - [ ] Update the status of your support ticket
  - [ ] Count total number of unread support messages
  - [ ] Mark support messages read

### Implemented Websocket API bindings
- [ ] Public Channels
  - [x] Ticker Channel
  - [x] Markets Channel
  - [x] Trades Channel
  - [ ] Orderbook Channel
  - [ ] Global Orderbook Channel
- [x] Private Channels
  - [x] Authentication
  - [x] Fills Channel
  - [x] Orders Channel
