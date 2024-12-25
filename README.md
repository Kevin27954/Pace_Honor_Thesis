# Bite Lang - Pace Honors Thesis

This project closely follows the book "*Crafting Interpreters*" by **Robert Nystrom**. However, a few deviations from the book, such as syntaxes (on the bottom), and features, such as OOP concepts, are missing from the book in my implementation. The main reason for this is because the purpose of this language is targeted towards absolute beginners or high schoolers, who might find OOP difficult to grasp.

This language is a dynamically typed programming language and functions as a first-class value, with syntax inspired by **Lua**.

## Features

**To use Bite**
```
cargo run run <path to file>
```

**Gamified Environment**

To start the gamified environment run:
```
cargo run learn
```


## EBNF Grammar

#### Variables and Types
```
let string_var = "Some String"

let number_var_decimal = 123.34
let number_var = 123

let bool = true

let none_var = none
```

#### Conditionals
```
let i = 23
let is_true = true
if(i != 23 && is_true) then
	print(true)
else
	print(false)
end
```

#### Loops
```
while(true) do
	print("Don't do this")
end

for let i = 0, i < 10, i = i + 1 do
	print(i)
end
```

#### Functions
```
function fib(n) do
	if n <= 1 then
		return n
	end

	return fib(n - 2) + fib(n - 1)
end

let func_var = fib

func_var(10) // Gives 55
fib(10)      // Gives 55
```

#### Structs
```
struct Person {
	name,
	age,
	is_tall
}

let new_person = Person{}
new_person.name = "Kevin Liu"
new_person.age = 20
new_person.is_tall = false
```



## BNF Grammar

##### Program
```
Program -> Declaration 
Program -> Program EOF
```

##### Declaration
```
Declaration -> StructDecl | FuncDecl | VarDecl | Statements

StructDecl -> "struct" Identifier "{" Parameters "}"

FuncDecl -> "function" Function "\n"

VarDecl -> "let" Identifier "=" Expression "\n"
VarDecl -> "let" Identifier "\n"

Function -> Identifier "(" Parameters ")" BlockStmt
Parameters -> Identifier
Parameters -> Parameters "," Identifier
```


##### Statements
```
Statements -> ExprStmt | IfStmt | ForStmt | WhileStmt | ReturnStmt | BlockStmt 

ExprStmt -> Expression + "\n"

IfStmt -> "if" Expression "then" Declaration "end"
IfStmt -> "if" Expression "then" Declaration "else" Declaration "end"

ForStmt -> "for" "(" ForStmtBlockOne + ForStmtBlockTwo + ForStmtBlockThree ")" Statements
ForStmtBlockOne -> VarDecl | Expression | ","
ForStmtBlockTwo -> Expression "," | ","
ForStmtBlockThree -> Expression | ""proofmusic435

WhileStmt -> "while" "(" Expression ")" Statements 

ReturnStmt -> "return" Expression "\n"
ReturnStmt -> "return" "\n"

BlockStmt -> "do" "\n" Declaration "end" "\n"
```

##### Expressions
```
Expression -> Assignment

Assignment -> Identifier "=" ( Assignment | Logical-Or )
Assignment -> Call "." Identifier "=" ( Assignment | Logical-Or )

Logical-Or -> Logical-And
Logical-Or -> Logical-Or "or" Logical-And

Logical-And -> Equality
Logical-And -> Logical-And "and" Equality

Equality -> Comparison
Equality -> Equality ( "==" | "!=" ) Comparison

Comparison -> Term
Comparison -> Comparison ( "<" | ">" | "<=" | ">=" ) Term

Term -> Factor
Term -> Term ( "+" | "-" ) Factor

Factor -> Unary
Factor -> Factor ( "*" | "/" ) Unary

Unary -> ( "!" | "-" ) Call
Unary -> ( "!" | "-" ) Unary

Call -> Primary "(" Arguments ")"
Call -> Primary "." Identifier
Call -> Primary "{" "}"

Primary -> String | Number | Decimal | Boolean | None | "(" Expression ")" | Identifier

Arguments -> Expression
Arguments -> Arguments "," Expression
```

##### Typings
```
Identifier -> Alpha
Identifier -> Identifier + Number
String -> "\"" <any character> "\""
Boolean -> true, false
Digit -> 0-9
Number -> Number "." Number
Number -> Number
Number -> Digit
Alpha -> "a"-"z" | "A"-"Z" 
None -> none
```

