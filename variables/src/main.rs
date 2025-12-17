fn main() {
    /*
    - By default variables in Rust are immutable, however we can still make them to be mutable
    - When a variable is immutable, once a value is bound to a name, you can’t change that value.
     */

    // creating a variable
    let immutable_variable = "This is an immutable variable";

    // my_immutable_variable = "change immutable variable" // this won't compile
    println!("{}", immutable_variable);

    // to declare immutable variables we use the `mut` keyword
    let mut mutable_variable = 42;
    println!("The original value of the mutable variable is {}", mutable_variable);

    // changing it
    mutable_variable = 5;
    println!("The value of mutable variable after the modification is {}", mutable_variable);

    /*
    Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables
    - We aren’t allowed to use mut with constants. Constants aren’t just immutable by default they’re always immutable.
     */

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The duration in seconds is {}", THREE_HOURS_IN_SECONDS);

    /*
    Shadowing of variables
    - Declaring a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows
     */

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
