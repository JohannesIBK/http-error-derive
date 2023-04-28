# Derive for creating easy http api errors
## Example

```rust
use http_error_derive::HttpError;

#[derive(HttpError)]
#[http(default_code = 501)]
enum ApiError {
    #[http(code = 401, message = "You must be logged in to access this resource")]
    Unauthorized,
    #[http(code = 403, message = "You have no permission to access this resource")]
    Forbidden,
}

fn main() {
    println!("{}", ApiError::Forbidden.http_code()); // 403u16
    println!("{:?}", ApiError::Forbidden.http_message()); // Some("You have no permission to access this resource")
}
```
