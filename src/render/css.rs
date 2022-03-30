#[macro_export]
macro_rules! css {

    /* ---------------------------------- Empty --------------------------------- */

    () => {

    };

    /* ------------------------------- Empty Rule ------------------------------- */

    (@rule $buf:ident, ) => {

    };

    /* ---------------------------- If/Else Statement --------------------------- */

    (@rule $buf:ident, if {{$subj:expr}} { $($body:tt)* } else { $($ebody:tt)* } $($next:tt)*) => {
        if $subj {
            $crate::css!(@rule $buf, $($body)*);
        } else {
            $crate::css!(@rule $buf, $($ebody)*);
        }

        $crate::css!(@rule $buf, $($next)*);
    };

    /* ------------------------------ If Statement ------------------------------ */

    (@rule $buf:ident, if {{$subj:expr}} { $($body:tt)*} $($next:tt)*) => {
        if $subj {
            $crate::css!(@rule $buf, $($body)*);
        }

        $crate::css!(@rule $buf, $($next)*);
    };

    /* --------------------------- Error: Unmatched If -------------------------- */

    (@rule $buf:ident, if $($_:tt)*) => {
        compile_error!("Unexpected keyword: `if`!");
    };

    /* -------------------------- Error: Unmatched Else ------------------------- */

    (@rule $buf:ident, else $($_:tt)*) => {
        compile_error!("Unexpected keyword: `else`!");
    };

    /* ------------------------------ Literal Rule ------------------------------ */

    (@rule $buf:ident, $prop:ident: $val:literal; $($next:tt)*) => {
        {
            use heck::ToKebabCase;
            use std::fmt::Write;

            let prop = stringify!($prop).to_kebab_case();
            write!($buf, concat!("{}: ", $val, ";"), prop).unwrap();
        }

        $crate::css!(@rule $buf, $($next)*);
    };

    /* ------------------------------ Escaped Rule ------------------------------ */

    (@rule $buf:ident, $prop:ident: {{$val:expr}}; $($next:tt)*) => {
        {
            use heck::ToKebabCase;
            use std::fmt::Write;

            let prop = stringify!($prop).to_kebab_case();
            write!($buf, "{}: {};", prop, $val).unwrap();
        }

        $crate::css!(@rule $buf, $($next)*);
    };

    /* ---------------------------------- Block --------------------------------- */

    ($var:ident { $($body:tt)* } $($next:tt)*) => {
        let $var = {
            let mut _buf = String::with_capacity(stringify!($($body)*).len());

            $crate::css!(@rule _buf, $($body)*);

            _buf
        };

        $crate::css!($($next)*);
    };
}
