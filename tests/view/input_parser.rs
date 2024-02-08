// #![allow(non_snake_case)]
//
// use insta::assert_yaml_snapshot as assert_snapshot;
// use treq::view::input::cli_definition::root_command;
// use treq::view::input::parser::parse_clap_input_to_commands;
//
// #[test]
// fn should_parse_to_normal_GET_submit_without_passing_method_as_subcommand_and_no_body() {
//     let input = "treq url.com";
//     let matches = root_command().get_matches_from(input.split_whitespace());
//     let result = parse_clap_input_to_commands(matches).unwrap();
//
//     assert!(result.len() == 1);
//     assert_snapshot!(result);
// }
//
// #[test]
// fn should_parse_to_normal_POST_submit_without_passing_method_as_subcommand_but_passing_some_body_data(
// ) {
//     let input = "treq url.com Hello=World";
//     let matches = root_command().get_matches_from(input.split_whitespace());
//     let result = parse_clap_input_to_commands(matches).unwrap();
//
//     assert!(result.len() == 1);
//     assert_snapshot!(result);
// }
//
// #[test]
// fn should_ignore_body_inputs_in_GET_request() {
//     let input = "treq GET url.com Hello=World";
//     let matches = root_command().get_matches_from(input.split_whitespace());
//     let result = parse_clap_input_to_commands(matches).unwrap();
//
//     assert!(result.len() == 1);
//     assert_snapshot!(result);
// }
//
// #[test]
// fn should_parse_all_methods_subcommands_to_normal_submits() {
//     let all_methods = ["GET", "POST", "PUT", "DELETE", "HEAD", "PATCH"];
//
//     let inputs = all_methods
//         .iter()
//         .map(|method| format!("treq {} url.com", method))
//         .collect::<Vec<_>>();
//
//     inputs.iter().for_each(|input| {
//         let matches = root_command().get_matches_from(input.split_whitespace());
//         let result = parse_clap_input_to_commands(matches).unwrap();
//         assert!(result.len() == 1);
//         assert_snapshot!(result);
//     });
// }
//
// #[test]
// fn should_parse_same_way_with_or_without_protocol_in_url() {
//     let input1 = "treq url.com";
//     let input2 = "treq http://url.com";
//
//     let matches1 = root_command().get_matches_from(input1.split_whitespace());
//     let matches2 = root_command().get_matches_from(input2.split_whitespace());
//
//     let _ = parse_clap_input_to_commands(matches1).unwrap();
//     let _ = parse_clap_input_to_commands(matches2).unwrap();
// }
//
// #[test]
// fn should_error_if_no_input() {
//     let input = "treq";
//     let matches = root_command().get_matches_from(input.split_whitespace());
//     let result = parse_clap_input_to_commands(matches);
//     assert!(result.is_err());
//     assert!(result.unwrap_err().to_string().contains("No inputs"));
// }
//
// #[test]
// fn should_error_if_url_is_invalid() {
//     let invalid_urls = ["treq htp://url", "treq url,io", "treq url-io:"];
//
//     let matches =
//         invalid_urls.map(|input| root_command().get_matches_from(input.split_whitespace()));
//     let mut results = matches.into_iter().map(parse_clap_input_to_commands);
//     assert!(results.all(|result| result.is_err()));
// }
//
// #[test]
// fn should_raw_flag_work_equal_param_body_definition() {
//     let input1 = "treq POST url.com Hello=World";
//     let matches1 = root_command().get_matches_from(input1.split_whitespace());
//
//     let input2 = r#"treq POST url.com --raw {"Hello":"World"}"#;
//     let matches2 = root_command().get_matches_from(input2.split_whitespace());
//
//     let result1 = parse_clap_input_to_commands(matches1);
//     let result2 = parse_clap_input_to_commands(matches2);
//
//     assert!(result1.is_ok());
//     assert!(result2.is_ok());
//     assert_eq!(result1.unwrap(), result2.unwrap());
// }
//
// #[test]
// fn should_return_error_if_raw_flag_and_param_body_are_both_used() {
//     let input = r#"treq POST url.com --raw {"Hello":"World"}" Hello=World"#;
//     let matches = root_command().get_matches_from(input.split_whitespace());
//     let result = parse_clap_input_to_commands(matches);
//     assert!(result.is_err());
//     assert!(result.unwrap_err().to_string().contains("--raw"));
// }
//
// #[test]
// fn should_execute_with_valid_urls() {
//     const VALID_URLS: &[&str] = &[
//         "google.com",
//         "google.com/",
//         "google.com?",
//         "google.com:81",
//         "google.com:81/",
//         "google.com/search/advanced",
//         "google.com/search/advanced/",
//         "google.com?search=Rust",
//         "google.com?search=Rust&country=br",
//         "google.com/search/advanced?name=john",
//         "google.com/search/advanced/?name=john",
//         "google.com/search/advanced?name=john&sort=true",
//         "google.com/search/advanced?name=john&sort=true#landing-page",
//         "google.com/search/advanced#landing-page",
//         "google.com/search/advanced/#landing-page",
//         "google.com#landing-page",
//         "google.com/#landing-page",
//         "localhost",
//         "localhost/",
//         "localhost:8081/",
//         "localhost:8081/api/v1/local",
//     ];
//
//     for url in VALID_URLS {
//         let input = format!("treq GET {}", url);
//         let matches = root_command().get_matches_from(input.split_whitespace());
//         let result = parse_clap_input_to_commands(matches);
//         assert!(result.is_ok());
//     }
// }
