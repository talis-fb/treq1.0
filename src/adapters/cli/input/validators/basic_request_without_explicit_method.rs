use anyhow::Result;

use crate::adapters::cli::input::cli_input::{CliCommandChoice, CliInput};
use crate::core::services::http_collections::entities::methods::METHODS;
use crate::utils::regexes;

pub fn validate_basic_request_without_explicit_method(mut input: CliInput) -> Result<CliInput> {
    if let CliCommandChoice::DefaultBasicRequest { ref url } = input.choice {
        let url = url.clone();
        input
            .request_input
            .request_items
            .iter()
            .any(|v| regexes::request_items::body_value().is_match(v))
            .then(|| {
                input.choice = CliCommandChoice::BasicRequest {
                    method: METHODS::POST,
                    url,
                };
            });
    }

    Ok(input)
}
