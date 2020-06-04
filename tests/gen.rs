/// ```
/// mod others {
///     pub fn top_fn() {}
///
///     mod one {
///         pub fn one_fn() {}
///     }
///
///     mod two {
///         pub fn two_fn() {}
///     }
/// }
/// ```
#[mod_gen::expand(path = "tests/others")]
mod others {}

#[test]
fn correctly_expanded() {
    others::top_fn();
    others::one::one_fn();
    others::two::two_fn();
}
