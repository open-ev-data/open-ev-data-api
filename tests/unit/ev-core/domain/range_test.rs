use ev_core::{Efficiency, Range, RangeCycle, RangeRated, RangeRealWorld, RealWorldProfile};

#[test]
fn test_range_wltp_range_km() {
    let range = Range {
        rated: vec![
            RangeRated {
                cycle: RangeCycle::Wltp,
                range_km: 500.0,
                notes: None,
            },
            RangeRated {
                cycle: RangeCycle::Epa,
                range_km: 450.0,
                notes: None,
            },
        ],
        real_world: None,
    };

    assert_eq!(range.wltp_range_km(), Some(500.0));
}

#[test]
fn test_range_epa_range_km() {
    let range = Range {
        rated: vec![
            RangeRated {
                cycle: RangeCycle::Wltp,
                range_km: 500.0,
                notes: None,
            },
            RangeRated {
                cycle: RangeCycle::Epa,
                range_km: 450.0,
                notes: None,
            },
        ],
        real_world: None,
    };

    assert_eq!(range.epa_range_km(), Some(450.0));
}

#[test]
fn test_range_wltp_not_found() {
    let range = Range {
        rated: vec![RangeRated {
            cycle: RangeCycle::Epa,
            range_km: 450.0,
            notes: None,
        }],
        real_world: None,
    };

    assert_eq!(range.wltp_range_km(), None);
}

#[test]
fn test_range_epa_not_found() {
    let range = Range {
        rated: vec![RangeRated {
            cycle: RangeCycle::Wltp,
            range_km: 500.0,
            notes: None,
        }],
        real_world: None,
    };

    assert_eq!(range.epa_range_km(), None);
}

#[test]
fn test_range_empty_rated() {
    let range = Range {
        rated: vec![],
        real_world: None,
    };

    assert_eq!(range.wltp_range_km(), None);
    assert_eq!(range.epa_range_km(), None);
}

#[test]
fn test_best_rated_range_km() {
    let range = Range {
        rated: vec![
            RangeRated {
                cycle: RangeCycle::Wltp,
                range_km: 500.0,
                notes: None,
            },
            RangeRated {
                cycle: RangeCycle::Epa,
                range_km: 450.0,
                notes: None,
            },
            RangeRated {
                cycle: RangeCycle::Nedc,
                range_km: 600.0,
                notes: None,
            },
        ],
        real_world: None,
    };

    assert_eq!(range.best_rated_range_km(), Some(600.0));
}

#[test]
fn test_best_rated_range_km_single() {
    let range = Range {
        rated: vec![RangeRated {
            cycle: RangeCycle::Cltc,
            range_km: 550.0,
            notes: None,
        }],
        real_world: None,
    };

    assert_eq!(range.best_rated_range_km(), Some(550.0));
}

#[test]
fn test_best_rated_range_km_empty() {
    let range = Range {
        rated: vec![],
        real_world: None,
    };

    assert_eq!(range.best_rated_range_km(), None);
}

#[test]
fn test_efficiency_km_per_kwh() {
    let efficiency = Efficiency {
        energy_consumption_wh_per_km: Some(150.0),
        mpge: None,
        notes: None,
    };

    let result = efficiency.km_per_kwh();
    assert!(result.is_some());
    let km_per_kwh = result.unwrap();
    assert!((km_per_kwh - 6.666666666666667).abs() < 0.0001);
}

#[test]
fn test_efficiency_km_per_kwh_low_consumption() {
    let efficiency = Efficiency {
        energy_consumption_wh_per_km: Some(100.0),
        mpge: Some(130.0),
        notes: Some("Efficient".to_string()),
    };

    let result = efficiency.km_per_kwh();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 10.0);
}

#[test]
fn test_efficiency_km_per_kwh_none() {
    let efficiency = Efficiency {
        energy_consumption_wh_per_km: None,
        mpge: Some(120.0),
        notes: None,
    };

    assert_eq!(efficiency.km_per_kwh(), None);
}

#[test]
fn test_efficiency_default() {
    let efficiency = Efficiency::default();

    assert_eq!(efficiency.energy_consumption_wh_per_km, None);
    assert_eq!(efficiency.mpge, None);
    assert_eq!(efficiency.notes, None);
    assert_eq!(efficiency.km_per_kwh(), None);
}

#[test]
fn test_range_rated_serialization() {
    let rated = RangeRated {
        cycle: RangeCycle::Wltp,
        range_km: 500.0,
        notes: Some("Combined".to_string()),
    };

    let json = serde_json::to_value(&rated).unwrap();
    assert_eq!(json["cycle"], "wltp");
    assert_eq!(json["range_km"], 500.0);
    assert_eq!(json["notes"], "Combined");
}

