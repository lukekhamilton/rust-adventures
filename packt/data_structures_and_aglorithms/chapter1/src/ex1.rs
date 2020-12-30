use std::rc::Rc;

#[derive(Debug)]
struct FileName {
    name: Rc<String>,
    ext: Rc<String>
}

pub fn ref_counter() {
    let name = Rc::new(String::from("main"));
    let ext = Rc::new(String::from("rs"));

    println!("{:#?}", name);

    for _ in 0..3 {
        println!("{:?}",FileName {
            name: name.clone(),
            ext: ext.clone()
        });
    }
}

