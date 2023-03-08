# Interpreter AD

Custom interpreter to use custom programming language.

File extension is `.iad`

## TODO

- You can extend the token struct to add necessary information where the token is. For example, another property to store what is the column number of the token. Remember that, we have already implemented the line number.

## Grammar

Here is the grammar:

```plaintext
expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "null" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;
```
