use tide::Redirect;
use tide::Request;
use tide::Response;
use tide::StatusCode;

use crate::dao::urls_dao::{delete_by_id, insert_url, select_by_name, select_by_target};
use crate::pojo::msg::Msg;
use crate::pojo::url::InsertUrl;
use crate::util::global_util::rand_hex_str;
use crate::State;
use crate::LOCAL_URL;
use chrono::Duration;
use chrono::Utc;

pub async fn create_url(mut req: Request<State>) -> tide::Result {
    let mut body = match req.body_string().await {
        Ok(v) => v,
        Err(_) => "".to_string(),
    };

    body = body.trim().to_string();

    println!("=================body = {}", body);

    let mut msg = Msg::new();

    if !body.is_empty() {
        let r = select_by_target(&req.state().db_pool, &body).await;

        match r {
            Ok(v) => {
                let local_url_ref = LOCAL_URL.lock().unwrap();
                msg.code = 200;
                msg.message = format!("{}t/{}", *local_url_ref, v.url_name);
            }
            Err(_) => {
                let hex_str = rand_hex_str().await;
                let hex_str_clone = hex_str.clone();

                let url = InsertUrl {
                    url_name: hex_str,
                    url_target: body,
                    url_time: Utc::now().checked_add_signed(Duration::days(1)).unwrap(),
                };

                let result = insert_url(&req.state().db_pool, &url).await;
                println!("rows_affected = {}", result.rows_affected());

                if result.rows_affected() > 0 {
                    let local_url_ref = LOCAL_URL.lock().unwrap();

                    msg.code = 200;
                    msg.message = format!("{}t/{}", *local_url_ref, hex_str_clone);
                }
            }
        }
    }

    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(serde_json::to_string(&msg).unwrap());
    resp.insert_header("Access-Control-Allow-Origin", "*");

    Ok(resp)
}

pub async fn redirect_target(req: Request<State>) -> tide::Result {
    match req.param("target") {
        Ok(v) => match select_by_name(&req.state().db_pool, &v.to_string()).await {
            Ok(v) => {
                if v.url_time.timestamp_millis() < Utc::now().timestamp_millis() {
                    delete_by_id(&req.state().db_pool, v.url_id).await;
                    return Ok("time expired!".into());
                }

                Ok(Redirect::new(v.url_target).into())
            }
            Err(_) => Ok("not found!".into()),
        },
        Err(_) => Ok("error".into()),
    }
}
