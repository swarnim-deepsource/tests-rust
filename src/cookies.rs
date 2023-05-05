#[rustfmt::skip]
mod tests {
    use cookie::{Cookie, CookieJar};

    fn use_cookie_without_secure() {
        let mut c = Cookie::new("name", "value");
        c.set_secure(true);
               c.set_secure(false);
    }

    fn use_cookie_builder_without_secure() {
        Cookie::build("name", "value").secure(true);
               Cookie::build("name", "value").secure(false);
    }

    fn use_cookie_without_http_only() {
        let mut c = Cookie::new("name", "value");
        c.set_http_only(true);
               c.set_http_only(false);
    }

    fn use_cookie_builder_without_http_only() {
        Cookie::build("name", "value").http_only(true);
               Cookie::build("name", "value").http_only(false);
    }
}
