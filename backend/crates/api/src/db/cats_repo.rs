use std::collections::HashMap;

use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::cat::{Cat, UpdateCatRequest},
};

// ---------------------------------------------------------------------------
// Public repo functions
// ---------------------------------------------------------------------------

pub async fn create(client: &Client, table: &str, cat: &Cat) -> Result<(), AppError> {
    client
        .put_item()
        .table_name(table)
        .set_item(Some(to_item(cat)))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}

pub async fn find_by_id(client: &Client, table: &str, id: &Uuid) -> Result<Option<Cat>, AppError> {
    let result = client
        .get_item()
        .table_name(table)
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    result.item.map(from_item).transpose()
}

pub async fn list_by_owner(
    client: &Client,
    table: &str,
    owner_id: &str,
) -> Result<Vec<Cat>, AppError> {
    let result = client
        .query()
        .table_name(table)
        .index_name("ownerId-index")
        .key_condition_expression("#oid = :oid")
        .expression_attribute_names("#oid", "ownerId")
        .expression_attribute_values(":oid", AttributeValue::S(owner_id.to_owned()))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    result
        .items
        .unwrap_or_default()
        .into_iter()
        .map(from_item)
        .collect()
}

pub async fn update(
    client: &Client,
    table: &str,
    id: &Uuid,
    owner_id: &str,
    req: &UpdateCatRequest,
) -> Result<Cat, AppError> {
    // Verify existence and ownership before updating.
    let existing = find_by_id(client, table, id)
        .await?
        .ok_or_else(|| AppError::NotFound("cat not found".to_string()))?;

    if existing.owner_id != owner_id {
        return Err(AppError::Forbidden);
    }

    let now = Utc::now();
    let mut set_parts: Vec<String> = vec!["#updatedAt = :updatedAt".to_string()];
    let mut expr_names: HashMap<String, String> = HashMap::new();
    let mut expr_values: HashMap<String, AttributeValue> = HashMap::new();

    expr_names.insert("#updatedAt".to_string(), "updatedAt".to_string());
    expr_values.insert(":updatedAt".to_string(), AttributeValue::S(now.to_rfc3339()));

    if let Some(name) = &req.name {
        set_parts.push("#name = :name".to_string());
        expr_names.insert("#name".to_string(), "name".to_string());
        expr_values.insert(":name".to_string(), AttributeValue::S(name.clone()));
    }
    if let Some(breed) = &req.breed {
        set_parts.push("#breed = :breed".to_string());
        expr_names.insert("#breed".to_string(), "breed".to_string());
        expr_values.insert(":breed".to_string(), AttributeValue::S(breed.clone()));
    }
    if let Some(birthdate) = &req.birthdate {
        set_parts.push("#birthdate = :birthdate".to_string());
        expr_names.insert("#birthdate".to_string(), "birthdate".to_string());
        expr_values.insert(":birthdate".to_string(), AttributeValue::S(birthdate.clone()));
    }
    if let Some(photo_key) = &req.photo_key {
        set_parts.push("#photoKey = :photoKey".to_string());
        expr_names.insert("#photoKey".to_string(), "photoKey".to_string());
        expr_values.insert(":photoKey".to_string(), AttributeValue::S(photo_key.clone()));
    }

    let update_expr = format!("SET {}", set_parts.join(", "));

    let result = client
        .update_item()
        .table_name(table)
        .key("id", AttributeValue::S(id.to_string()))
        .update_expression(update_expr)
        .set_expression_attribute_names(Some(expr_names))
        .set_expression_attribute_values(Some(expr_values))
        .return_values(aws_sdk_dynamodb::types::ReturnValue::AllNew)
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    from_item(result.attributes.unwrap_or_default())
}

pub async fn delete(
    client: &Client,
    table: &str,
    id: &Uuid,
    owner_id: &str,
) -> Result<(), AppError> {
    let existing = find_by_id(client, table, id)
        .await?
        .ok_or_else(|| AppError::NotFound("cat not found".to_string()))?;

    if existing.owner_id != owner_id {
        return Err(AppError::Forbidden);
    }

    client
        .delete_item()
        .table_name(table)
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(())
}

// ---------------------------------------------------------------------------
// DynamoDB ↔ Cat conversion helpers
// ---------------------------------------------------------------------------

type Item = HashMap<String, AttributeValue>;

fn to_item(cat: &Cat) -> Item {
    let mut item = HashMap::new();
    item.insert("id".to_owned(), AttributeValue::S(cat.id.to_string()));
    item.insert("ownerId".to_owned(), AttributeValue::S(cat.owner_id.clone()));
    item.insert("name".to_owned(), AttributeValue::S(cat.name.clone()));
    item.insert("breed".to_owned(), AttributeValue::S(cat.breed.clone()));
    item.insert(
        "birthdate".to_owned(),
        AttributeValue::S(cat.birthdate.to_string()),
    );
    if let Some(key) = &cat.photo_key {
        item.insert("photoKey".to_owned(), AttributeValue::S(key.clone()));
    }
    item.insert(
        "createdAt".to_owned(),
        AttributeValue::S(cat.created_at.to_rfc3339()),
    );
    item.insert(
        "updatedAt".to_owned(),
        AttributeValue::S(cat.updated_at.to_rfc3339()),
    );
    item
}

fn from_item(item: Item) -> Result<Cat, AppError> {
    let id = Uuid::parse_str(&get_s(&item, "id")?)
        .map_err(|e| AppError::Internal(e.into()))?;

    let birthdate = NaiveDate::parse_from_str(&get_s(&item, "birthdate")?, "%Y-%m-%d")
        .map_err(|e| AppError::Internal(e.into()))?;

    let created_at = get_datetime(&item, "createdAt")?;
    let updated_at = get_datetime(&item, "updatedAt")?;

    Ok(Cat {
        id,
        owner_id: get_s(&item, "ownerId")?,
        name: get_s(&item, "name")?,
        breed: get_s(&item, "breed")?,
        birthdate,
        photo_key: get_s_opt(&item, "photoKey"),
        created_at,
        updated_at,
    })
}

fn get_s(item: &Item, key: &str) -> Result<String, AppError> {
    item.get(key)
        .and_then(|v| v.as_s().ok())
        .map(|s| s.to_owned())
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("DynamoDB: missing field `{key}`")))
}

fn get_s_opt(item: &Item, key: &str) -> Option<String> {
    item.get(key).and_then(|v| v.as_s().ok()).map(|s| s.to_owned())
}

fn get_datetime(
    item: &Item,
    key: &str,
) -> Result<chrono::DateTime<chrono::Utc>, AppError> {
    let s = get_s(item, key)?;
    chrono::DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|e| AppError::Internal(e.into()))
}
