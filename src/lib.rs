#[macro_export]
macro_rules! match_fallthrough_make_match {
    ($elem:expr, ($($pat:pat => $branch:expr)*)) => {{
        match $elem {$(
            $pat => $branch,
        )*}
    }}
}

#[macro_export]
macro_rules! match_fallthrough_make_loops {
    ($test:expr, $exit:expr, ($pat:pat => $branch:expr); ($($p:pat => $r:expr)*)) => {{
        'fallthrough: loop {
            loop {
                $crate::match_fallthrough_make_match!($test, ($pat => break 'fallthrough $($p => $r)*));
            }
            $exit
        }
        $branch
    }};
    ($test:expr, $exit:expr, ($pat:pat => $branch:expr, $($pu:pat => $bu:expr),+) ; ($($p:pat => $r:expr)*)) => {{
        'fallthrough: loop {
            loop {
                $crate::match_fallthrough_make_loops!($test, $exit, ($($pu => $bu),+) ; ($pat => break 'fallthrough $($p => $r)*));
                break 'fallthrough
            }
            $exit
        }
        $branch
    }};
}

#[macro_export]
macro_rules! match_fallthrough_reverse_branches {
    ($test:expr, ($pat:pat => $branch:expr); ($($p:pat => $r:expr)*)) => {{
        'exit: loop {
            $crate::match_fallthrough_make_loops!($test, break 'exit, ($pat => $branch, $($p => $r),*); ());
            break;
        }
    }};
    ($test:expr, ($pat:pat => $branch:expr, $($pu:pat => $bu:expr),+); ($($p:pat => $r:expr)*)) => ((
        $crate::match_fallthrough_reverse_branches!($test, ( $($pu => $bu),+ ) ; ($pat => $branch $($p => $r)*))
    ))
}

#[macro_export]
macro_rules! switch {
    ($test:expr, { $( $pat:pat => $branch:expr ),+ } ) => {{
        $crate::match_fallthrough_reverse_branches!($test, ($($pat => $branch),+); ())
    }};
    ($test:expr, { $( $pat:pat => $branch:expr, )+ } ) => {{
        $crate::match_fallthrough_reverse_branches!($test, ($($pat => $branch),+); ())
    }}
}

#[test]
#[allow(unreachable_code)]
fn it_works() {
    let mut x = 0;

    switch!(x, {
        0 => { assert_eq!(x,0); x = 1; },
        1 => { assert_eq!(x,1); x = 2; break; },
        _ => { panic!("Should not reach the default case"); },
    });
    assert_eq!(x, 2);
}
