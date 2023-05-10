#[macro_export]
macro_rules! call_auto_hrl_symbol {
    (
        $( $lib_variable:ident ).*, $symbol:ident($( $arg:expr ),*)
            : $( $modifiers:ident )* $( $abi:literal )? fn($( $param_type:ty ),*)$( -> $return_type:ty)?
    ) => {
        $($lib_variable).*
            .symbol_op::<$($modifiers )*$($abi )?fn($($param_type),*)$( -> $return_type)?, _>(
                stringify!($symbol), |$symbol| unsafe { $symbol($($arg),*) }
            )
    };
}

#[macro_export]
macro_rules! auto_hrl_symbol_value {
    ($( $lib_variable:ident ).*, $symbol:ident: $type:ty) => {
        $($lib_variable).*
            .symbol_op::<*const $type, _>(stringify!($symbol), |&$symbol| unsafe {
                *$symbol
            })
    };
}

#[macro_export]
macro_rules! set_auto_hrl_symbol_value {
    ($( $lib_variable:ident ).*, $symbol:ident: $type:ty = $value:expr) => {
        $($lib_variable).*
            .symbol_op::<*mut $type, _>(stringify!($symbol), |&$symbol| unsafe {
                *$symbol = $value;
            })
    };
}
