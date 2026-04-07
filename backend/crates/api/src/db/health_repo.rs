use std::collections::HashMap;

use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::health_record::{HealthRecord, UpdateHealthRecordRequest},
};

// ---------------------------------------------------------------------------
// Public repo functions
// ---------------------------------------------------------------------------

pub async fn create(client: &Client, table: &str, record: &HealthRecord) -> Result<(), AppError> {
    client
        .put_item()
        .table_name(table)
        .set_item(Some(to_item(record)))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}

pub async fn find_by_id(
    client: &Client,
    table: &str,
    id: &Uuid,
) -> Result<Option<HealthRecord>, AppError> {
    let result = client
        .get_item()
        .table_name(table)
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    result.item.map(from_item).transpose()
}

/// List health records for a cat, sorted by `recordedAt` descending (newest first).
pub async fn list_by_cat(
    client: &Client,
    table: &str,
    cat_id: &Uuid,
    owner_id: &str,
) -> Result<Vec<HealthRecord>, AppError> {
    let result = client
        .query()
        .table_name(table)
        .index_name("catId-recordedAt-index")
        .key_condition_expression("#cid = :cid")
        .filter_expression("#oid = :oid")
        .expression_attribute_names("#cid", "catId")
        .expression_attribute_names("#oid", "ownerId")
        .expression_attribute_values(":cid", AttributeValue::S(cat_id.to_string()))
        .expression_attribute_values(":oid", AttributeValue::S(owner_id.to_owned()))
        .scan_index_forward(false) // descending by recordedAt
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
    req: &UpdateHealthRecordRequest,
) -> Result<HealthRecord, AppError> {
    let existing = find_by_id(client, table, id)
        .await?
        .ok_or_else(|| AppError::NotFound("health record not found".to_string()))?;

    if existing.owner_id != owner_id {
        return Err(AppError::Forbidden);
    }

    let now = Utc::now();
    let mut set_parts: Vec<String> = vec!["#updatedAt = :updatedAt".to_string()];
    let mut expr_names: HashMap<String, String> = HashMap::new();
    let mut expr_values: HashMap<String, AttributeValue> = HashMap::new();

    expr_names.insert("#updatedAt".to_string(), "updatedAt".to_string());
    expr_values.insert(
        ":updatedAt".to_string(),
        AttributeValue::S(now.to_rfc3339()),
    );

    if let Some(v) = &req.record_type {
        set_parts.push("#recordType = :recordType".to_string());
        expr_names.insert("#recordType".to_string(), "recordType".to_string());
        expr_values.insert(":recordType".to_string(), AttributeValue::S(v.clone()));
    }
    if let Some(v) = &req.title {
        set_parts.push("#title = :title".to_string());
        expr_names.insert("#title".to_string(), "title".to_string());
        expr_values.insert(":title".to_string(), AttributeValue::S(v.clone()));
    }
    if let Some(v) = &req.description {
        set_parts.push("#description = :description".to_string());
        expr_names.insert("#description".to_string(), "description".to_string());
        expr_values.insert(":description".to_string(), AttributeValue::S(v.clone()));
    }
    if let Some(v) = &req.recorded_at {
        set_parts.push("#recordedAt = :recordedAt".to_string());
        expr_names.insert("#recordedAt".to_string(), "recordedAt".to_string());
        expr_values.insert(":recordedAt".to_string(), AttributeValue::S(v.clone()));
    }
    if let Some(v) = &req.attachment_key {
        set_parts.push("#attachmentKey = :attachmentKey".to_string());
        expr_names.insert("#attachmentKey".to_string(), "attachmentKey".to_string());
        expr_values.insert(":attachmentKey".to_string(), AttributeValue::S(v.clone()));
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
        .ok_or_else(|| AppError::NotFound("health record not found".to_string()))?;

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
// DynamoDB ↔ HealthRecord conversion helpers
// ---------------------------------------------------------------------------

type Item = HashMap<String, AttributeValue>;

fn to_item(r: &HealthRecord) -> Item {
    let mut item = HashMap::new();
    item.insert("id".to_owned(), AttributeValue::S(r.id.to_string()));
    item.insert("catId".to_owned(), AttributeValue::S(r.cat_id.to_string()));
    item.insert("ownerId".to_owned(), AttributeValue::S(r.owner_id.clone()));
    item.insert(
        "recordType".to_owned(),
        AttributeValue::S(r.record_type.clone()),
    );
    item.insert("title".to_owned(), AttributeValue::S(r.title.clone()));
    item.insert(
        "description".to_owned(),
        AttributeValue::S(r.description.clone()),
    );
    item.insert(
        "recordedAt".to_owned(),
        AttributeValue::S(r.recorded_at.to_rfc3339()),
    );
    if let Some(key) = &r.attachment_key {
        item.insert("attachmentKey".to_owned(), AttributeValue::S(key.clone()));
    }
    item.insert(
        "createdAt".to_owned(),
        AttributeValue::S(r.created_at.to_rfc3339()),
    );
    item.insert(
        "updatedAt".to_owned(),
        AttributeValue::S(r.updated_at.to_rfc3339()),
    );
    item
}

fn from_item(item: Item) -> Result<HealthRecord, AppError> {
    let id = Uuid::parse_str(&get_s(&item, "id")?).map_err(|e| AppError::Internal(e.into()))?;
    let cat_id =
        Uuid::parse_str(&get_s(&item, "catId")?).map_err(|e| AppError::Internal(e.into()))?;

    let recorded_at = get_datetime(&item, "recordedAt")?;
    let created_at = get_datetime(&item, "createdAt")?;
    let updated_at = get_datetime(&item, "updatedAt")?;

    let attachment_key = item
        .get("attachmentKey")
        .and_then(|v| v.as_s().ok())
        .map(|s| s.to_owned());

    Ok(HealthRecord {
        id,
        cat_id,
        owner_id: get_s(&item, "ownerId")?,
        record_type: get_s(&item, "recordType")?,
        title: get_s(&item, "title")?,
        description: get_s(&item, "description")?,
        recorded_at,
        attachment_key,
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

fn get_datetime(item: &Item, key: &str) -> Result<DateTime<Utc>, AppError> {
    let s = get_s(item, key)?;
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| AppError::Internal(e.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn sample() -> HealthRecord {
        HealthRecord {
            id: Uuid::new_v4(),
            cat_id: Uuid::new_v4(),
            owner_id: "user-123".to_string(),
            record_type: "VACCINATION".to_string(),
            title: "Annual Vaccination".to_string(),
            description: "FVRCP booster, all clear.".to_string(),
            recorded_at: Utc::now(),
            attachment_key: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn round_trips_without_attachment() {
        let r = sample();
        let recovered = from_item(to_item(&r)).unwrap();
        assert_eq!(r.id, recovered.id);
        assert_eq!(r.record_type, recovered.record_type);
        assert_eq!(r.title, recovered.title);
        assert!(recovered.attachment_key.is_none());
    }

    #[test]
    fn round_trips_with_attachment() {
        let mut r = sample();
        r.attachment_key = Some("attachments/uuid/doc.pdf".to_string());
        let recovered = from_item(to_item(&r)).unwrap();
        assert_eq!(recovered.attachment_key, r.attachment_key);
    }

    #[test]
    fn missing_title_returns_error() {
        let mut item = to_item(&sample());
        item.remove("title");
        assert!(from_item(item).is_err());
    }
}
