use std::collections::HashMap;
use std::str::FromStr;

use clap::ArgMatches;

use crate::app::services::request::entities::{OptionalRequestData, METHODS};
use crate::view::cli::commands::CliCommand;
use crate::view::cli::validators;

fn get_inputs_from_clap_matches(args: &ArgMatches) -> Result<Vec<&String>, String> {
    Ok(args
        .get_many::<String>("inputs")
        .ok_or("No inputs at command")?
        .collect())
}

pub fn parse_clap_input_to_commands(args: ArgMatches) -> Result<Vec<CliCommand>, String> {
    if args.subcommand().is_none() {
        let inputs = get_inputs_from_clap_matches(&args)?;
        let (url, extra_inputs) = inputs.split_first().ok_or("No inputs")?;

        let mut optional_request = parse_list_of_data_to_request_data(extra_inputs.to_vec())?;
        optional_request.url = Some(url.to_string());

        if optional_request.body.is_some() && optional_request.method.is_none() {
            optional_request.method = Some(METHODS::POST);
        }

        let request = optional_request.to_request_data();

        return Ok(vec![CliCommand::SubmitRequest { request }]);
    }

    let subcommand = args.subcommand().unwrap();

    match subcommand {
        ("GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "PATCH", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let (url, extra_inputs) = inputs.split_first().ok_or("No inputs")?;
            let method = METHODS::from_str(subcommand.0)?;

            let mut optional_request = parse_list_of_data_to_request_data(extra_inputs.to_vec())?;
            optional_request.url = Some(url.to_string());
            optional_request.method = Some(method);

            let mut commands = Vec::new();

            let has_save_as_flag = matches.get_one::<String>("save-as");
            if let Some(request_name) = has_save_as_flag {
                commands.push(CliCommand::SaveRequest {
                    request: optional_request.clone(),
                    request_name: request_name.clone(),
                })
            }

            let request = optional_request.to_request_data();

            commands.push(CliCommand::SubmitRequest { request });

            Ok(commands)
        }
        ("edit", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let (name_saved_request, extra_inputs) = inputs.split_first().ok_or("No inputs")?;
            let mut optional_request_data =
                parse_list_of_data_to_request_data(extra_inputs.to_vec())?;

            let mut commands = Vec::new();

            let has_manual_url_flag = matches.get_one::<String>("url_manual");
            if let Some(url) = has_manual_url_flag {
                optional_request_data.url = Some(url.clone());
            }

            let has_manual_method_flag = matches.get_one::<String>("method_manual");
            if let Some(method) = has_manual_method_flag {
                optional_request_data.method = Some(METHODS::from_str(method)?);
            }

            let has_save_as_flag = matches.get_one::<String>("save-as");
            if let Some(request_name) = has_save_as_flag {
                commands.push(CliCommand::SaveRequest {
                    request: optional_request_data.clone(),
                    request_name: request_name.clone(),
                })
            }

            Ok(Vec::from([CliCommand::SaveRequest {
                request: optional_request_data,
                request_name: name_saved_request.to_string(),
            }]))
        }
        ("rename", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let old_name = inputs[0];
            let new_name = inputs[1];
            Ok(Vec::from([CliCommand::RenameSavedRequest {
                request_name: old_name.to_string(),
                new_name: new_name.to_string(),
            }]))
        }
        ("remove", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;
            let request_name = inputs[0].to_string();
            Ok(Vec::from([CliCommand::RemoveSavedRequest { request_name }]))
        }
        ("run", matches) => {
            let inputs = get_inputs_from_clap_matches(matches)?;

            let request_name = inputs[0].to_string();
            let extra_inputs = &inputs[1..];

            let mut commands = Vec::new();

            if extra_inputs.is_empty() {
                commands.push(CliCommand::SubmitSavedRequest { request_name });
                return Ok(commands);
            }

            let mut optional_request_data =
                parse_list_of_data_to_request_data(extra_inputs.to_vec())?;

            let has_manual_url_flag = matches.get_one::<String>("url_manual");
            if let Some(url) = has_manual_url_flag {
                optional_request_data.url = Some(url.clone());
            }

            let has_manual_method_flag = matches.get_one::<String>("method_manual");
            if let Some(method) = has_manual_method_flag {
                optional_request_data.method = Some(METHODS::from_str(method)?);
            }

            let has_save_current_flag = matches.get_one::<bool>("save");
            if let Some(&true) = has_save_current_flag {
                commands.push(CliCommand::SaveRequest {
                    request: optional_request_data.clone(),
                    request_name: request_name.clone(),
                })
            }

            let has_save_as_flag = matches.get_one::<String>("save-as");
            if let Some(request_name) = has_save_as_flag {
                commands.push(CliCommand::SaveRequest {
                    request: optional_request_data.clone(),
                    request_name: request_name.clone(),
                })
            }

            commands.push(CliCommand::SubmitSavedRequestWithAdditionalData {
                request_name,
                request_data: optional_request_data,
            });

            Ok(commands)
        }
        _ => Err("No valid subcommand".into()),
    }
}

fn parse_list_of_data_to_request_data<'a>(
    inputs: impl IntoIterator<Item = &'a String>,
) -> Result<OptionalRequestData, String> {
    let mut request = OptionalRequestData::default();
    let mut body_data_values = HashMap::new();

    inputs.into_iter().for_each(|input| {
        if request.url.is_none() && validators::is_url(input) {
            request.url = Some(input.to_owned());
        }

        if validators::is_header_input(input) {
            let (key, value) = input.split_once(':').unwrap();
            request
                .headers
                .get_or_insert(HashMap::new())
                .insert(key.to_owned(), value.to_owned());
        } else if validators::is_body_data_input(input) {
            let (key, value) = input.split_once('=').unwrap();
            body_data_values.insert(key.to_owned(), value.to_owned());
        }
    });

    request.body = {
        if !body_data_values.is_empty() {
            Some(
                serde_json::to_string(&body_data_values)
                    .map_err(|_| "Error to parse data body to json")?,
            )
        } else {
            None
        }
    };

    Ok(request)
}
