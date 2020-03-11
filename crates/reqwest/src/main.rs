use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fetch()?;
    Ok(())
}

#[tokio::main]
async fn fetch() -> Result<(String), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    // println!("{:#?}", resp.get("origin").unwrap());
    let result = String::from(resp.get("origin").unwrap());

    Ok(result)
}

#[test]
fn test_main() {
    let results = fetch();
    assert_eq!(results.unwrap(), "220.240.182.41");
}
