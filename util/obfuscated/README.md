# Utility: Secret Strings

This modules provides a utily method for obfuscating environment and secret values, preventing their display being leaked to the console.

### Example

```rust
use util_obfuscate::obfuscate;

fn main() {
    let password = obfuscate("super secret key");
    println!("Password: {password}"); // Password: ********
}
```
