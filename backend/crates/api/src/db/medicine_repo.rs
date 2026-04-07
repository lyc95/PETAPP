use std::collections::HashMap;

use aws_sdk_dynamodb::{types::AttributeValue, Client};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::medicine_reminder::{MedicineReminder, UpdateMedicineReminderRequest},
};

// ---------------------------------------------------------------------------
// Public repo functions
// ---------------------------------------------------------------------------

pub async fn create(
    client: &Client,
    table: &str,
    reminder: &MedicineReminder,
) -> Result<(), AppError> {
    client
        .put_item()
        .table_name(table)
        .set_item(Some(to_item(reminder)))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(())
}

pub async fn find_by_id(
    client: &Client,
    table: &str,
    id: &Uuid,
) -> Result<Option<MedicineReminder>, AppError> {
    let result = client
        .get_item()
        .table_name(table)
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    result.item.map(from_item).transpose()
}

pub async fn list_by_cat(
    client: &Client,
    table: &str,
    cat_id: &Uuid,
    owner_id: &str,
) -> Result<Vec<MedicineReminder>, AppError> {
    let result = client
        .query()
        .table_name(table)
        .index_name("catId-index")
        .key_condition_expression("#cid = :cid")
        .filter_expression("#oid = :oid")
        .expression_attribute_names("#cid", "catId")
        .expression_attribute_names("#oid", "ownerId")
        .expression_attribute_values(":cid", AttributeValue::S(cat_id.to_string()))
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
    req: &UpdateMedicineReminderRequest,
) -> Result<MedicineReminder, AppError> {
    let existing = find_by_id(client, table, id)
        .await?
        .ok_or_else(|| AppError::NotFound("medicine reminder not found".to_string()))?;

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

    if let Some(v) = &req.reminder_type {
        set_parts.push("#reminderType = :reminderType".to_string());
        expr_names.insert("#reminderType".to_string(), "reminderType".to_string());
        expr_values.insert(":reminderType".to_string(), AttributeValue::S(v.clone()));
    }
    if let Some(v) = &req.label {
        set_parts.push("#label = :label".to_string());
        expr_names.insert("#label".to_string(), "label".to_string());
        expr_values.insert(":label".to_string(), AttributeValue::S(v.clone()));
    }
    if let Some(v) = &req.scheduled_date {
        set_parts.push("#scheduledDate = :scheduledDate".to_string());
        expr_names.insert("#scheduledDate".to_string(), "scheduledDate".to_string());
        expr_values.insert(":scheduledDate".to_string(), AttributeValue::S(v.clone()));
    }
    if let Some(v) = req.is_recurring {
        set_parts.push("#isRecurring = :isRecurring".to_string());
        expr_names.insert("#isRecurring".to_string(), "isRecurring".to_string());
        expr_values.insert(":isRecurring".to_string(), AttributeValue::Bool(v));
    }
    if let Some(v) = req.interval_days {
        set_parts.push("#intervalDays = :intervalDays".to_string());
        expr_names.insert("#intervalDays".to_string(), "intervalDays".to_string());
        expr_values.insert(
            ":intervalDays".to_string(),
            AttributeValue::N(v.to_string()),
        );
    }
    if let Some(v) = req.is_active {
        set_parts.push("#isActive = :isActive".to_string());
        expr_names.insert("#isActive".to_string(), "isActive".to_string());
        expr_values.insert(":isActive".to_string(), AttributeValue::Bool(v));
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
        .ok_or_else(|| AppError::NotFound("medicine reminder not found".to_string()))?;

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
// DynamoDB ↔ MedicineReminder conversion helpers
// ---------------------------------------------------------------------------

type Item = HashMap<String, AttributeValue>;

fn to_item(r: &MedicineReminder) -> Item {
    let mut item = HashMap::new();
    item.insert("id".to_owned(), AttributeValue::S(r.id.to_string()));
    item.insert("catId".to_owned(), AttributeValue::S(r.cat_id.to_string()));
    item.insert("ownerId".to_owned(), AttributeValue::S(r.owner_id.clone()));
    item.insert(
        "reminderType".to_owned(),
        AttributeValue::S(r.reminder_type.clone()),
    );
    item.insert("label".to_owned(), AttributeValue::S(r.label.clone()));
    item.insert(
        "scheduledDate".to_owned(),
        AttributeValue::S(r.scheduled_date.clone()),
    );
    item.insert(
        "isRecurring".to_owned(),
        AttributeValue::Bool(r.is_recurring),
    );
    if let Some(days) = r.interval_days {
        item.insert(
            "intervalDays".to_owned(),
            AttributeValue::N(days.to_string()),
        );
    }
    item.insert("isActive".to_owned(), AttributeValue::Bool(r.is_active));
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

fn from_item(item: Item) -> Result<MedicineReminder, AppError> {
    let id = Uuid::parse_str(&get_s(&item, "id")?).map_err(|e| AppError::Internal(e.into()))?;
    let cat_id =
        Uuid::parse_str(&get_s(&item, "catId")?).map_err(|e| AppError::Internal(e.into()))?;

    let interval_days = item
        .get("intervalDays")
        .and_then(|v| v.as_n().ok())
        .and_then(|s| s.parse::<u32>().ok());

    let is_recurring = item
        .get("isRecurring")
        .and_then(|v| v.as_bool().ok())
        .copied()
        .unwrap_or(false);

    let is_active = item
        .get("isActive")
        .and_then(|v| v.as_bool().ok())
        .copied()
        .unwrap_or(true);

    let created_at = get_datetime(&item, "createdAt")?;
    let updated_at = get_datetime(&item, "updatedAt")?;

    Ok(MedicineReminder {
        id,
        cat_id,
        owner_id: get_s(&item, "ownerId")?,
        reminder_type: get_s(&item, "reminderType")?,
        label: get_s(&item, "label")?,
        scheduled_date: get_s(&item, "scheduledDate")?,
        is_recurring,
        interval_days,
        is_active,
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

fn get_datetime(item: &Item, key: &str) -> Result<chrono::DateTime<chrono::Utc>, AppError> {
    let s = get_s(item, key)?;
    chrono::DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|e| AppError::Internal(e.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn sample() -> MedicineReminder {
        MedicineReminder {
            id: Uuid::new_v4(),
            cat_id: Uuid::new_v4(),
            owner_id: "user-123".to_string(),
            reminder_type: "MEDICATION".to_string(),
            label: "Flea Treatment".to_string(),
            scheduled_date: "2026-05-01T10:00:00Z".to_string(),
            is_recurring: true,
            interval_days: Some(30),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn round_trips_with_interval_days() {
        let r = sample();
        let recovered = from_item(to_item(&r)).unwrap();
        assert_eq!(r.id, recovered.id);
        assert_eq!(r.cat_id, recovered.cat_id);
        assert_eq!(r.reminder_type, recovered.reminder_type);
        assert_eq!(r.interval_days, recovered.interval_days);
        assert_eq!(r.is_recurring, recovered.is_recurring);
        assert_eq!(r.is_active, recovered.is_active);
    }

    #[test]
    fn round_trips_without_interval_days() {
        let mut r = sample();
        r.interval_days = None;
        r.is_recurring = false;
        let recovered = from_item(to_item(&r)).unwrap();
        assert!(recovered.interval_days.is_none());
        assert!(!recovered.is_recurring);
    }

    #[test]
    fn missing_required_field_returns_error() {
        let mut item = to_item(&sample());
        item.remove("label");
        assert!(from_item(item).is_err());
    }
}
