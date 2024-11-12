## i hate `#[serde(rename = "")]`-ing everything

### What does it do
Slap `#[serde_infer]` on top of whatever struct you're using.  
You'll get all the most common alternative casings provided only on deserialization for free, through heck.  
It'll probably save you a few minutes here and there.

### I want to rename to a specific case outgoing-ly
`#[serde_infer(outgoing = "case")]`

### What cases are supported
All [heck](https://docs.rs/heck) cases, and upper/lowercase:
- "kebab"
- "lower_camel"
- "pascal"
- "shouty_kebab"
- "shouty_snake"
- "snake"
- "title"
- "train"
- "upper_camel"
- "uppercase" | "upper"
- "lowercase" | "lower"