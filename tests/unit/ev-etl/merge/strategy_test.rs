use ev_etl::merge::deep_merge;
use serde_json::json;

#[test]
fn test_merge_simple_objects() {
    let base = json!({"a": 1, "b": 2});
    let overlay = json!({"b": 3, "c": 4});
    let result = deep_merge(&base, &overlay);

    assert_eq!(result["a"], 1);
    assert_eq!(result["b"], 3);
    assert_eq!(result["c"], 4);
}

#[test]
fn test_merge_nested_objects() {
    let base = json!({
        "battery": {
            "capacity": 60,
            "chemistry": "nmc"
        },
        "make": "Tesla"
    });

    let overlay = json!({
        "battery": {
            "capacity": 75
        }
    });

    let result = deep_merge(&base, &overlay);

    assert_eq!(result["battery"]["capacity"], 75);
    assert_eq!(result["battery"]["chemistry"], "nmc");
    assert_eq!(result["make"], "Tesla");
}

#[test]
fn test_merge_array_replacement() {
    let base = json!({
        "charge_ports": ["ccs1", "type2"]
    });

    let overlay = json!({
        "charge_ports": ["nacs"]
    });

    let result = deep_merge(&base, &overlay);

    assert_eq!(result["charge_ports"].as_array().unwrap().len(), 1);
    assert_eq!(result["charge_ports"][0], "nacs");
}

#[test]
fn test_merge_deeply_nested() {
    let base = json!({
        "charging": {
            "dc": {
                "max_power_kw": 150,
                "architecture_voltage_class": "400v"
            },
            "ac": {
                "max_power_kw": 11
            }
        }
    });

    let overlay = json!({
        "charging": {
            "dc": {
                "max_power_kw": 250
            }
        }
    });

    let result = deep_merge(&base, &overlay);

    assert_eq!(result["charging"]["dc"]["max_power_kw"], 250);
    assert_eq!(
        result["charging"]["dc"]["architecture_voltage_class"],
        "400v"
    );
    assert_eq!(result["charging"]["ac"]["max_power_kw"], 11);
}

#[test]
fn test_merge_variant_over_base() {
    let base = json!({
        "schema_version": "1.0.0",
        "make": {"slug": "tesla", "name": "Tesla"},
        "model": {"slug": "model_3", "name": "Model 3"},
        "year": 2024,
        "trim": {"slug": "base", "name": "Base"},
        "battery": {
            "pack_capacity_kwh_net": 60.0
        },
        "range": {
            "rated": [{"cycle": "wltp", "range_km": 513}]
        }
    });

    let variant = json!({
        "variant": {"slug": "long_range", "name": "Long Range"},
        "battery": {
            "pack_capacity_kwh_net": 82.0
        },
        "range": {
            "rated": [{"cycle": "wltp", "range_km": 678}]
        }
    });

    let result = deep_merge(&base, &variant);

    assert_eq!(result["make"]["slug"], "tesla");
    assert_eq!(result["variant"]["slug"], "long_range");
    assert_eq!(result["battery"]["pack_capacity_kwh_net"], 82.0);
    assert_eq!(result["range"]["rated"][0]["range_km"], 678);
}

#[test]
fn test_merge_preserves_scalar_types() {
    let base = json!({
        "year": 2024,
        "battery": {
            "heat_pump": false,
            "capacity_kwh": 60.0
        }
    });

    let overlay = json!({
        "battery": {
            "heat_pump": true
        }
    });

    let result = deep_merge(&base, &overlay);

    assert_eq!(result["year"], 2024);
    assert_eq!(result["battery"]["heat_pump"], true);
    assert_eq!(result["battery"]["capacity_kwh"], 60.0);
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

#[test]
fn test_merge_arrays_replace() {
    let base = vec![json!("a"), json!("b")];
    let overlay = vec![json!("c")];
    let result = ev_etl::merge::merge_arrays_replace(&base, &overlay);
    assert_eq!(result, vec![json!("c")]);
}

#[test]
fn test_merge_arrays_replace_empty_overlay() {
    let base = vec![json!("a"), json!("b")];
    let overlay: Vec<serde_json::Value> = vec![];
    let result = ev_etl::merge::merge_arrays_replace(&base, &overlay);
    assert!(result.is_empty());
}

#[test]
fn test_remove_null_values_simple() {
    let mut value = json!({"a": 1, "b": null, "c": 3});
    ev_etl::merge::remove_null_values(&mut value);
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
    ev_etl::merge::remove_null_values(&mut value);
    assert_eq!(value, json!({"a": 1, "nested": {"c": 2}}));
}

#[test]
fn test_remove_null_values_array() {
    let mut value = json!([{"a": null}, {"b": 1}]);
    ev_etl::merge::remove_null_values(&mut value);
    assert_eq!(value, json!([{}, {"b": 1}]));
}

#[test]
fn test_remove_null_values_no_nulls() {
    let mut value = json!({"a": 1, "b": 2});
    ev_etl::merge::remove_null_values(&mut value);
    assert_eq!(value, json!({"a": 1, "b": 2}));
}

#[test]
fn test_remove_null_values_scalar() {
    let mut value = json!(42);
    ev_etl::merge::remove_null_values(&mut value);
    assert_eq!(value, json!(42));
}
