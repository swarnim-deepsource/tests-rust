#[rustfmt::skip]
mod tests {
    use cookie::{Cookie, CookieJar};

    fn use_cookie_without_secure() {
        let mut c = Cookie::new("name", "value");
        c.set_secure(true);
       //> [RS-A1002]: "`Secure` attribute explicitly set to `false` with `Cookie::set_secure`"
        c.set_secure(false);
    }

    fn use_cookie_builder_without_secure() {
        Cookie::build("name", "value").secure(true);
       //> [RS-A1002]: "`Secure` attribute explicitly set to `false` with `CookieBuilder::secure`"
        Cookie::build("name", "value").secure(false);
    }

    fn use_cookie_without_http_only() {
        let mut c = Cookie::new("name", "value");
        c.set_http_only(true);
       //> [RS-A1003]: "`HttpOnly` attribute explicitly set to `false` with `Cookie::set_http_only`"
        c.set_http_only(false);
    }

    fn use_cookie_builder_without_http_only() {
        Cookie::build("name", "value").http_only(true);
       //> [RS-A1003]: "`HttpOnly` attribute explicitly set to `false` with `CookieBuilder::http_only`"
        Cookie::build("name", "value").http_only(false);
    }
}
