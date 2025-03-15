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
    mock.expect_add_ten().returning(|v| v + 10);

    println!("env var is {}", std::env::var("DATABASE_USERNAME").unwrap());

    assert_eq!(10, test_struct(&mock));
}

#[automock]
trait SampleStruct {
    fn add_ten(&self, x: u32) -> u32;
}

// normally, traits cannot be an argument type, but dynamically resolved by mockall
fn test_struct(x: &dyn SampleStruct) -> u32 {
    let res = x.add_ten(10);
    println!("result is {}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    // Run this commands before implementation of test code
    // cargo add rstest mockall --dev
    // cargo add cargo-expand
    // cargo add sqlx --features runtime-tokio,postgres

    // test with fixture
    // 1. run cargo add rstest cargo-expand --dev,
    // 2. use rstest macro on test fn
    // 3. define fixture with fixture macro
    // 4. pass fixture as arg on test fn (type is return value of fixture)
    #[rstest]
    fn it_works_with_fixture(get24: i64) {
        let _result = add(2, 2);
        assert_eq!(get24, 4);
    }
    #[fixture]
    fn get24() -> i64 {
        4
    }

    // test with parameter for multi-caes test
    // 1. run cargo add mockall
    // 2. apply #[case] attribute (arg and expected values)
    // 3. write values into the function's parameter expression
    // 4. use the values function's body
    #[rstest]
    #[case(10, 5, 15)]
    #[case(200, 15, 215)]
    fn it_works_with_pameters(#[case] a: u64, #[case] b: u64, #[case] expected: u64) {
        let _result = add(a, b);
        assert_eq!(_result, expected);
    }

    #[sqlx::test]
    async fn it_works_with_sqlx(pool: sqlx::PgPool) {
        let row = sqlx::query!("SELECT 1 + 1 AS result")
            // let row = sqlx::query!("SELECT COUNT(*) FROM books;")
            .fetch_one(&pool)
            .await
            .unwrap();
        // if test failed, row will be printed
        dbg!(&row);
        let result = row.result;
        assert_eq!(result, Some(2));
    }

    #[sqlx::test(migrator = "MIGRATOR", fixtures("common"))]
    async fn it_works_with_sqlx2(pool: sqlx::PgPool) {
        // sqlx test
        // https://github.com/launchbadge/sqlx/blob/main/CHANGELOG.md#changed-10
        // https://docs.rs/sqlx/latest/sqlx/attr.test.html#automatic-test-database-management-requires-migrate-feature

        let row = sqlx::query!("SELECT name FROM users WHERE name = 'hiro-admin';")
            .fetch_one(&pool)
            .await
            .unwrap();

        dbg!(&row);

        // when want to debug variables, use dbg!
        // dbg!(std::env::vars());
        // dbg!(std::env::var("DATABASE_URL").unwrap());
        let result = row.name;
        assert_eq!(result, "hiro-admin");
    }

    // since 0.6.1, #[sqlx::test] attribute includes automatic database management, migration(migrate feature is required) and fixture application.
    // However, it migrations dir is not in the same directory as Cargo.toml, it is necessary to specify the path.
    // https://docs.rs/sqlx/latest/sqlx/attr.test.html#automatic-migrations-requires-migrate-feature
    pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../adapter/migrations");
}
