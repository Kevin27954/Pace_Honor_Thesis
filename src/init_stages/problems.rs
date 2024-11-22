pub fn init_problems() -> [&'static str; 6] {
    [
        "\
// Play around with the print function. Get use to it.
// Try with numbers or multiple numbers:

// print(123)
// print(123, 456, 789)
",
        "\
// Declare and print an integer:
// print(10)

// Declare and print a float:
// print(3.14)

// Perform some operations:
// print(10 + 5, 3.14 * 2.0)
",
        "\
// Declare and print a boolean:
// print(true)

// Combine booleans with logical operators:
// print(true and true, true or false)

// print(!true)
",
        "\
// Declare and print a string:
// print(\"Hello Mom!\")

// Combine strings:
// print(\"John\" + \" \" + \"Doe\")

// Use escape characters for formatting:
// print(\"A new line\\n WOW!\")
",
        "\
// Give the 'age' variable a value.
let age = 
print(age)

// Update the 'age' variable.
age = 
print(age)
",
        "\
// Use an `if` statement to check a condition:
let number = 10

// Use the correct math comparsion operator from:
// <, >, >=, <=
// To fix the expression below
// number   5
//        ^
if number  5 then
    print(\"Number is greater than 5!\")
end

// Give 'is_sunny' as boolean value
let is_sunny = 

// Add an `else` clause for alternative actions:
if is_sunny then
    print(\"Let's go outside!\")
else 
    print(\"Better stay indoors.\")
end

",
    ]
}
