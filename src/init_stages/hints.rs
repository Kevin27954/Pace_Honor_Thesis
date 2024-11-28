pub fn init_hints() -> [&'static str; 9] {
    [
        "\
To make Bitelang talk to you, you can 
use:
print()

Put something inside the parentheses, 
like words or numbers. For words, 
use quotes like \"hello\".

Try typing: print(\"hello world!\").

Want to print numbers instead?
Type: print(42)

No quotes needed for numbers.
",
        "\
You can type whole numbers like 5 or 
1000.

Want to include fractions or decimals?
Use numbers like 3.14 or 0.25.

Try typing:
print(5 + 3)
Bitelang can do math too!
    + (Add)        - (Subtract)
    * (Multiply)   / (Divide)
",
        "\
Bitelang can understand true or false!
These are called booleans.

Try typing:
print(true) or print(false)

Booleans are like tiny decision-makers.
For example, type:
print(5 > 3)
Does Bitelang say True?
",
        "\
Strings are just text in Bitelang.
Write them inside quotes, like 
\"hello\" or \"world\".

Any characters in between the \"\"
are accepted. \n
Ex.
\"This is a sample String.\"
\"This too: 🐶.\"
\"Numbers too: 12345\"

Use print() to display the String.
Watch Bitelang echo your words! \n
",
        "\
Variables are like containers.
You can store anything in them: 
numbers, strings, or booleans.

To create a variable, use 'let',
a name, and '='.

Try this:
let message = \"Hello, Bitelang!\"
print(message)

You can change what's inside a
variable.
let message = \"Hello, Bitelang!\"
message = message + \" Nice!\"
//      or
message = 3939
print(message)
",
        "\
Conditionals let Bitelang make 
decisions!

In Bitelang, we use logical operator
to create logic.\n
In Bitelang, you can use:
    > (Greater)
    < (Less)
    >= (Greater or Equal)
    <= (Less or Equal)
    == (Equals)
    != (Not Equals)
    and (And Logcial Operator)
    or (Or Logcial Operator)

These will evaluate expressions to
either 'true' or 'false'.
",
        "\
As long as the <expression> evaluate
to either 'true' or 'false', a `while`
loop will work.

Use what you learned last stage to
help you out here.

              -----------

You can also use a for loop to control
your repetition more precisely!
A for loop needs:

A starting variable declaration.
An expression to check when to stop.
An update to the variable.
",
        "\
Functions are reusable blocks of code.
They let you group instructions 
together.
",
        "\
Structs let you bundle related data.
Once defined, you can create and use
them!

Here’s how to use a struct in Bite:

Create an empty instance:
let var_name = <struct_name>{}

Initialize its fields:
var_name.<field1> = \"Field 1\"
var_name.<field2> = 42

Access its fields:
print(var_name.<field1>)
            -- Prints \"Field 1\"
print(var_name.<field2>) 
            -- Prints 42


",
    ]
}
