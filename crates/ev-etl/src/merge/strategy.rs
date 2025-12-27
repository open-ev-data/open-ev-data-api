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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deep_merge_simple() {
        let base = json!({"a": 1, "b": 2});
        let overlay = json!({"b": 3, "c": 4});
        let result = deep_merge(&base, &overlay);
        assert_eq!(result, json!({"a": 1, "b": 3, "c": 4}));
    }

    #[test]
    fn test_deep_merge_nested() {
        let base = json!({
            "battery": {
                "capacity": 60,
                "chemistry": "nmc"
            }
        });
        let overlay = json!({
            "battery": {
                "capacity": 75
            }
        });
        let result = deep_merge(&base, &overlay);
        assert_eq!(
            result,
            json!({
                "battery": {
                    "capacity": 75,
                    "chemistry": "nmc"
                }
            })
        );
    }

    #[test]
    fn test_deep_merge_array_replacement() {
        let base = json!({"ports": ["ccs1", "ccs2"]});
        let overlay = json!({"ports": ["nacs"]});
        let result = deep_merge(&base, &overlay);
        assert_eq!(result, json!({"ports": ["nacs"]}));
    }

    #[test]
    fn test_merge_arrays_replace() {
        let base = vec![json!("a"), json!("b")];
        let overlay = vec![json!("c")];
        let result = merge_arrays_replace(&base, &overlay);
        assert_eq!(result, vec![json!("c")]);
    }

    #[test]
    fn test_merge_arrays_replace_empty_overlay() {
        let base = vec![json!("a"), json!("b")];
        let overlay: Vec<Value> = vec![];
        let result = merge_arrays_replace(&base, &overlay);
        assert!(result.is_empty());
    }

    #[test]
    fn test_remove_null_values_simple() {
        let mut value = json!({"a": 1, "b": null, "c": 3});
        remove_null_values(&mut value);
        assert_eq!(value, json!({"a": 1, "c": 3}));
    }

    #[test]
    fn test_remove_null_values_nested() {
        let mut value = json!({
            "a": 1,
            "nested": {
                "b": null,
                "c": 2
            }
        });
        remove_null_values(&mut value);
        assert_eq!(value, json!({"a": 1, "nested": {"c": 2}}));
    }

    #[test]
    fn test_remove_null_values_array() {
        let mut value = json!([{"a": null}, {"b": 1}]);
        remove_null_values(&mut value);
        assert_eq!(value, json!([{}, {"b": 1}]));
    }

    #[test]
    fn test_remove_null_values_no_nulls() {
        let mut value = json!({"a": 1, "b": 2});
        remove_null_values(&mut value);
        assert_eq!(value, json!({"a": 1, "b": 2}));
    }

    #[test]
    fn test_remove_null_values_scalar() {
        let mut value = json!(42);
        remove_null_values(&mut value);
        assert_eq!(value, json!(42));
    }

    #[test]
    fn test_deep_merge_scalar_override() {
        let base = json!({"value": 1});
        let overlay = json!({"value": "string"});
        let result = deep_merge(&base, &overlay);
        assert_eq!(result["value"], "string");
    }

    #[test]
    fn test_deep_merge_non_object_base() {
        let base = json!("scalar");
        let overlay = json!({"a": 1});
        let result = deep_merge(&base, &overlay);
        assert_eq!(result, json!({"a": 1}));
    }
}
