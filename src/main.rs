
use reqwest::header;
use std::env;

use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct TextCompletion {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
    index: u32,
    logprobs: Option<Vec<f64>>,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // check if we have args
    if args.len() < 2 {
        println!("Usage: {} <query>", args[0]);
        return Ok(());
    }
    let query = &args[1];

    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", "Bearer ".parse().unwrap());

    // let prompt = "Explain this: {QUERY} Answer:".replace("{QUERY}", query);
    // make a json object out or this body query string
    // but replace QUERY with a variable query
    let payload = r#"{
  "model": "text-davinci-003",
  "prompt": "Explain this: \n {PROMPT} \n Answer:",
  "max_tokens": 64,
  "top_p": 1.0,
  "frequency_penalty": 0.0,
  "presence_penalty": 0.0
}"#.replace("{PROMPT}", &query);


    let client = reqwest::blocking::Client::new();

    let res = client.post("https://api.openai.com/v1/completions")
    // let res = client.post("http://localhost:8889")
        .headers(headers)
        .body(payload)
        .send()?
        .text()?;
    // println!("json res: {}", res);

    let result: TextCompletion = serde_json::from_str(&res.to_string()).unwrap();
    // check if we have a result
    if result.choices.len() < 1 {
        println!("No result");
        return Ok(());
    }
    let mut answer = result.choices[0].text.clone();
    answer = answer.replace("\n ", "");
    println!("{:#?}", answer);

    Ok(())
}
