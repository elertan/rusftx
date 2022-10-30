use rusftx::endpoint::EndpointCom;
use rusftx::rest::requests::create_subaccount::CreateSubaccountRequest;
use rusftx::rest::requests::get_account_information::GetAccountInformationRequest;
use rusftx::rest::requests::get_all_subaccounts::GetAllSubaccountsRequest;
use rusftx::rest::requests::get_subaccount_balances::GetSubaccountBalancesRequest;
use rusftx::rest::RestApiWithAuthenticationBuilder;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = std::env::var("FTX_API_KEY").expect("FTX_API_KEY is not set");
    let secret = std::env::var("FTX_SECRET").expect("FTX_SECRET is not set");

    println!("Starting...");
    let mut rest_api = RestApiWithAuthenticationBuilder::new()
        .endpoint(EndpointCom)
        .authentication(api_key, secret)
        .build();

    let subaccounts = match rest_api.send(GetAllSubaccountsRequest).await {
        Ok(x) => x,
        Err(err) => {
            panic!("Failed to get subaccounts: {:?}", err);
        }
    };
    let has_rusftx_subaccount = subaccounts
        .iter()
        .any(|subaccount| subaccount.nickname == "rusftx");
    if !has_rusftx_subaccount {
        println!("No rusftx subaccount found, creating one...");
        let request = CreateSubaccountRequest::new().nickname("rusftx").build();
        match rest_api.send(request).await {
            Ok(_) => {
                println!("Subaccount created");
            }
            Err(err) => {
                panic!("Failed to create subaccount: {:?}", err);
            }
        }
    } else {
        println!("rusftx subaccount exists, using it");
    }

    rest_api.change_subaccount(Some("rusftx".to_string()));
    let balances_request = GetSubaccountBalancesRequest::new()
        .nickname("rusftx")
        .build();
    let balances = match rest_api.send(balances_request).await {
        Ok(x) => x,
        Err(err) => {
            panic!("Failed to get balances: {:?}", err);
        }
    };
    println!("Balances: {:?}", balances);

    let account_info = match rest_api.send(GetAccountInformationRequest).await {
        Ok(x) => x,
        Err(err) => {
            panic!("Failed to get account info: {:?}", err);
        }
    };
    println!("Account info: {:?}", account_info);
}
