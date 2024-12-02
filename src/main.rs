use warp::Filter;
use warp::Reply;

#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let route = warp::any().map(|| "Mi primer servidor basico hecho por mi Luis Castillo 02/12/2024");
    Ok(route.boxed().into())
}
