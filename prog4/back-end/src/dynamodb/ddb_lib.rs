use aws_sdk_dynamodb::*;
use std::collections::HashMap;

use aws_sdk_dynamodb::model::{AttributeValue, PutRequest, Select, WriteRequest};
use rocket::log::private::log;

static TABLE_NAME: &str = "prog4Tony";

pub async fn ddb_combine_maps(
    map: HashMap<String, HashMap<String, String>>,
) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let mut new_map = HashMap::new();
    for (key, value) in map.into_iter() {
        let mut inner_map = HashMap::new();
        let items = client
            .get_item()
            .table_name(TABLE_NAME)
            .key("FullName", AttributeValue::S(key.to_string()))
            .send()
            .await?;
        if items.item.is_some() {
            for (inner_key, inner_value) in items.item().unwrap().iter() {
                if inner_key.contains("FullName") {
                    continue;
                }
                inner_map.insert(inner_key.to_owned(), inner_value.as_s().unwrap().to_owned());
            }
        }
        inner_map.extend(value);
        new_map.insert(key.to_owned(), inner_map.to_owned());
    }
    Ok(new_map)
}

pub async fn ddb_upload(
    map: HashMap<String, HashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    for (key, value) in map.into_iter() {
        let mut item = HashMap::new();
        let split_name: Vec<&str> = key.as_str().split(' ').collect();
        item.insert("FullName".to_string(), AttributeValue::S(key.to_string()));
        item.insert(
            "Last-Name".to_string(),
            AttributeValue::S(split_name[0].to_string()),
        );
        item.insert(
            "First-Name".to_string(),
            AttributeValue::S(split_name[1].to_string()),
        );
        for (inner_key, inner_value) in value.into_iter() {
            item.insert(
                inner_key.to_string(),
                AttributeValue::S(inner_value.to_string()),
            );
        }
        client
            .batch_write_item()
            .request_items(
                TABLE_NAME,
                vec![WriteRequest::builder()
                    .put_request(PutRequest::builder().set_item(Some(item)).build())
                    .build()],
            )
            .send()
            .await?;
    }
    Ok(())
}

pub async fn ddb_query_last_name(
    query: &str,
) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let last_name_response = client
        .query()
        .table_name(TABLE_NAME)
        .index_name("Last-Name-index")
        .key_condition_expression("#key = :value".to_string())
        .expression_attribute_names("#key".to_string(), "Last-Name".to_string())
        .expression_attribute_values(":value".to_string(), AttributeValue::S(query.to_string()))
        .select(Select::AllAttributes)
        .send()
        .await?;

    let mut items: HashMap<String, HashMap<String, String>> = HashMap::new();

    if last_name_response.items().is_some() {
        for item in last_name_response.items.unwrap_or_default() {
            let mut inner_map: HashMap<String, String> = HashMap::new();
            for (inner_key, inner_value) in item.iter() {
                inner_map.insert(inner_key.to_owned(), inner_value.as_s().unwrap().to_owned());
            }
            inner_map.remove("First-Name");
            inner_map.remove("Last-Name");
            items.insert(inner_map.remove("FullName").unwrap_or_default(), inner_map);
        }
    }
    Ok(items)
}

pub async fn ddb_query_first_name(
    query: &str,
) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let mut items: HashMap<String, HashMap<String, String>> = HashMap::new();
    let first_name_response = client
        .query()
        .table_name(TABLE_NAME)
        .index_name("First-Name-index")
        .key_condition_expression("#key = :value".to_string())
        .expression_attribute_names("#key".to_string(), "First-Name".to_string())
        .expression_attribute_values(":value".to_string(), AttributeValue::S(query.to_string()))
        .select(Select::AllAttributes)
        .send()
        .await?;
    if first_name_response.items().is_some() {
        for item in first_name_response.items.unwrap_or_default() {
            let mut inner_map: HashMap<String, String> = HashMap::new();
            for (inner_key, inner_value) in item.iter() {
                inner_map.insert(inner_key.to_owned(), inner_value.as_s().unwrap().to_owned());
            }
            inner_map.remove("First-Name");
            inner_map.remove("Last-Name");
            items.insert(inner_map.remove("FullName").unwrap_or_default(), inner_map);
        }
    }
    Ok(items)
}

pub async fn ddb_query_full_name(
    query: &str,
) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let mut items: HashMap<String, HashMap<String, String>> = HashMap::new();
    let full_name_response = client
        .query()
        .table_name(TABLE_NAME)
        .key_condition_expression("#key = :value".to_string())
        .expression_attribute_names("#key".to_string(), "FullName".to_string())
        .expression_attribute_values(":value".to_string(), AttributeValue::S(query.to_string()))
        .select(Select::AllAttributes)
        .send()
        .await?;

    if full_name_response.items().is_some() {
        for item in full_name_response.items.unwrap_or_default() {
            let mut inner_map: HashMap<String, String> = HashMap::new();
            for (inner_key, inner_value) in item.iter() {
                inner_map.insert(inner_key.to_owned(), inner_value.as_s().unwrap().to_owned());
            }
            inner_map.remove("First-Name");
            inner_map.remove("Last-Name");
            items.insert(inner_map.remove("FullName").unwrap_or_default(), inner_map);
        }
    }
    Ok(items)
}

pub async fn ddb_clear() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let response = client.scan().table_name(TABLE_NAME).send().await?;
    if response.items().is_some() {
        for item in response.items.unwrap_or_default() {
            client
                .delete_item()
                .table_name(TABLE_NAME)
                .key("FullName", item["FullName"].clone())
                .send()
                .await?;
        }
    }
    Ok(())
}
