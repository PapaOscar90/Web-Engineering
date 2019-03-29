/// Create `Cors` allowing for access from the frontend.
pub fn cors() -> rocket_cors::Cors {
    // FIXME Quickly thrown together CORS rules.
    use rocket::http::Method;
    use rocket_cors::{AllowedHeaders, AllowedOrigins};
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:3000"]);
    assert!(failed_origins.is_empty());

    rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
}
