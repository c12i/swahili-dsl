//!## swahili-dsl 
//!An attempt at creating a DSL. A DSL is a mini "language" embedded in a Rust macro. Made for educational purposes.
//! 
//!Heavily influenced by [swahili-lang](https://github.com/malcolmkiano/swahili) and [macro-lisp](https://github.com/JunSuzukiJapan/macro-lisp)

//!## Examples

//!```rs
//!// declaring variables
//!swh!(wacha jina = 2020);
//! 
//!// booleans
//!swh!(wacha swala = swh!(kweli));
//!swh!(wacha swala = swh!(uwongo));
//! 
//!// list comprehensions
//!swh!(matokeo; kwa n katika swh!(masafa(0,10)) => kama n%2 == 0);
//!
//! // Declaring functions (very rudimental)
//!swh!(shughuli ongeza(n: u32) -> u32 {
//!    rudisha n + 11
//!});
//!swh!(wacha hesabu = ongeza(1));
//!  
//!// inbuilt functions
//!swh!(wacha urefu = swh!(urefu(vec![1,2,4])));
//!swh!(andika("Habari Duinia"));
//!swh!(wacha orodha = swh!(masafa(1, 5)));
//! 
//!// collections
//!swh!(wacha l = swh!(orodha -> [1,2,4]));
//!swh!(wacha hm = swh!(kamusi -> {
//!     "id" => "#12",
//!     "jina" => "Juma"
//!}));
//! 
//!// arithmetic operations
//!swh!(wacha hesabu = swh!(suluhisha 4 * 4));
//!swh!(wacha hesabu = swh!(suluhisha (12/4) * (16/4)));
//!swh!(wacha hesabu = swh!(suluhisha 4 * 4, suluhisha 4 * 2));
//! 
//!// ternary operator
//!swh!(wacha swala = swh!(kweli));
//!swh!(swala => {
//!     swh!(andika("Kweli")) ;
//!     swh!(andika("Uwongo"))
//!});
//!```


#[macro_export]
macro_rules! swh {
    // Assignment operators
    (wacha $name:ident = $expression:expr) => (
        #[allow(unused_mut)]
        let mut $name = $expression;
    );

    (wacha_mut $name:ident = $expression:expr) => (
        #[allow(unused_mut)]
        let mut $name = $expression;
    );

    (suluhisha $expression:expr) => (
        $expression as usize
    );

    (suluhisha $e:expr, $(suluhisha $es:expr),+) => {
        swh!(suluhisha $e) + swh!($(suluhisha $es),+)
    };

    // Comprehensions
    ($matokeo:ident; kwa $i:ident katika $iterator:expr => kama $condition:expr) => (
        swh!(wacha_mut $matokeo = Vec::new()); 
        for $i in $iterator {
            if $condition {
                $matokeo.push($i.clone())
            }
        }
    );
    
    // Collections
    (orodha -> [$($val:expr),*]) => ({
            let mut vec = Vec::new();
            $(
                vec.push($val);
            )*
            vec
    });

    (kamusi -> {$($k:expr => $v:expr),*}) => ({
        let mut hm = std::collections::HashMap::new();
        $(
            hm.insert($k, $v);
        )*
        hm
    });

    // Ternary operator
    ($condition:expr => {$expression:expr ; $alternative:expr}) => (
        if $condition {
            $expression
        } else {
            $alternative
        }
    );

    // Declaring functions
    (shughuli $f:ident($($arg:ident: $t:ty),*) -> $ret:ty { rudisha $r:expr }) => (
        fn $f($($arg: $t)*) -> $ret {
            $r
        }
    );

    // "Inbuilt" functions
    (andika($expression:expr)) => (
        println!("{:?}", $expression);
    );    

    (urefu($expression:expr)) => (
        $expression.len()
    );

    (masafa($start:expr, $stop:expr)) => ({
        let vector: Vec<_> = ($start..=$stop).into_iter().collect();
        vector
    });

    // Statements
    (kweli) => (
        true
    );

    (uwongo) => (
        false
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_statement() {
        // hello world!
        swh!(andika("Habari Duinia!"));
        assert!(true);
    }

    #[test]
    fn variables() {
        swh!(wacha jina = 2020);
        jina += 10;
        assert_eq!(jina, 2030);
    }

    #[test]
    fn list_comprehension() {
        swh!(matokeo; kwa n katika swh!(masafa(0,10)) => kama n%2 == 0);
        assert_eq!(matokeo, vec![0,2,4,6,8,10]);
    }

    #[test]
    fn len() {
        let l = swh!(
            urefu(vec![1,2,4])
        );
        assert_eq!(3, l);
    }

    #[test]
    fn list() {
        swh!(wacha l = swh!(orodha -> [1,2,4]));
        assert_eq!(l, vec![1,2,4]);
    }

    #[test]
    fn map() {
        swh!(wacha hm = swh!(kamusi -> {
            "id" => "#12",
            "jina" => "Juma"
        }));
        let mut rhs = std::collections::HashMap::new();
        rhs.insert("id", "#12");
        rhs.insert("jina", "Juma");
        assert_eq!(hm, rhs);
    }

    #[test]
    fn eval() {
        swh!(wacha hesabu = swh!(suluhisha 4 * 4));
        assert_eq!(hesabu, 16);
    }

    #[test]
    fn variadic() {
        swh!(wacha hesabu = swh!(suluhisha 4 * 4, suluhisha 4 * 2));
        assert_eq!(hesabu, 24);
    }

    #[test]
    fn ternary() {
        swh!(wacha swala = swh!(kweli));
        // similar to `true ? ... : ...` where the ? is replaced with a at arrow and : by a semi-colon
        swh!(swala => {
            swh!(andika("Kweli")) ;
            swh!(andika("Uwongo"))
        });
        assert!(swala);
    }

    #[test]
    fn range() {
        swh!(wacha orodha = swh!(masafa(1, 5)));
        swh!(andika(orodha));
        assert_eq!(vec![1,2,3,4,5], orodha);
    }


    #[test]
    fn declaring_functions() {
        swh!(shughuli ongeza(n: u32) -> u32 {
            rudisha n + 11
        });
        swh!(wacha hesabu = ongeza(1));
        assert_eq!(hesabu, 12);
    }
}