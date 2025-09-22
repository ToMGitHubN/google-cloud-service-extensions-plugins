use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use log::info;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(MyHttpContext) });
}}

struct MyHttpContext;

impl Context for MyHttpContext {}


// ベーシック認証
impl HttpContext for MyHttpContext {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {

        // ステータス401 ベーシック認証を要求するレスポンスを生成
        self.send_http_response(
            401,
            vec![
                ("WWW-authenticate", "Basic realm=\"Secure Area\""),
            ],
            Some(format!("Unauthorized").as_bytes()),
        );
        
        info!("Forbidden request: Authorization header missing or invalid.");

        return Action::Pause;
    }
}