use tide::Request;

use crate::dao::urls_dao::insert_url;
use crate::pojo::msg::Msg;
use crate::pojo::url::InsertUrl;
use crate::util::global_util::rand_hex_str;
use crate::State;
use crate::LOCAL_URL;
use chrono::Utc;

pub async fn create_url(mut req: Request<State>) -> tide::Result {
    let body = match req.body_string().await {
        Ok(v) => v,
        Err(_) => "".to_string(),
    };

    println!("body = {}", body);

    let mut msg = Msg::new();

    if !body.is_empty() {
        let hex_str = rand_hex_str().await;
        let hex_str_clone = hex_str.clone();

        let url = InsertUrl {
            url_name: hex_str,
            url_target: body,
            url_time: Utc::now(),
        };

        let result = insert_url(&req.state().db_pool, &url).await;
        println!("rows_affected = {}", result.rows_affected());

        if result.rows_affected() > 0 {
            let local_url_ref = LOCAL_URL.lock().unwrap();

            msg.code = 200;
            msg.message = format!("{}{}", *local_url_ref, hex_str_clone);
        }
    }

    Ok(serde_json::to_string(&msg).unwrap().into())
}
