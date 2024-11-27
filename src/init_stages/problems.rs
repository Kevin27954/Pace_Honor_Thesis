pub fn init_problems() -> [&'static str; 7] {
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
        "\
// Fix the loops conditions

// Use a `while` loop to repeat until a condition is false:

// Syntax for a while loop
// while <expression> do
//     code here...
// end

let count = 0
// Fix the code below
while count <  do
     print(\"Count is:\", count)
     count = count + 1 // Increment the counter
end

// Use a `for` loop to iterate over a range:

// Syntax for a for loop
// for <var decl> , <expression> , <expression> do
//     code here...
// end

// Fix the code below
for let i = 0, i < , i = i +  do
    print(\"Number: \", i)
end
",
    ]
}
