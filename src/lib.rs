pub mod result;

#[macro_export]
macro_rules! ok_or_return {
    ($result:expr)=>{
        match $result.await {
            Ok(value)=> {Ok(value)},
            Err(err)=>{
                error!("error {:?} at file {} line {}", err, file!(),line!());
                Err(err)
            }
        }
    };
}
