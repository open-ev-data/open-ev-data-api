use ev_server::models::Pagination;

#[test]
fn test_pagination_logic() {
    // Exact division
    let p = Pagination::new(1, 20, 100);
    assert_eq!(p.total_pages, 5);

    // Remainder
    let p = Pagination::new(1, 20, 101);
    assert_eq!(p.total_pages, 6);

    // Empty
    let p = Pagination::new(1, 20, 0);
    assert_eq!(p.total_pages, 0);

    // Single page partial
    let p = Pagination::new(1, 20, 5);
    assert_eq!(p.total_pages, 1);
}
