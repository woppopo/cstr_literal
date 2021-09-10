# cstr_literal

## Usage

```
[dependencies]
cstr_literal = { git = "https://github.com/woppopo/cstr_literal" }
```

```rust
use std::ffi::CStr;
use cstr_literal::cstr;

const STRING: &'static CStr = cstr!("C STRING");
```