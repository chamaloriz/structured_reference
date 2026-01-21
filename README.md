
# structured data for banking.

ex: +++ 010/0001/02540 +++

```rust
use structured_reference::prelude::*;

let data = StructuredData::new(10, 1, 26).unwrap();
let bank_format = data.to_bank_format();
let digits = data.to_digits();

println!("{} {}", bank_format, digits);
```