use vignette;

#[test]
fn check_auth_cookie_name() {
    assert_eq!(vignette::utils::AUTH_COOKIE_NAME, "vignette-id");
}

// TODO: set up integration tests for apis here.

// TODO: add function to get a test client for rocket
