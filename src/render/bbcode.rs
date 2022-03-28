#[macro_export]
macro_rules! bbcode {
    (in {{$buf:ident}}; $($body:tt)*) => {{
        $buf.reserve(stringify!($($body)*).len());
        $crate::parse_bbcode!($buf, $($body)*);
    }};
}

#[macro_export]
macro_rules! parse_bbcode {
    /* ----------------------------- Utility: Write ----------------------------- */

    (@write $buf:ident, $text:expr) => {{
        use std::fmt::Write;
        write!($buf, $text).unwrap();
    }};

    /* ------------------------- Utility: Dynamic Write ------------------------- */

    (@writedyn $buf:ident, $expr:expr) => {{
        use std::fmt::Write;
        write!($buf, "{}", $expr).unwrap();
    }};

    /* ----------------------- Utility: Start of Open Tag ----------------------- */

    (@openstart $buf:ident, $tag:ident) => {
        $crate::parse_bbcode!(@write $buf, concat!("[", stringify!($tag)));
    };

    /* ----------------------- Utility: Value of Open Tag ----------------------- */

    (@openval $buf:ident, $val:literal) => {
        $crate::parse_bbcode!(@write $buf, concat!("=\"", $val, "\""));
    };

    /* ------------------- Utility: Dynamic Value of Open Tag ------------------- */

    (@openvaldyn $buf:ident, $val:expr) => {
        $crate::parse_bbcode!(@write $buf, "=\"");
        $crate::parse_bbcode!(@writedyn $buf, $val);
        $crate::parse_bbcode!(@write $buf, "\"");
    };

    /* ------------------------ Utility: End of Open Tag ------------------------ */

    (@openend $buf:ident) => {
        $crate::parse_bbcode!(@write $buf, "]");
    };

    /* --------------------------- Utility: Close Tag --------------------------- */

    (@close $buf:ident, $tag:ident) => {
        $crate::parse_bbcode!(@write $buf, concat!("[/", stringify!($tag), "]"));
    };

    (@defer $buf:ident, $($body:tt)*) => {
        |buf: &mut String| {
            buf.reserve(stringify!($($body)*).len());
            $crate::parse_bbcode!(buf, $($body)*);
        }
    };

    /* ---------------------------------- Empty --------------------------------- */

    ($buf:ident, ) => {

    };

    /* ----------------------------- Yield Statement ---------------------------- */

    ($buf:ident, yield {{$body:expr}}; $($next:tt)*) => {
        $body($buf);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ------------------------------ For Statement ----------------------------- */

    ($buf:ident, for $var:ident in {{$iter:expr}} {$($body:tt)*} $($next:tt)*) => {
        for $var in $iter {
            $crate::parse_bbcode!($buf, $($body)*);
        }

        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ---------------------------- If/Else Statement --------------------------- */

    ($buf:ident, if {{$cond:expr}} {$($body:tt)*} else {$($ebody:tt)*} $($next:tt)*) => {
        if $cond {
            $crate::parse_bbcode!($buf, $($body)*);
        } else {
            $crate::parse_bbcode!($buf, $($ebody)*);
        }

        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ------------------------------ If Statement ------------------------------ */

    ($buf:ident, if {{$cond:expr}} {$($body:tt)*} $($next:tt)*) => {
        if $cond {
            $crate::parse_bbcode!($buf, $($body)*);
        }

        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ------------------------- Error: Unmatched Yield ------------------------- */

    ($buf:ident, yield $($_:tt)*) => {
        compile_error!("Unexpected keyword: `yield`!");
    };

    /* -------------------------- Error: Unmatched For -------------------------- */

    ($buf:ident, for $($_:tt)*) => {
        compile_error!("Unexpected keyword: `for`!");
    };

    /* --------------------------- Error: Unmatched If -------------------------- */

    ($buf:ident, if $($_:tt)*) => {
        compile_error!("Unexpected keyword: `if`!");
    };

    /* -------------------------- Error: Unmatched Else ------------------------- */

    ($buf:ident, else $($_:tt)*) => {
        compile_error!("Unexpected keyword: `else`!");
    };

    /* ---------------------------- Literal Statement --------------------------- */


    ($buf:ident, $lit:literal; $($next:tt)*) => {
        $crate::parse_bbcode!(@write $buf, $lit);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ---------------------------- Escaped Statement --------------------------- */

    ($buf:ident, {{$expr:expr}}; $($next:tt)*) => {
        $crate::parse_bbcode!(@writedyn $buf, $expr);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ------------------------- Tag with Escaped Value ------------------------- */

    ($buf:ident, $tag:ident {{$val:expr}}; $($next:tt)*) => {
        $crate::parse_bbcode!(@openstart $buf, $tag);
        $crate::parse_bbcode!(@openvaldyn $buf, $val);
        $crate::parse_bbcode!(@openend $buf);
        $crate::parse_bbcode!(@close $buf, $tag);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ----------------------------- Tag with Value ----------------------------- */

    ($buf:ident, $tag:ident $val:literal; $($next:tt)*) => {
        $crate::parse_bbcode!(@openstart $buf, $tag);
        $crate::parse_bbcode!(@openval $buf, $val);
        $crate::parse_bbcode!(@openend $buf);
        $crate::parse_bbcode!(@close $buf, $tag);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* -------------------------------- Empty Tag ------------------------------- */

    ($buf:ident, $tag:ident; $($next:tt)*) => {
        $crate::parse_bbcode!(@openstart $buf, $tag);
        $crate::parse_bbcode!(@openend $buf);
        $crate::parse_bbcode!(@close $buf, $tag);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ------------------- Tag with Escaped Value and Children ------------------ */

    ($buf:ident, $tag:ident {{$val:expr}} {$($body:tt)*} $($next:tt)*) => {
        $crate::parse_bbcode!(@openstart $buf, $tag);
        $crate::parse_bbcode!(@openvaldyn $buf, $val);
        $crate::parse_bbcode!(@openend $buf);
        $crate::parse_bbcode!($buf, $($body)*);
        $crate::parse_bbcode!(@close $buf, $tag);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ----------------------- Tag with Value and Children ---------------------- */

    ($buf:ident, $tag:ident $val:literal {$($body:tt)*} $($next:tt)*) => {
        $crate::parse_bbcode!(@openstart $buf, $tag);
        $crate::parse_bbcode!(@openval $buf, $val);
        $crate::parse_bbcode!(@openend $buf);
        $crate::parse_bbcode!($buf, $($body)*);
        $crate::parse_bbcode!(@close $buf, $tag);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ---------------------------- Tag with Children --------------------------- */

    ($buf:ident, $tag:ident {$($body:tt)*} $($next:tt)*) => {
        $crate::parse_bbcode!(@openstart $buf, $tag);
        $crate::parse_bbcode!(@openend $buf);
        $crate::parse_bbcode!($buf, $($body)*);
        $crate::parse_bbcode!(@close $buf, $tag);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ---------------------- Component with Escaped Value ---------------------- */

    ($buf:ident, do $cmp:ident {{$val:expr}}; $($next:tt)*) => {
        $cmp(&mut $buf, $val);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* -------------------------- Component with Value -------------------------- */

    ($buf:ident, do $cmp:ident $val:literal; $($next:tt)*) => {
        $cmp(&mut $buf, $val);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ----------------------------- Empty Component ---------------------------- */

    ($buf:ident, do $cmp:ident; $($next:tt)*) => {
        $cmp(&mut $buf);
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ---------------- Component with Escaped Value and Children --------------- */

    ($buf:ident, do $cmp:ident {{$val:expr}} {$($body:tt)*} $($next:tt)*) => {
        $cmp(&mut $buf, $val, $crate::parse_bbcode!(@defer $buf, $($body)*));
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* -------------------- Component with Value and Children ------------------- */

    ($buf:ident, do $cmp:ident $val:literal {$($body:tt)*} $($next:tt)*) => {
        $cmp(&mut $buf, $val, $crate::parse_bbcode!(@defer $buf, $($body)*));
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ------------------------- Component with Children ------------------------ */

    ($buf:ident, do $cmp:ident {$($body:tt)*} $($next:tt)*) => {
        $cmp(&mut $buf, $crate::parse_bbcode!(@defer $buf, $($body)*));
        $crate::parse_bbcode!($buf, $($next)*);
    };

    /* ----------------------------- Error: Fallback ---------------------------- */

    ($buf:ident, $($unexpected:tt)*) => {
        compile_error!(concat!("Unexpected ", $($unexpected)*, "!"));
    };
}
