<h1 align="center">Rust Derive Preset</h1>
<p align="center">
    <a href="https://github.com/dekirisu/derive_preset" style="position:relative"><img src="https://img.shields.io/badge/github-dekirisu/derive_preset-ee6677"></a>
    <a href="https://crates.io/crates/derive_preset" style="position:relative"><img src="https://img.shields.io/crates/v/derive_preset"></a>
</p>

Create derive presets inside a proc-macro crate - with generated docs attached to view which derives are included!

## Inside a Proc-Macro Crate:
```rust
derive_preset::create!{
    hashable "PartialEq,Eq,Hash,Clone,Debug"
    serde "Serialize,Deserialize,Clone"
}
```

## Use it in another Crate:
```rust
use my_proc_crate::*;

#[hashable(Clone,Default)]
struct Id(u32);

#[serde(Debug)]
struct Data(f32);
```

---
### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
