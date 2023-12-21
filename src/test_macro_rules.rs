#[macro_export]
macro_rules! assert_rows_eq {
    ( $left:expr, $( [ $($x:expr),* ] ),* ) => {
        let mut iterator = $left;
        $(
            assert_eq!(iterator.next(), Some(vec![$($x.to_string()),*]));
        )*
        assert_eq!(iterator.next(), None);
    };
}

#[macro_export]
macro_rules! rows {
    ( $( [ $($x:expr),* ] ),* ) => {
        vec![$(vec![$($x.to_string()),*]),*].into_iter()
    }
}

#[macro_export]
macro_rules! sheet {
    ( $( [ $($x:expr),* ] ),* ) => {
        VecSheet { rows: vec![$(vec![$($x.to_string()),*]),*].into_iter() }
    }
}
