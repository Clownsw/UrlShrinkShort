use tide::Request;

use crate::State;

pub async fn hello(_req: Request<State>) -> tide::Result {
    Ok("Hello World".into())
}
