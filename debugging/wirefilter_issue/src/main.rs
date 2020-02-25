use wirefilter::{ExecutionContext, Scheme};

#[derive(Debug)]
struct Data {
    email: String,
    rule: String,
}

fn main() {
    let data = Data {
        email: "luke@mettadata.com".to_string(),
        rule: r#""#.to_string(),
    };

    let result = process_rule_data(data);
}

fn process_rule_data(data: Data) -> Result<bool, failure::Error> {
    let scheme = Scheme! {
        email: Bytes,
    };

    // Parse a Wireshark-like expression into an AST
    let ast = scheme.parse(&data.rule)?; //  borrowed value does not live long enough??? HELP

    // Compile the AST into an executable filter
    let filter = ast.compile();

    // Set runtime field values to test the filter against.
    let mut ctx = ExecutionContext::new(&scheme);

    ctx.set_field_value("email", &*data.email)?;

    // Execute the filer with given runtime values.
    Ok(filter.execute(&ctx)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_process_rule_data() -> Result<(), failure::Error> {
        let test_data = Data {
            email: "luke@mettadata.com".to_string(),
            rule: r#"
                email == "luke@mettadata.com"
            "#
            .to_string(),
        };

        assert_eq!(process_rule_data(test_data)?, true);
        Ok(())
    }
}
