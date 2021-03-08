


#[tokio::main]
async fn main() -> anyhow::Result<()>{
    println!("Hello, world egg-mode crate");

    let con_token = egg_mode::KeyPair::new("Z9rXKuLRUnPcBUERaMuT3LVPY", "HTnEwhAKtcQiiBtOpAF5Ea3lpMYEoloUewfVB7axqjRI903sIc");

    println!("{:?}", con_token);

    let request_token = egg_mode::auth::request_token(&con_token, "oob").await.unwrap();

    println!("Go to the following URL, sign in, and give me the PIN that comes back:");
    let auth_url = egg_mode::auth::authenticate_url(&request_token);
    println!("{:?}",auth_url);

    let mut pin = String::new();

    print!(">");
    std::io::stdin().read_line(&mut pin)?;

     let tok_result = egg_mode::auth::access_token(con_token, &request_token, pin)
                .await
                .unwrap();

    token = tok_result.0;
            user_id = tok_result.1;
            username = tok_result.2;

    println!("{:?}", token);

    Ok(())

}
