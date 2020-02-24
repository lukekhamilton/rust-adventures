use wirefilter::{ExecutionContext, Scheme};

fn main() -> Result<(), failure::Error> {
    // Create a map of possible filter fields
    let schema = Scheme! {
        http.method: Bytes,
        http.ua: Bytes,
        port: Int,
    };

    // Parse a wireshark-like experssion into an AST.
    let ast = schema.parse(
        r#"
        http.method != "POST" &&
        not http.ua matches "(googlebot|facebook)" &&
        port in {80 443} 
    "#,
    )?;

    println!("Parsed filter representation: {:?}", ast);

    let filter = ast.compile();

    let mut ctx = ExecutionContext::new(&schema);

    ctx.set_field_value("http.method", "GET")?;

    ctx.set_field_value(
        "http.ua",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:66.0) Gecko/20100101 Firefox/66.0",
    )?;

    ctx.set_field_value("port", 443)?;

    // Execute the filter with given runtime values.
    println!("Filer matches: {:?}", filter.execute(&ctx)?); // true

    // Amend one of the runtime values and execute the filter again.
    ctx.set_field_value("port", 8080)?;

    println!("Filer matches: {:?}", filter.execute(&ctx)?); // false
    Ok(())
}
