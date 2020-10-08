# swahili-dsl
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/swahili-dsl)](https://crates.io/crates/swahili-dsl)
[![Travis CI](https://travis-ci.com/collinsmuriuki/swahili-dsl.svg?branch=master)]("https://travis-ci.com/collinsmuriuki/swahili-dsl)

> An attempt at creating a Swahili-based DSL. A DSL is a mini "language" embedded in a Rust macro. Made for educational purposes.
>
> Heavily influenced by [swahili-lang](https://github.com/malcolmkiano/swahili) and [macro-lisp](https://github.com/JunSuzukiJapan/macro-lisp)

## Examples

```rs
// declaring variables
swh!(wacha jina = 2020);

// booleans
swh!(wacha swala = swh!(kweli));
swh!(wacha swala = swh!(uwongo));

// list comprehensions
swh!(matokeo; kwa n katika swh!(masafa(0,10)) => kama n%2 == 0);

// inbuilt functions
swh!(wacha urefu = swh!(urefu(vec![1,2,4])));
swh!(andika("Habari Duinia"));
swh!(wacha orodha = swh!(masafa(1, 5)));

// collections
swh!(wacha l = swh!(orodha -> [1,2,4]));
swh!(wacha hm = swh!(kamusi -> 
            "id" => "#12",
            "jina" => "Juma"));

// arithmetic operations
swh!(wacha hesabu = swh!(suluhisha 4 * 4));
swh!(wacha hesabu = swh!(suluhisha (12/4) * (16/4)));
swh!(wacha hesabu = swh!(suluhisha 4 * 4, suluhisha 4 * 2));

// ternary operator
swh!(wacha swala = swh!(kweli));
swh!(swala => {
    swh!(andika("Kweli")) ;
    swh!(andika("Uwongo"))
});
```

## Run tests

```sh
cargo test -- --show-output
```

This project is [MIT](LICENSE) licensed.
