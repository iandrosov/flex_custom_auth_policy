pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde::Deserialize;
use std::time::Duration;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(CustomAuthRootContext {
            config: CustomAuthConfig::default(),
        })
    });
}}


struct CustomAuthRootContext {
    config: CustomAuthConfig,
}

impl Context for CustomAuthRootContext {}

impl RootContext for CustomAuthRootContext {

    fn on_configure(&mut self, _: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            self.config = serde_json::from_slice(config_bytes.as_slice()).unwrap();
        }

        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(CustomAuthHttpContext {
            config: self.config.clone(),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

#[derive(Default, Clone, Deserialize)]
struct CustomAuthConfig {

    #[serde(alias = "secret-value")]
    secret_value: String,
    #[serde(alias = "server-host")] 
    server_host: String,
}

struct CustomAuthHttpContext {
    pub config: CustomAuthConfig,
}

impl Context for CustomAuthHttpContext {}

impl HttpContext for CustomAuthHttpContext {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {

        if let Some(value) = self.get_http_request_header("x-custom-auth") {
            if self.config.secret_value == value {
                // Check value host URL from config param and set default if missing
                let confighost = &self.config.server_host;
                if !confighost.is_empty() {
                    println!("{}",confighost);   
                }
                let serverhost = String::from("httpbin.org");
                println!("wasm got response from HTTP AUTH call");
                //X-CLIENT-ID
                //return Action::Continue;
                // HTTP Call to auth value
                self.dispatch_http_call(
                    "httpbin",
                    vec![
                        (":method", "GET"),
                        (":path", "/bytes/1"),
                        (":authority", &serverhost),
                    ],
                    None,
                    vec![],
                    Duration::from_secs(5),
                )
                .unwrap();

            }else{
                self.send_http_response(401, Vec::new(), None);
            }
        }else{
            self.send_http_response(401, Vec::new(), None);
        }
        Action::Pause
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        println!("wasm got response from HTTP AUTH call");
        self.set_http_response_header("Powered-By", Some("proxy-wasm"));
        Action::Continue
    }
}