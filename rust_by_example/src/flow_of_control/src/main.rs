fn example1() {
    println!("Hello, world!");

    // Make `optional` of type `Option<i32>`
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^ Needed 2 indentations just so we could destructure
            // `i` from the option.
        }
        _ => {} // ^ Required because `match` is exhaustive. Doesn't it seem
                // like wasted space?
    };
}
fn example2() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // the `if let` construct reads `if let` destructures number into
    // `Some(i)`, evaluate the block `({})`
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specity a failure use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i)
    } else {
        println!("Didn't matched a number. Let's go with a letter!");
    }

    // Provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i)
    } else if i_like_letters {
        println!("Didnt matched a number. Lets go with another letter!");
    } else {
        println!("I dont like letters. Lets go with an emoticon!");
    }
}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}
fn example3() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches foo:bar
    if let Foo::Bar = a {
        println!("a is a foobar")
    }

    if let Foo::Bar = b {
        println!("b is a foobar")
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value)
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred!")
    }

    if let Foo = c {
        println!("c is a Foo")
    }

    let x = 1;
    if let Foo = x {
        println!("x is {}", Foo)
    }
}

fn main() {
    // example1();
    // example2();
    example3();
}
