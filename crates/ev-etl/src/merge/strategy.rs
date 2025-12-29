use serde_json::Value;

pub fn deep_merge(base: &Value, overlay: &Value) -> Value {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            let mut result = base_map.clone();

            for (key, overlay_value) in overlay_map {
                let merged_value = if let Some(base_value) = base_map.get(key) {
                    deep_merge(base_value, overlay_value)
                } else {
                    overlay_value.clone()
                };
                result.insert(key.clone(), merged_value);
            }

            Value::Object(result)
        }
        (_, overlay) => overlay.clone(),
    }
}

#[allow(dead_code)]
pub fn merge_arrays_replace(_base: &[Value], overlay: &[Value]) -> Vec<Value> {
    overlay.to_vec()
}

#[allow(dead_code)]
pub fn remove_null_values(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let keys_to_remove: Vec<String> = map
                .iter()
                .filter(|(_, v)| v.is_null())
                .map(|(k, _)| k.clone())
                .collect();

            for key in keys_to_remove {
                map.remove(&key);
            }

            for (_, v) in map.iter_mut() {
                remove_null_values(v);
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                remove_null_values(item);
            }
        }
        _ => {}
    }
}
