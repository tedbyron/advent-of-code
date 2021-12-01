#![macro_use]

/// Read a file `$f`, split on pattern `$s`, and parse each item as a `$t`.
///
/// ```
/// split_parse!($f:literal, $s:expr, $t:ty) -> Vec<$t>
/// ```
macro_rules! split_parse {
    ($f:literal, $s:expr, $t:ty) => {
        include_str!($f)
            .split($s)
            .filter_map(|s| s.parse::<$t>().ok())
            .collect::<Vec<$t>>()
    };
}
