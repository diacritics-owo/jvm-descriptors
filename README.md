# jvm-descriptors

## Examples

```rust
"Ljava/lang/String;".parse::<Type>();

Method {
  name: "hello".to_string(),
  parameters: vec![],
  return_type: None,
}.to_string();
```

## Notes

Note that this crate is somewhat lenient—the goal is to do basic parsing and no other validation. If you want to make sure that e.g. a given package path has at least two parts, validate it on your own. Additionally, absolutely no validation is done when serialising to a string.
