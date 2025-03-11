pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use rstest::{rstest, fixture};
    use super::*;

    // test with fixture
    // 1. use rstest macro on test fn
    // 2. define fixture with fixture macro
    // 3. pass fixture as arg on test fn (type is return value of fixture)

    #[rstest]
    fn it_works(get24: i64) {
        let _result = add(2, 2);
        assert_eq!(get24, 4);
    }

    #[fixture]
    fn get24() -> i64 {
        4
    }
}
