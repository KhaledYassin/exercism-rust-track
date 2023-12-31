#[macro_export(local_inner_macros)]
macro_rules! hashmap {
    {$($key:expr => $value:expr),*} => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )*
            m
        };
     };

     {$($key:expr => $value:expr,)+} => {
        {
            hashmap!($($key => $value),+)
        };
     };
}