#[test]
fn test_range_rated_without_notes_serialization() {
    let rated = RangeRated {
        cycle: RangeCycle::Epa,
        range_km: 450.0,
        notes: None,
    };

    let json = serde_json::to_value(&rated).unwrap();
    assert_eq!(json["cycle"], "epa");
    assert_eq!(json["range_km"], 450.0);
    assert!(json.get("notes").is_none());
}

#[test]
fn test_range_real_world_serialization() {
    let real_world = RangeRealWorld {
        profile: RealWorldProfile::Highway,
        range_km: 400.0,
        conditions: None,
        notes: None,
    };

    let json = serde_json::to_value(&real_world).unwrap();
    assert_eq!(json["profile"], "highway");
    assert_eq!(json["range_km"], 400.0);
}

#[test]
fn test_range_real_world_with_notes() {
    let real_world = RangeRealWorld {
        profile: RealWorldProfile::City,
        range_km: 350.0,
        conditions: None,
        notes: Some("Urban driving".to_string()),
    };

    let json = serde_json::to_value(&real_world).unwrap();
    assert_eq!(json["profile"], "city");
    assert_eq!(json["range_km"], 350.0);
    assert_eq!(json["notes"], "Urban driving");
}

#[test]
fn test_efficiency_serialization() {
    let efficiency = Efficiency {
        energy_consumption_wh_per_km: Some(160.0),
        mpge: Some(105.0),
        notes: Some("Combined cycle".to_string()),
    };

    let json = serde_json::to_value(&efficiency).unwrap();
    assert_eq!(json["energy_consumption_wh_per_km"], 160.0);
    assert_eq!(json["mpge"], 105.0);
    assert_eq!(json["notes"], "Combined cycle");
}

#[test]
fn test_range_with_real_world() {
    let range = Range {
        rated: vec![RangeRated {
            cycle: RangeCycle::Wltp,
            range_km: 500.0,
            notes: None,
        }],
        real_world: Some(vec![
            RangeRealWorld {
                profile: RealWorldProfile::Highway,
                range_km: 400.0,
                conditions: None,
                notes: None,
            },
            RangeRealWorld {
                profile: RealWorldProfile::City,
                range_km: 450.0,
                conditions: None,
                notes: None,
            },
        ]),
    };

    let json = serde_json::to_value(&range).unwrap();
    assert!(json["real_world"].is_array());
    assert_eq!(json["real_world"].as_array().unwrap().len(), 2);
}

#[test]
fn test_range_deserialization() {
    let json = r#"{
        "rated": [
            {"cycle": "wltp", "range_km": 500.0},
            {"cycle": "epa", "range_km": 450.0}
        ]
    }"#;

    let range: Range = serde_json::from_str(json).unwrap();
    assert_eq!(range.rated.len(), 2);
    assert_eq!(range.wltp_range_km(), Some(500.0));
    assert_eq!(range.epa_range_km(), Some(450.0));
}

#[test]
fn test_all_range_cycles() {
    let range = Range {
        rated: vec![
            RangeRated {
                cycle: RangeCycle::Wltp,
                range_km: 500.0,
                notes: None,
            },
            RangeRated {
                cycle: RangeCycle::Epa,
                range_km: 450.0,
                notes: None,
            },
            RangeRated {
                cycle: RangeCycle::Nedc,
                range_km: 550.0,
                notes: None,
            },
            RangeRated {
                cycle: RangeCycle::Cltc,
                range_km: 520.0,
                notes: None,
            },
            RangeRated {
                cycle: RangeCycle::Jc08,
                range_km: 480.0,
                notes: None,
            },
            RangeRated {
                cycle: RangeCycle::Other,
                range_km: 400.0,
                notes: Some("Custom test".to_string()),
            },
        ],
        real_world: None,
    };

    assert_eq!(range.rated.len(), 6);
    assert_eq!(range.best_rated_range_km(), Some(550.0));
}

#[test]
fn test_all_real_world_profiles() {
    let range = Range {
        rated: vec![],
        real_world: Some(vec![
            RangeRealWorld {
                profile: RealWorldProfile::Highway,
                range_km: 380.0,
                conditions: None,
                notes: None,
            },
            RangeRealWorld {
                profile: RealWorldProfile::City,
                range_km: 420.0,
                conditions: None,
                notes: None,
            },
            RangeRealWorld {
                profile: RealWorldProfile::Mixed,
                range_km: 400.0,
                conditions: None,
                notes: None,
            },
            RangeRealWorld {
                profile: RealWorldProfile::ColdWeather,
                range_km: 320.0,
                conditions: None,
                notes: None,
            },
            RangeRealWorld {
                profile: RealWorldProfile::Winter,
                range_km: 340.0,
                conditions: None,
                notes: None,
            },
            RangeRealWorld {
                profile: RealWorldProfile::Summer,
                range_km: 430.0,
                conditions: None,
                notes: None,
            },
        ]),
    };

    assert_eq!(range.real_world.as_ref().unwrap().len(), 6);
}
