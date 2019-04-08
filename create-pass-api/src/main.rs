extern crate serde_derive;
extern crate rand;

use http::{header, StatusCode};
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use std::error::Error;

use rand::seq::SliceRandom;
use rand::thread_rng;

mod constants;

/// パスワード作成処理
fn create_pass() -> String {
    let mut pass_base: String = constants::PASS_PHRASE.to_string();
    // FIXME: using get_params (special_chars)
    // for ch in &constants::SPECIAL_CHARS {
    //     if args.flag_char.contains(&ch.to_string()) {
    //         pass_base += &ch.to_string();
    //     }
    // }
    // FIXME: using get_params (length)
    // let length: i32 = if args.flag_length > 0 {
    //     args.flag_length
    // } else {
    //     constants::DEFAULT_PASS_LEN
    // };
    let length: i32 = constants::DEFAULT_PASS_LEN;
    let pass_base_list: Vec<char> = pass_base.chars().collect();
    let mut rng = thread_rng();
    let pass_str: Option<String> = (0..length)
        .map(|_| Some(*pass_base_list.choose(&mut rng)? as char))
        .collect();
    return pass_str.unwrap();
}

fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .body(format!("{}", create_pass()))
        .expect("failed to render response");

    Ok(response)
}

/// refer: https://qiita.com/Anharu/items/6178cd57a4310ba1824f
// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
