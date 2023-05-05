#[rustfmt::skip]
mod tests {
    use http::header::{HeaderMap, SERVER, SET_COOKIE};

    fn f() {
        let mut h = HeaderMap::new();
        //> [RS-A1004]: "Exposure of sensitive header: `SERVER`"
        h.insert(SERVER, 5.into());
        h.insert(SET_COOKIE, 42.into());
    }
}
