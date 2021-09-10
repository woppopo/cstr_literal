# cstr-literal

## Usage

```
[dependencies]
cstr-literal = { git = "https://github.com/woppopo/cstr-literal" }
```

```rust
use std::ffi::CStr;
use cstr_literal::cstr;

const STRING: &'static CStr = cstr!("C STRING");
```