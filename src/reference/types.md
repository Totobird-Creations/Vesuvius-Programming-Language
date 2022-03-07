#### Normal Types
Type Name            | Rust Equivalent
---------------------|------------------------
` Bool              `|` bool                 `
` Int               `|` i64                  `
` RInt<I, A>        `|  I < `i64` < A
` Float             `|` f64                  `
` RInt<I, A>        `|  I < `f64` < A
` Char              `|` char                 `
` Str               `|` String               `
` Tuple<T, ...>     `|` (T, ...)             `
` Array<T, L>       `|` [T, L]               `
` Func<<A, ...>, R> `|` fn(A, ...) -> R      `
` Point<T>          `|  Pointer of type T

#### User Defined Types
Type Name            | Rust Equivalent
---------------------|------------------------
  Class              |  Structure
  Enum               |  Enumeration


#### Build Types
Type Name            | Rust Equivalent
---------------------|------------------------
  Builder            |  -
  Build              |  Built in objects


#### Miscellanious Types
Type Name            | Rust Equivalent
---------------------|------------------------
` Void              `|  `()`
` Crash             `|  Never


#### Internal Types
Type Name            | Rust Equivalent
---------------------|------------------------
  Inferred           |  Inferred
