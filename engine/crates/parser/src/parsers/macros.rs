#[macro_export]
macro_rules! collect {
    ($( $res:ident ),+) => {
        collect!(ret | exc | $($res),* )
    };
    (ret | $name:tt | $( $res:ident ),+) => {
        collect!($name, $($res),*);

        if !$name.is_empty() {
            return combine::unexpected_any(combine::error::Info::Format($name)).right();
        };
    };
    ($name:tt | $( $res:ident ),+) => {
        let mut $name = variants::Errors::new();
        $(
        if let Err(err) = &$res {
            $name += err.clone();
        }
        )*
    };
}
