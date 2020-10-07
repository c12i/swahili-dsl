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

    (kamusi -> $($k:expr => $v:expr),*) => ({
        let mut hm = std::collections::HashMap::new();
        $(
            hm.insert($k, $v);
        )*
        hm
    });

    // Ternary operator
    ($condition:expr => $expression:expr ; $alternative:expr) => (
        if $condition {
            $expression
        } else {
            $alternative
        }
    );

    // Functions
    (andika($expression:expr)) => (
        println!("{:?}", $expression);
    );    

    (urefu($expression:expr)) => (
        $expression.len()
    );

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
        swh!(
            andika("Habari Duinia")
        );
        assert!(true);
    }

    #[test]
    fn variables() {
        swh!(
            wacha jina = 2020
        );
        jina += 10;
        assert_eq!(jina, 2030);
    }

    #[test]
    fn list_comprehension() {
        swh!(
           matokeo; kwa n katika 0..=10 => kama n%2 == 0
        );

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
        swh!(wacha hm = swh!(kamusi -> 
            "id" => "#12",
            "jina" => "Juma"
        ));
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
    fn ternary() {
        swh!(wacha swala = swh!(kweli));
        swh!(swala => swh!(andika("Kweli")) ; swh!(andika("Uwongo")));
        assert!(swala);
    }
}