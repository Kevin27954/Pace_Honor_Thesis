pub fn init_problems() -> [&'static str; 10] {
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
        "\
// Define and call a simple function called 'greet':
function () do
     print(\"Hello from a function!\")
end
// call your function below
<name>()

// Create a function that takes parameters, 'a' and 'b':
function add_numbers() do
    print(\"The sum is:\", a + b)
end
add_numbers(5, 10)

// Create a function that returns a number squared:
let num = <type your number>
function square(number) do
    return <number squared>
end
let result = square(num)
print(\"Square of \",  num, \" is:\", result)
            ",
        "\
// Define and use a basic struct:
struct Person { name }
let alice = Person {};
// Access and modify the field 'name' from the struct alice.
alice. = 
print(\"Name:\", alice.name)

// Create a Point struct with fields 'x' and 'y'
struct Point {
    Remove me and fill me in
}
let origin = Point {}
print(\"Point at:\", origin.x, origin.y)
",
        "\
// Write a program to compare three books individually. Each book has a 'title', 'author', and
// number of 'pages'. Your program should:
// 
// Define a struct to represent a book.
// - Create three separate book variables.
// - Write a function to compare the number of pages between two books and determine which
//       one has more pages.
// - Use conditionals to identify and print the book with the most pages at the end.
",
    ]
}
