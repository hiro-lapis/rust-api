use mockall::predicate::*;
use mockall::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn concat(left: &String, right: &String) -> String {
    left.to_owned() + right
}

pub fn main() {
    let mut mock = MockSampleStruct::new();
    // set mock return value of add_ten function
    mock.expect_add_ten().returning(|v|  v +10);

    assert_eq!(10, test_struct(&mock));
}

#[automock]
trait SampleStruct {
    fn add_ten(&self, x:u32) -> u32;
}

// normally, traits cannot be an argument type, but dynamically resolved by mockall
fn test_struct(x: &dyn SampleStruct) -> u32 {
    let res = x.add_ten(10);
    println!("result is {}", res);
    res
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
    fn it_works_with_fixture(get24: i64) {
        let _result = add(2, 2);
        assert_eq!(get24, 4);
    }

    // test with parameter for multi-caes test
    // 1. apply #[case] attribute (arg and expected values)
    // 2. write values into the function's parameter expression
    // 3. use the values function's body
    #[rstest]
    #[case(10, 5, 15)]
    #[case(200, 15, 215)]
    fn it_works_with_pameters(#[case] a: u64,#[case] b: u64,#[case] expected: u64) {
        let _result = add(a, b);
        assert_eq!(_result, expected);
    }

    #[fixture]
    fn get24() -> i64 {
        4
    }
}
