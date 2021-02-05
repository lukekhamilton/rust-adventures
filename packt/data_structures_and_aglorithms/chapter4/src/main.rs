mod doublelinkedlist;
mod singlelinkedlist;

fn singlelistexample() {
    let mut tlog = singlelinkedlist::TransactionLog::new_empty();

    tlog.append(String::from("1"));
    tlog.append(String::from("2"));
    tlog.append(String::from("3"));

    println!("{:#?}", tlog);

    let result = tlog.pop();

    println!("{:?}", result);
    println!("{:#?}", tlog)
}

fn doublelistexample() {}

fn main() {
    println!("Hello, world!");
    singlelistexample()
}
