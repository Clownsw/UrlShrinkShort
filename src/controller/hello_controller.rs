use tide::Request;

use crate::State;

pub async fn hello(req: Request<State>) -> tide::Result {
    let name: String = req.param("name")?.to_string();

    Ok(format!("Hello {}", name).into())
}

pub async fn test_redirect(req: Request<State>) -> tide::Result {
    let query_params = match req.url().query() {
        Some(v) => v,
        None => "",
    };

    println!("query_params = {}", query_params);

    Ok(tide::Redirect::new("https://www.baidu.com/").into())
}
