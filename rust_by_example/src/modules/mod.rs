mod my;

fn function() {
    println!("called function()");
}

pub fn run() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function()
}
