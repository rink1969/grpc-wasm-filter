// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use log::info;
use prost::Message;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpBodyRoot {config: "".to_string()}) });
}}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashDataRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}

struct HttpBodyRoot {
    config: String,
}

impl Context for HttpBodyRoot {}

impl RootContext for HttpBodyRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpBody {
            config: self.config.clone(),
        }))
    }

    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        info!("on_configure");
        if let Some(config) = self.get_plugin_configuration() {
            if let Ok(black_list) = String::from_utf8(config) {
                info!("black_list: {}", black_list);
                self.config = black_list;
            } else {
                info!("invalid config");
            }
        }
        true
    }
}

struct HttpBody {
    config: String,
}

impl Context for HttpBody {}

impl HttpContext for HttpBody {
    fn on_http_request_headers(&mut self, num_headers: usize, _end_of_stream: bool) -> Action {
        info!("on_http_request_headers, num_headers {}", num_headers);
        let headers = self.get_http_request_headers();
        for h in headers {
            info!("{}: {}", h.0, h.1);
        }
        Action::Continue
    }
    fn on_http_request_body(&mut self, body_size: usize, _end_of_stream: bool) -> Action {
        info!("on_http_request_body");
        if let Some(body) = self.get_http_request_body(0, body_size) {
            info!("body: {:?}", body);
            if let Ok(req) = HashDataRequest::decode(&body[5..]) {
                info!("req: {:?}", req);
                if hex::encode(req.data) == self.config {
                    info!("match black list");
                    self.send_http_response(
                        403,
                        vec![("content-type", "text/plain")],
                        Some(b"Blocked by proxy-wasm"),
                    );
                    return Action::Pause;
                }
            }
        }
        Action::Continue
    }
}
