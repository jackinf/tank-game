pub fn ok_or_panic<TSuccess>(f: &mut dyn FnMut() -> Result<TSuccess, String>) -> TSuccess {
    let res = f();
    if let Err(e) = res {
        println!("Error: {}", e);
        panic!("{}", e);
    }

    res.unwrap()
}
