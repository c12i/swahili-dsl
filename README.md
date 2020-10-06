# swahili-dsl
![Version](https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> An attempt of creating a DSL. A DSL is a mini "language" embedded in a Rust macro. Will continue to expand on it as I practice my declarative macro skills.
>
> Heavily influenced by [swahili-lang](https://github.com/malcolmkiano/swahili) and [macro-lisp](https://github.com/JunSuzukiJapan/macro-lisp)

## Examples

```rs
// declaring variables
swh!(wacha jina = 2020);

// list comprehensions
swh!(matokeo; kwa n katika 0..=10 => kama n%2 == 0);

// inbuilt functions
let l = swh!(urefu(vec![1,2,4]));
swh!(andika("Habari Duinia"));

// collections
let l = swh!(orodha -> [1,2,4]);
let hm = swh!(kamusi -> 
            "id" => "#12",
            "jina" => "Juma");

// arithmetic operations
swh!(wacha hesabu = swh!(suluhisha 4 * 4));
swh!(wacha hesabu = swh!(suluhisha (12/4) * (16/4)));
```

## Run tests

```sh
cargo test
```

## 📝 License

Copyright © 2020 [collinsmuriuki](https://github.com/collinsmuriuki).

This project is [MIT](LICENSE) licensed.
