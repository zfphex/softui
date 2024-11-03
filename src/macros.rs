#[macro_export]
macro_rules! builder {
    ($($variable:ident: $type:ty),*) => {
        $(
            pub fn $variable(mut self, $variable: $type) -> Self {
                self.$variable = $variable;
                self
            }
        )*
    };
}

#[macro_export]
macro_rules! count_expr {
    () => { 0 };
    ($first:expr $(, $rest:expr)*) => {
        1 + count_expr!($($rest),*)
    };
}

// #[macro_export]
// macro_rules! count_idents {
//     ( $( $i:ident ),* ) => {
//         ${count($i)}
//     };
// }
