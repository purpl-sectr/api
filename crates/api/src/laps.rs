use rocket::serde::json::Json;
use rocket::{get, routes, State};

use infrastructure::ConnectionPool;
use shared::prelude::*;

#[get("/<series>/laps?<param..>")]
pub fn laps(
    db: &State<ConnectionPool>,
    series: Series,
    param: shared::parameters::GetLapsParameter,
) -> Result<Json<Response<Vec<Laps>>>> {
    let conn = &mut db.from_series(series).get().unwrap();

    let query = application::laps::LapsQueryBuilder::params(param).build();

    let res = query.query_and_count(conn);

    let response = Response {
        data: res.0,
        pagination: res.1,
        series,
    };

    Ok(Json(response))
}

pub fn handlers() -> Vec<rocket::Route> {
    routes![laps]
}