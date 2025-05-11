use super::LLMEngine;
use crate::util::{option_or_env, option_or_env_fallback, OptionMap};
use anyhow::Result;
use log::debug;
use serde_json::json;
use serde_json::Value as json;

pub struct Tool {
    name: String,
    definition: json,
    callback: Option<Box<dyn FnMut(json)>>,
}

pub struct Anthropic {
    model: String,
    api_key: String,
    base_url: String,
    tools: Vec<Tool>,
    content: Vec<json>,
    web_search: bool,
    thinking: bool,
    thinking_tokens: u32,
}

impl Anthropic {
    pub fn add_content(&mut self, content: json) {
        self.content.push(content);
    }

    fn anthropic_tool_definition(tool: &Tool) -> json {
        json!({
            "name": tool.definition["name"],
            "description": tool.definition["description"],
            "input_schema": tool.definition["parameters"],
        })
    }
}

impl LLMEngine for Anthropic {
    fn new(options: &OptionMap) -> Self {
        let api_key = option_or_env(&options, "api_key", "ANTHROPIC_API_KEY");
        let base_url = option_or_env_fallback(
            &options,
            "base_url",
            "ANTHROPIC_BASE_URL",
            "https://api.anthropic.com",
        );
        let model = options.get("model").unwrap().to_string();
        let web_search = options.get("web_search").map_or(false, |v| v == "true");
        let thinking = options.get("thinking").map_or(false, |v| v == "true");
        let thinking_tokens = options
            .get("thinking_tokens")
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(5000);

        Self {
            model,
            base_url,
            api_key,
            tools: Vec::new(),
            content: Vec::new(),
            web_search,
            thinking,
            thinking_tokens,
        }
    }

    fn register_tool(&mut self, name: &str, definition: json, callback: Box<dyn FnMut(json)>) {
        self.tools.push(Tool {
            name: name.to_string(),
            definition,
            callback: Some(callback),
        });
    }

    fn add_text_content(&mut self, text: &str) {
        self.add_content(json!({
            "type": "text",
            "text": text,
        }));
    }

    fn add_image_content(&mut self, base64_image: &str) {
        self.add_content(json!({
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": "image/png",
                "data": base64_image
            }
        }));
    }

    fn clear_content(&mut self) {
        self.content.clear();
    }

    fn execute(&mut self) -> Result<()> {
        let mut tool_definitions = self.tools.iter().map(|tool| Self::anthropic_tool_definition(tool)).collect::<Vec<_>>();

        // Add web search tool if enabled
        if self.web_search {
            tool_definitions.push(json!({
                "type": "web_search_20250305",
                "name": "web_search",
                "max_uses": 5
            }));
        }

        let mut body = json!({
            "model": self.model,
            "max_tokens": 10000,
            "messages": [{
                "role": "user",
                "content": self.content
            }],
            "tools": tool_definitions,
            "tool_choice": {
                "type": "auto",
                // "disable_parallel_tool_use": true
            }
        });

        // Add thinking configuration if enabled
        if self.thinking {
            body["thinking"] = json!({
                "type": "enabled",
                "budget_tokens": self.thinking_tokens
            });
        }

        debug!("Request: {}", body);

        let raw_response = ureq::post(&format!("{}/v1/messages", self.base_url))
            .header("x-api-key", self.api_key.as_str())
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .send_json(&body);

        let mut response = match raw_response {
            Ok(response) => response,
            Err(err) => {
                debug!("API Error: {}", err);
                return Err(anyhow::anyhow!("API ERROR: {}", err));
            }
        };

        // Read response body as string
        let body_text = response.body_mut().read_to_string().unwrap();
        let json: json = serde_json::from_str(&body_text).unwrap();
        debug!("Response: {}", json);
        let content_array = &json["content"];

        // Loop through all content entries
        for content_item in content_array.as_array().unwrap_or(&Vec::new()) {
            let content_type = content_item["type"].as_str().unwrap_or("");

            match content_type {
                "tool_use" => {
                    let function_name = content_item["name"].as_str().unwrap();
                    let function_input = &content_item["input"];
                    let tool = self
                        .tools
                        .iter_mut()
                        .find(|tool| tool.name == function_name);

                    if let Some(tool) = tool {
                        if let Some(callback) = &mut tool.callback {
                            callback(function_input.clone());
                            return Ok(());
                        } else {
                            return Err(anyhow::anyhow!(
                                "No callback registered for tool {}",
                                function_name
                            ));
                        }
                    } else {
                        return Err(anyhow::anyhow!(
                            "No tool registered with name {}",
                            function_name
                        ));
                    }
                },
                "thinking" => {
                    if let Some(thinking) = content_item.get("thinking") {
                        debug!("Thinking: {}", thinking);
                    }
                },
                "text" => {
                    if let Some(text) = content_item.get("text") {
                        debug!("Text: {}", text);
                    }
                },
                _ => {
                    debug!("Unknown content type: {}", content_type);
                }
            }
        }

        Err(anyhow::anyhow!("No tool calls found in response"))
    }
}
