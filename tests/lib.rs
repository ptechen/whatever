use whatever::result::AppError::CustomError;
use whatever::result::AppResult;

#[test]
fn test() {
    let v = test1();
    println!("{:?}", v);
}

fn test1() -> AppResult<()> {
    let v = test_custom_error()?;
    println!("{:?}", v);
    Ok(())
}

fn test_custom_error() -> AppResult<Option<()>> {
    Err(CustomError(String::from("123")))
}
