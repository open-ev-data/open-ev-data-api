use ev_server::db::{ListParams, PostgresDatabase};

#[test]
fn test_postgres_init() {
    assert!(PostgresDatabase::new("postgresql://localhost/test").is_ok());
    assert!(PostgresDatabase::new("postgres://localhost/test").is_ok());
    assert!(PostgresDatabase::new("mysql://localhost/test").is_err());
    assert!(PostgresDatabase::new("invalid").is_err());
}

#[test]
fn test_postgres_stub_methods() {
    let db = PostgresDatabase::new("postgres://localhost").unwrap();

    assert_eq!(db.get_vehicle_count().unwrap(), 0);

    let params = ListParams {
        make: None,
        model: None,
        year: None,
        vehicle_type: None,
        min_range_km: None,
        max_range_km: None,
        page: 1,
        per_page: 20,
        sort_by: None,
        sort_order: None,
    };
    let (list, total) = db.list_vehicles(&params).unwrap();
    assert!(list.is_empty());
    assert_eq!(total, 0);

    assert!(db.list_makes().unwrap().is_empty());

    let (results, count) = db.search("query", 1, 20).unwrap();
    assert!(results.is_empty());
    assert_eq!(count, 0);
}
