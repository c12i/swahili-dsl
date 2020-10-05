#[macro_export]
macro_rules! swh {
    (andika($expression:expr)) => {{
        println!("{:?}", $expression);
    }};    

    (wacha $name:ident = $expression:expr) => {{
        let mut $name = $expression;
    
    }};
}