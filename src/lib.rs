#[macro_export]
macro_rules! swh {
    (wacha $name:ident = $expression:expr) => (
        #[allow(unused_mut)]
        let mut $name = $expression;
    );

    ($matokeo:ident; kwa $i:ident katika $iterator:expr => kama $condition:expr) => (
        let mut $matokeo = vec![];

        for $i in $iterator {
            if $condition {
                $matokeo.push($i.clone())
            }
        }
    );

    // Functions
    (andika($expression:expr)) => (
        println!("{:?}", $expression);
    );    

    (urefu($expression:expr)) => (
        $expression.len()
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
}