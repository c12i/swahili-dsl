#[macro_export]
macro_rules! swh {
    (andika($expression:expr)) => {
        println!("{:?}", $expression);
    };    

    (wacha $name:ident = $expression:expr) => {
        #[allow(unused_mut)]
        let mut $name = $expression;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn andika() {
        swh!(
            andika("Habari Duinia")
        );
        assert!(true);
    }

    #[test]
    fn wacha() {
        swh!(
            wacha jina = 2020
        );
        jina += 10;
        assert_eq!(jina, 2030);
    }
}