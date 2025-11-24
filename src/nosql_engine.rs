use mongodb::Client;
use mongodb::bson::{doc, Document};
use aws_sdk_dynamodb::{Client as DynamoClient, types::AttributeValue};
use aws_config;
use tokio::runtime::Runtime;

pub fn mongo_write(items: [String; 3]) -> Result<(), String> {
    let rt = Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(async {
        let client = Client::with_uri_str("mongodb://127.0.0.1:27017")
            .await
            .map_err(|e| e.to_string())?;
        let db = client.database("test_db");
        let coll = db.collection::<Document>("accounts");

        let filter0 = doc! { "username": &items[0] };
        let update0 = doc! { "$set": { "last_query": &items[0], "flag": "used_0" } };
        coll.find_one_and_update(filter0, update0).await.map_err(|e| e.to_string())?;

        let filter1 = doc! { "username": &items[1] };
        let update1 = doc! { "$set": { "last_query": &items[1], "flag": "tainted_used" } };
        //SINK
        coll.find_one_and_update(filter1, update1).await.map_err(|e| e.to_string())?;

        let filter2 = doc! { "username": &items[2] };
        let update2 = doc! { "$set": { "last_query": &items[2], "flag": "used_2" } };
        coll.find_one_and_update(filter2, update2).await.map_err(|e| e.to_string())?;

        Ok::<(), String>(())
    })?;
    Ok(())
}

pub fn dynamo_write(items: [String; 3]) -> Result<(), String> {
    let rt = Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(async {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let client = DynamoClient::new(&config);

        let preview0 = items[0].replace('"', "");
        client
            .put_item()
            .table_name("accounts")
            .item("pk", AttributeValue::S("static_pk".to_string()))
            .item("preview", AttributeValue::S(preview0))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let preview1 = items[1].replace('"', "");
        //SINK
        client
            .put_item()
            .table_name("accounts")
            .item("pk", AttributeValue::S("static_pk".to_string()))
            .item("preview", AttributeValue::S(preview1))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let preview2 = items[2].replace('"', "");
        client
            .put_item()
            .table_name("accounts")
            .item("pk", AttributeValue::S("static_pk".to_string()))
            .item("preview", AttributeValue::S(preview2))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok::<(), String>(())
    })?;
    Ok(())
}
