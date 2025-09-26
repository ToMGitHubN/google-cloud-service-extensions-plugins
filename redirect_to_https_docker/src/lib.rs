use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use log::info;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(RedirectContext::default()) });
}}

#[derive(Default)]
struct RedirectContext {
    responded: bool,
}

impl Context for RedirectContext {}

impl HttpContext for RedirectContext {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {

        // 既にHTTPSなら何もしない
        if let Some(proto) = self.get_http_request_header("x-forwarded-proto") {
            if proto.eq_ignore_ascii_case("https") { return Action::Continue; }
        }
        if let Some(scheme) = self.get_http_request_header(":scheme") {
            if scheme.eq_ignore_ascii_case("https") { return Action::Continue; }
        }

        let authority = match self.get_http_request_header(":authority") {
            Some(a) => a,
            None => {
                self.send_http_response(
                    400,
                    vec![("Content-Type", "text/plain; charset=utf-8")],
                    Some(b"Bad Request: missing :authority"),
                );
                return Action::Pause;
            }
        };

        let path = self.get_http_request_header(":path").unwrap_or_else(|| "/".to_string());
        let location = format!("https://{}{}", authority, path);

        // 308 Permanent Redirect を返す
        let body = format!(
            "<html><head><title>Redirect</title></head><body><p>Permanent Redirect: <a href=\"{0}\">{0}</a></p></body></html>",
            location
        );
        self.send_http_response(
            308,
            vec![
                ("Location", location.as_str()),
                ("Cache-Control", "no-store"),
                ("Content-Type", "text/html; charset=utf-8"),
            ],
            Some(body.as_bytes()),
        );
        info!("Redirecting to HTTPS: {}", location);

        Action::Pause
    }
}
