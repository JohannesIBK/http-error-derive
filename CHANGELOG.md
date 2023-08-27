# Changelog

### v2.0.1
- fix tests and documentation

### v2.0.0
- Removed default_code
- Added error, an optional u16 field, accessible via `http_error()`
- `http_code()` now returns an Option<u16> instead of u16