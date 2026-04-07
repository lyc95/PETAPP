use std::collections::HashMap;

use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::weight_log::{UpdateWeightLogRequest, WeightLog},
};

// ---------------------------------------------------------------------------
// Public repo functions
// ---------------------------------------------------------------------------

pub async fn create(client: &Client, table: &str, log: &WeightLog) -> Result<(), AppError> {
    client
        .put_item()
        .table_name(table)
        .set_item(Some(to_item(log)))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}

pub async fn find_by_id(
    client: &Client,
    table: &str,
    id: &Uuid,
) -> Result<Option<WeightLog>, AppError> {
    let result = client
        .get_item()
        .table_name(table)
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    result.item.map(from_item).transpose()
}

/// List weight logs for a cat, sorted by `loggedAt` descending (newest first).
pub async fn list_by_cat(
    client: &Client,
    table: &str,
    cat_id: &Uuid,
    owner_id: &str,
) -> Result<Vec<WeightLog>, AppError> {
    let result = client
        .query()
        .table_name(table)
        .index_name("catId-loggedAt-index")
        .key_condition_expression("#cid = :cid")
        .filter_expression("#oid = :oid")
        .expression_attribute_names("#cid", "catId")
        .expression_attribute_names("#oid", "ownerId")
        .expression_attribute_values(":cid", AttributeValue::S(cat_id.to_string()))
        .expression_attribute_values(":oid", AttributeValue::S(owner_id.to_owned()))
        .scan_index_forward(false) // descending by loggedAt
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
    req: &UpdateWeightLogRequest,
) -> Result<WeightLog, AppError> {
    let existing = find_by_id(client, table, id)
        .await?
        .ok_or_else(|| AppError::NotFound("weight log not found".to_string()))?;

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

    if let Some(kg) = req.weight_kg {
        set_parts.push("#weightKg = :weightKg".to_string());
        expr_names.insert("#weightKg".to_string(), "weightKg".to_string());
        expr_values.insert(":weightKg".to_string(), AttributeValue::N(kg.to_string()));
    }
    if let Some(la) = &req.logged_at {
        set_parts.push("#loggedAt = :loggedAt".to_string());
        expr_names.insert("#loggedAt".to_string(), "loggedAt".to_string());
        expr_values.insert(":loggedAt".to_string(), AttributeValue::S(la.clone()));
    }
    if let Some(note) = &req.note {
        set_parts.push("#note = :note".to_string());
        expr_names.insert("#note".to_string(), "note".to_string());
        expr_values.insert(":note".to_string(), AttributeValue::S(note.clone()));
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
        .ok_or_else(|| AppError::NotFound("weight log not found".to_string()))?;

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
// DynamoDB ↔ WeightLog conversion helpers
// ---------------------------------------------------------------------------

type Item = HashMap<String, AttributeValue>;

fn to_item(log: &WeightLog) -> Item {
    let mut item = HashMap::new();
    item.insert("id".to_owned(), AttributeValue::S(log.id.to_string()));
    item.insert(
        "catId".to_owned(),
        AttributeValue::S(log.cat_id.to_string()),
    );
    item.insert(
        "ownerId".to_owned(),
        AttributeValue::S(log.owner_id.clone()),
    );
    item.insert(
        "weightKg".to_owned(),
        AttributeValue::N(log.weight_kg.to_string()),
    );
    item.insert(
        "loggedAt".to_owned(),
        AttributeValue::S(log.logged_at.to_rfc3339()),
    );
    if let Some(note) = &log.note {
        item.insert("note".to_owned(), AttributeValue::S(note.clone()));
    }
    item.insert(
        "createdAt".to_owned(),
        AttributeValue::S(log.created_at.to_rfc3339()),
    );
    item.insert(
        "updatedAt".to_owned(),
        AttributeValue::S(log.updated_at.to_rfc3339()),
    );
    item
}

fn from_item(item: Item) -> Result<WeightLog, AppError> {
    let id = Uuid::parse_str(&get_s(&item, "id")?).map_err(|e| AppError::Internal(e.into()))?;
    let cat_id =
        Uuid::parse_str(&get_s(&item, "catId")?).map_err(|e| AppError::Internal(e.into()))?;

    let weight_kg = item
        .get("weightKg")
        .and_then(|v| v.as_n().ok())
        .and_then(|s| s.parse::<f64>().ok())
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("DynamoDB: missing field `weightKg`")))?;

    let logged_at = get_datetime(&item, "loggedAt")?;
    let created_at = get_datetime(&item, "createdAt")?;
    let updated_at = get_datetime(&item, "updatedAt")?;

    let note = item
        .get("note")
        .and_then(|v| v.as_s().ok())
        .map(|s| s.to_owned());

    Ok(WeightLog {
        id,
        cat_id,
        owner_id: get_s(&item, "ownerId")?,
        weight_kg,
        logged_at,
        note,
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

    fn sample() -> WeightLog {
        WeightLog {
            id: Uuid::new_v4(),
            cat_id: Uuid::new_v4(),
            owner_id: "user-123".to_string(),
            weight_kg: 4.2,
            logged_at: Utc::now(),
            note: Some("After breakfast".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn round_trips_with_note() {
        let log = sample();
        let recovered = from_item(to_item(&log)).unwrap();
        assert_eq!(log.id, recovered.id);
        assert_eq!(log.cat_id, recovered.cat_id);
        assert!((log.weight_kg - recovered.weight_kg).abs() < f64::EPSILON);
        assert_eq!(log.note, recovered.note);
    }

    #[test]
    fn round_trips_without_note() {
        let mut log = sample();
        log.note = None;
        let recovered = from_item(to_item(&log)).unwrap();
        assert!(recovered.note.is_none());
    }

    #[test]
    fn missing_weight_returns_error() {
        let mut item = to_item(&sample());
        item.remove("weightKg");
        assert!(from_item(item).is_err());
    }
}
