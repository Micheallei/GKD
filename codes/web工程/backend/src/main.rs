#[macro_use]
extern crate mysql;

use serde::{Deserialize, Serialize};
use actix_web::{
    get,post,http::header,middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use actix_cors::Cors;


/*
#[derive(Debug, Serialize, Deserialize)]
struct UserReg {
    userName: String,
    userPasswd: String,
}
*/

mod userManagement;
mod database;
mod com;

use userManagement::UserReg::UserReg;
use userManagement::UserLogin::UserLogin;
use userManagement::FileDownloader::FileDownloader;
use userManagement::GetFileList::GetFileList;



#[derive(Serialize, Deserialize)]//用户名和密码，注册登录时用到
pub struct User {
    userName: String,
    userPasswd:String,
}

#[derive(Serialize, Deserialize)]//添加下载请求时的数据格式
pub struct FileDownloader_param {
    path: String,
    name: String,
}

#[derive(Serialize, Deserialize)]//添加下载请求时的数据格式
pub struct GetFileList_param {
    QueryPath: String,
}


#[derive(Serialize, Deserialize)]
struct Return_string {
    result: String,
}


/*
#[get("/index1")]
async fn index() -> impl Responder {
    println!("sss");
    HttpResponse::Ok().body("Hello world!")
}
*/
#[post("/UserReg")]
async fn register(params: web::Json<User>) -> web::Json<Return_string> {//pqz
    println!("username: {0} ,userPasswd:{1}",params.userName,params.userPasswd);
    let result:String = UserReg::execute(params.userName.clone(),params.userPasswd.clone());
    //let result:String=String::from("sddd");
    web::Json(Return_string {
        result,
    })
}

#[post("/UserLogin")]
async fn login(params: web::Json<User>) -> web::Json<Return_string> {
    println!("username: {0} ,userPasswd:{1}",params.userName,params.userPasswd);
    let result:String = UserLogin::execute(params.userName.clone(),params.userPasswd.clone());//pqz
    web::Json(Return_string {
        result,
    })
}

#[post("/DownloadReg")]
async fn downloadreg(params: web::Json<FileDownloader_param>) -> web::Json<Return_string> {
    println!("hhh");
    println!("path: {0} ,name:{1}",params.path,params.name);
    let result:String = FileDownloader::downloadRegister(params.path.clone(),params.name.clone());
    println!("{}", result);
    web::Json(Return_string {
        result,
    })
}


#[post("/GetFileList")]
async fn getfilelist(params: web::Json<GetFileList_param>) -> web::Json<Return_string> {
    println!("Querypath: {0} ",params.QueryPath);
    let result:String = GetFileList::execute(params.QueryPath.clone());
    //println!("{}", result);
    web::Json(Return_string {
        result,//返回的是html代码的字符串
    })
}

#[post("/progressCheck")]
async fn progresscheck(params: web::Json<FileDownloader_param>) -> web::Json<Return_string> {
    println!("path: {0} ,name:{1}",params.path,params.name);
    let result:String = FileDownloader::progressCheck(params.path.clone(),params.name.clone());
    println!("{}", result);
    web::Json(Return_string {
        result,//进度条数值的字符串形式或"Error"
    })
}

#[post("/decodeFile")]
async fn decodefile(params: web::Json<FileDownloader_param>) -> web::Json<Return_string> {
    println!("path: {0} ,name:{1}",params.path,params.name);
    let result:String = FileDownloader::decodeFile(params.path.clone(),params.name.clone());
    println!("{}", result);
    web::Json(Return_string {
        result,//"Error"或"OK"
    })
}


/*
#[post("/index")]
fn handle_post_1(params: web::Form<User>) -> Result<HttpResponse> {
    println!("username: {} ,userPasswd:{}",params.userName,params.userPasswd)
    Ok(HttpResponse::Ok().json(Result {
        result: "success".to_string(),
    }))
}
*/

/*
fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
*/

#[actix_rt::main]
async fn main()  -> std::io::Result<()>{
    //std::env::set_var("RUST_LOG", "actix_web=info");
    //env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .service(register)
            .service(login)
            .service(downloadreg)
            .service(getfilelist)
            .service(progresscheck)
            .service(decodefile)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
