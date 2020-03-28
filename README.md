# hypua

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/kiwiyou/hypua/Rust?style=flat-square)](https://github.com/kiwiyou/hypua/actions?query=workflow%3ARust)
[![License](https://img.shields.io/github/license/kiwiyou/hypua?style=flat-square)](https://github.com/kiwiyou/hypua/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/hypua?style=flat-square)](https://crates.io/crates/hypua)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/kiwiyou/hypua?style=flat-square)

한양 PUA 코드로 인코딩된 옛한글을 첫가끝(IPF) 방식으로 변환하는 라이브러리

[문서](https://docs.rs/hypua)

## 사용법

- `Cargo.toml`의 `dependencies` 항목에 `hypua`를 추가합니다.

```toml
[dependencies]
hypua = "0.2.0"
```

## 사용 예

[example/hunmin.rs](https://github.com/kiwiyou/hypua/blob/master/example/hunmin.rs):

```rust
use hypua::to_ipf_string;

fn main() {
    let text = "이런 젼로 어린 百姓이 니르고져  배 이셔도.";
    println!("{}", to_ipf_string(text));
}
```
