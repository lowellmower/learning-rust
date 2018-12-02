fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // compiler will also tell you how to silence this by adding
    // note: #[warn(unused_variables)] on by default

    println!("********* BREAK *********");

    // variable bindings are immutable by default, however, this can be
    // overridden with a reserved word modifier 'mut'
    let _immutable_bind = 1;
    let mut mutable_bind = 1;

    mutable_bind += 1;
    mutable_bind += 1;
    println!("Mutable bind after augmentation {}", mutable_bind);

    // _immutable_bind += 1;
    // to compile, you'll have to comment L33 out otherwise receive a very
    // helpful error from the compiler like:
    // 33 |     _immutable_bind += 1;
    //    |     ^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable    
}