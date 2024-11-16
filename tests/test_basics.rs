use serde_json::json;
use near_sdk::json_types::U64;
use anyhow::Result;
use chrono::Utc;

#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
    let contract_wasm = near_workspaces::compile_project("./").await?;

    test_setup_auction(&contract_wasm).await?;
    Ok(())
}

async fn test_setup_auction(contract_wasm: &[u8]) -> Result<(), Box<dyn std::error::Error>> {

    let sandbox = near_workspaces::sandbox().await?;
    let contract = sandbox.dev_deploy(contract_wasm).await?;

    let (owner, owner_skey) = sandbox.dev_generate().await;
    let user = sandbox.dev_create_account().await?;

    let now = Utc::now().timestamp();
    let minute_from_now = (now + 600) * 1_000_000_000;

    let outcome = contract
        .call("init")
        .args_json(
            json!({
                "end_time": U64::from(123),
                "auctioneer": owner,
            })
        )
        .transact()
        .await?;

    assert!(outcome.is_success());

    Ok(())
}


// async fn test_bid_on_auction() -> Result<()> {

//     Ok(())
// }

// async fn test_basics_on(contract_wasm: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
//     let sandbox = near_workspaces::sandbox().await?;
//     let contract = sandbox.dev_deploy(contract_wasm).await?;

//     let user_account = sandbox.dev_create_account().await?;

//     let outcome = user_account
//         .call(contract.id(), "set_greeting")
//         .args_json(json!({"greeting": "Hello World!"}))
//         .transact()
//         .await?;
//     assert!(outcome.is_success());

//     let user_message_outcome = contract.view("get_greeting").args_json(json!({})).await?;
//     assert_eq!(user_message_outcome.json::<String>()?, "Hello World!");

//     Ok(())
// }


