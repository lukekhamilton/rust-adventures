// A concrete type A
struct A;

// Concrete type of S
struct S(A);

// Generic type 'SGen'
struct SGen<T>(T);

// In defining the type `Single`, the first use of A is not preceded by <A>
// therefore `Single` is a concrete type, and `A` is defined as above.
struct Single(A);

// Here, <T> precedes the first use of `T`, so `SingleGen` is a generic type.
// because the type parameter `T` is generic, it could be anything, including
// the concrete type `A` defined at the top.
struct SingleGen<T>(T);

pub fn structs() {
    // `Single` is concrete and explicitly takes `A`.
    let _s = Single(A);

    // Create a variable '_char' of type 'SingleGen<char>'
    // and give it the value 'SingleGen('a')
    // Here 'SingleGen has a type parameter explicitly specified
    let _char: SingleGen<char> = SingleGen('a');

    // SingleGen can also have a type parameter implicitly specified
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}

// The following functions all take ownership of the variable passed into
// then and immediately go out of scope, freeing the variable

// Define a function 'reg_fn' that takes a argument
// This has no <T> so this is not a generic function
fn reg_fn(_s: S) {}

// Define a function 'gen_spec_t' that takes an argument '_s' of type 'SGen<T>
// It has been explicitly given the type parameter 'A', but becuase 'A' has not
// been specified as a generic type parameter for 'gen_spec_t' it is not generic.
fn gen_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
// Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
fn generic<T>(_s: SGen<T>) {}

fn generic_next<G>(_s: SGen<G>) {}

pub fn functions_sigs() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // Explicitly specified type parameter 'char' to 'generic()'
    generic::<char>(SGen('a'));

    // Implicitly
    generic(SGen('C'));
}
