use ev_core::{Range, RangeCycle, RangeRated, RangeRealWorld, RealWorldProfile};

#[test]
fn test_range_helpers() {
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
        real_world: Some(vec![RangeRealWorld {
            profile: RealWorldProfile::Highway,
            range_km: 400.0,
            notes: None,
            conditions: None,
        }]),
    };

    assert_eq!(range.wltp_range_km(), Some(500.0));
    assert_eq!(range.epa_range_km(), Some(450.0));
    // cltc_range_km does not exist
}

#[test]
fn test_range_empty() {
    let range = Range {
        rated: vec![],
        real_world: None,
    };
    assert_eq!(range.wltp_range_km(), None);
    assert_eq!(range.epa_range_km(), None);
}
