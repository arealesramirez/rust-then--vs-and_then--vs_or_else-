use futures::{
    FutureExt,
    TryFutureExt,
    future::err,
};

#[tokio::main]
async fn main() {
    test_then_method().await;

    test_and_then_method().await;

    test_or_else_method().await;
}

async fn test_then_method() {
    let my_future_is_bright = async { String::from("bright") };
    let my_future_is_not_bright =
        my_future_is_bright.then(|x| async move { format!("{} {}", String::from("not"), x) });

    assert_eq!(my_future_is_not_bright.await, String::from("not bright"));
}

async fn test_and_then_method() {
    let ok_future = async { Ok::<u8, u8>(1) };
    let updated_ok_future = ok_future.and_then(|x| async move { Ok(x + 5) });

    assert_eq!(updated_ok_future.await, Ok(6));
}

async fn test_or_else_method() {
    let err_future = err::<String, String>(String::from("Expected Error"));
    let updated_err_future = err_future.or_else(|x| async move {
       Err(
        format!("{} {}", String::from("[Error]"), x)
       )
    });

    assert_eq!(updated_err_future.await, Err(String::from("[Error] Expected Error")));
}