#### Normal Types
Type Name            | Rust Equivalent        | Vesuvius Syntax
---------------------|------------------------|-----------------------------------------------------------------------------------------------------
 `Bool`              | `bool`                 | N/A
 `Int`               | `i64`                  | INTEGER
 `RInt<I, A>`        |  I < `i64` < A         | INTEGER
 `IntRange`          | `i64..i64`             | INTEGER DOUBLEPERIOD INTEGER
 `Float`             | `f64`                  | FLOAT
 `RFloat<I, A>`      |  I < `f64` < A         | FLOAT
 `FloatRange`        | `f64..f64`             | FLOAT DOUBLEPERIOD FLOAT
 `Char`              | `char`                 | CHARACTER
 `String`            | `String`               | STRING
 `Tuple<T, ...>`     | `(T, ...)`             | LCARAT {expression {COMMA expression}*}? RCARAT
 `List<T>`           | `Vec<T>`               | LBRACKET {expression {COMMA expression}*}? RBRACKET PERIOD IDENTIFIER("to_list") LPARENTHESIS RPARENTHESIS
 `Array<T, L>`       | `[T, L]`               | LBRACKET {expression {COMMA expression}*}? RBRACKET
 `Dict<K, V>`        | `HashMap<K, V>`        | LBRACE {literal COLON expression {literal COLON expression}*} RBRACE
 `Func<<A, ...>, R>` | `fn(A, ...) -> R`      | IDENTIFIER("func") IDENTIFIER LPAREN {IDENTIFIER COLON TYPE {COMMA IDENTIFIER COLON TYPE}*}? RPAREN
 `Pointer<T>`        |  Pointer of type T     | N/A

#### User Defined Types
Type Name            | Rust Equivalent        | Vesuvius Syntax
---------------------|------------------------|-----------------------------------------------------------------------------------------------------
  Class              |  Structure             | IDENTIFIER("class") IDENTIFIER (IDENTIFIER("extends") IDENTIFIER (COMMA IDENTIFIER)*)?
  Enum               |  Enumeration           | IDENTIFIER("enum") IDENTIFIER LBRACE (IDENTIFIER (COMMA IDENTIFIER)*)? RBRACE


#### External Types
Type Name            | Rust Equivalent        | Vesuvius Syntax
---------------------|------------------------|-----------------------------------------------------------------------------------------------------
  Module             |  `mod` or `use`        | (IDENTIFIER("extern") | IDENTIFIER("use")) IDENTIFIER


#### Miscellanious Types
Type Name            | Rust Equivalent        | Vesuvius Syntax
---------------------|------------------------|-----------------------------------------------------------------------------------------------------
 `Void`              |  `()`                  | N/A
 `Crash`             |  Never                 | N/A


#### Internal Types
Type Name            | Rust Equivalent        | Vesuvius Syntax
---------------------|------------------------|-----------------------------------------------------------------------------------------------------
  Inferred           |  Inferred              | N/A
