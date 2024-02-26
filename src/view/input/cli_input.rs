use std::str::FromStr;

use anyhow::{Error, Result};
use clap::ArgMatches;

use crate::app::services::request::entities::methods::METHODS;

pub struct CliInput {
    pub choice: CliCommandChoice,
    pub request_input: RequestBuildingOptions,
    pub save_options: SavingOptions,
}

pub enum CliCommandChoice {
    DefaultBasicRequest {
        url: String,
        print_body_only: bool,
    },
    BasicRequest {
        method: METHODS,
        url: String,
        print_body_only: bool,
    },
    Run {
        request_name: String,
        print_body_only: bool,
        save: bool,
    },
    Edit {
        request_name: String,
    },
    Remove {
        request_name: String,
    },
    Rename {
        request_name: String,
        new_name: String,
        has_to_confirm: bool,
    },
    Inspect {
        request_name: String,
    },
    Ls,
}

impl CliInput {
    pub fn from_clap_matches(matches: &ArgMatches) -> Result<CliInput> {
        if matches.subcommand().is_none() {
            let url = clap_args_utils::get_input(matches)?.to_string();
            let request_input = RequestBuildingOptions::from_clap_matches(matches)?;
            let print_body_only = *matches.get_one::<bool>("print-body-only").unwrap_or(&false);
            let save_options = SavingOptions::from_clap_matches(matches)?;

            return Ok(CliInput {
                choice: CliCommandChoice::DefaultBasicRequest {
                    url,
                    print_body_only,
                },
                request_input,
                save_options,
            });
        }

        let (subcommand, matches) = matches.subcommand().unwrap();

        let request_input = RequestBuildingOptions::from_clap_matches(matches)?;
        let save_options = SavingOptions::from_clap_matches(matches)?;

        match subcommand {
            "edit" => {
                let request_name = clap_args_utils::get_input(matches)?.to_string();

                Ok(CliInput {
                    choice: CliCommandChoice::Edit { request_name },
                    request_input,
                    save_options,
                })
            }
            "rename" => {
                let inputs = clap_args_utils::get_many_inputs(matches)?;
                let request_name = inputs[0].to_string();
                let new_name = inputs[1].to_string();
                let has_to_confirm = !*matches.get_one::<bool>("no-confirm").unwrap_or(&false);

                Ok(CliInput {
                    choice: CliCommandChoice::Rename {
                        request_name,
                        new_name,
                        has_to_confirm,
                    },
                    request_input,
                    save_options,
                })
            }
            "remove" => {
                let request_name = clap_args_utils::get_input(matches)?.to_string();

                Ok(CliInput {
                    choice: CliCommandChoice::Remove { request_name },
                    request_input,
                    save_options,
                })
            }
            "ls" => Ok(CliInput {
                choice: CliCommandChoice::Ls,
                request_input,
                save_options,
            }),
            "inspect" => {
                let request_name = clap_args_utils::get_input(matches)?.to_string();

                Ok(CliInput {
                    choice: CliCommandChoice::Inspect { request_name },
                    request_input,
                    save_options,
                })
            }
            "run" => {
                let request_name = clap_args_utils::get_input(matches)?.to_string();
                let should_save_current_request = matches.get_one::<bool>("save").unwrap_or(&false);
                let print_body_only = *matches.get_one::<bool>("print-body-only").unwrap_or(&false);

                Ok(CliInput {
                    choice: CliCommandChoice::Run {
                        request_name,
                        print_body_only,
                        save: *should_save_current_request,
                    },
                    request_input,
                    save_options,
                })
            }
            "GET" | "POST" | "PUT" | "DELETE" | "HEAD" | "PATCH" => {
                let url = clap_args_utils::get_input(matches)?.to_string();
                let method = METHODS::from_str(subcommand)?;
                let print_body_only = *matches.get_one::<bool>("print-body-only").unwrap_or(&false);

                Ok(CliInput {
                    choice: CliCommandChoice::BasicRequest {
                        method,
                        url,
                        print_body_only,
                    },
                    request_input,
                    save_options,
                })
            }
            _ => Err(Error::msg("No valid subcommand")),
        }
    }
}

#[derive(Default)]
pub struct RequestBuildingOptions {
    pub request_items: Vec<String>,
    pub raw_body: Option<String>,
    pub url_manual: Option<String>,
    pub method_manual: Option<METHODS>,
}
impl RequestBuildingOptions {
    pub fn from_clap_matches(matches: &ArgMatches) -> Result<RequestBuildingOptions> {
        Ok(RequestBuildingOptions {
            request_items: clap_args_utils::get_many(matches, "request-items").unwrap_or_default(),
            raw_body: clap_args_utils::get_one(matches, "raw"),
            url_manual: clap_args_utils::get_one(matches, "url-manual"),
            method_manual: clap_args_utils::get_one(matches, "method-manual")
                .and_then(|m| METHODS::from_str(&m).ok()),
        })
    }
}

#[derive(Default)]
pub struct SavingOptions {
    pub save_as: Option<String>,
}
impl SavingOptions {
    pub fn from_clap_matches(matches: &ArgMatches) -> Result<SavingOptions> {
        Ok(SavingOptions {
            save_as: clap_args_utils::get_one(matches, "save-as"),
        })
    }
}

mod clap_args_utils {
    use super::*;

    pub fn get_input(args: &ArgMatches) -> Result<String> {
        clap_args_utils::get_one(args, "inputs").ok_or(Error::msg("No input given"))
    }

    pub fn get_many_inputs(args: &ArgMatches) -> Result<Vec<String>> {
        clap_args_utils::get_many(args, "inputs").ok_or(Error::msg("No inputs given"))
    }

    pub fn get_one(args: &ArgMatches, name: &str) -> Option<String> {
        args.try_get_one::<String>(name).ok().flatten().cloned()
    }

    pub fn get_many(args: &ArgMatches, name: &str) -> Option<Vec<String>> {
        Some(
            args.try_get_many::<String>(name)
                .ok()??
                .cloned()
                .collect::<Vec<_>>(),
        )
    }
}
