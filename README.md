## Background

The goal of this project is to create a complete lexer for the Ruby language
using parser combinators.

## Differences from MRI

 * Control characters can be recursively escaped:
   ```ruby
   "\C-\C-\\n" # => "\n"
   ```

## Implementation Notes

#### Expressions / Statements / Groups

 * There is no semantic difference between an expression and a statement in this lib
 * `Token::Block` contains a list of statements (each item is considered a statement)
 * `Token::Expression` contains a list of tokens that make up an individual statement
 * These tokens may be nested via the use of parenthesis:
   ```ruby
   (2 + (4 - 8)) * 5
   ```

#### Lexing partial inputs

The `Input` type can be extended to track a boolean field that denotes whether
the lexer's input is complete or partial (such as within IRB's REPL). Combinators
that may be partially completed (such as open strings, arrays, etc.) can additionally
return a `Token::IncompleteInput` on the end of their token stream in partial mode
to signal that the token has not been completed by the end of the user's input.

I'm not yet sure if start / end tokens should be used for complex objects, but
they may be helpful when dealing with partial inputs.
