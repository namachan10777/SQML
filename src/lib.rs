#[cfg(test)]
extern crate test_case;

pub fn echo(s: &str) -> &str {
    s
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("WEHRE")]
    #[test_case("READ")]
    fn echo_tests(query: &str) {
        assert_eq!(echo(query), query);
    }
}
