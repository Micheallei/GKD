#[macro_use]
extern crate mysql;

use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value, json};
use serde_json::{Value, json};
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
use userManagement::FileUploader::FileUploader;


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

#[derive(Serialize, Deserialize)]//获得文件目录时的数据格式
pub struct GetFileList_param {
    whose:String,
    QueryPath: String,
}


#[derive(Serialize, Deserialize)]//删除文件数据格式
pub struct FileDelete_param {
    namelist:Vec<String>,
    pathlist: Vec<String>,
    whose:String,
}


#[derive(Serialize, Deserialize)]//文件重命名时的数据格式
pub struct FileRename_param {
    Filename: String,
    Filepath: String,
    newname:String,
    whose:String,
}

#[derive(Serialize, Deserialize)]//创建新文件夹时的数据格式
pub struct NewFolder_param {
    Filename: String,
    path: String,
    whose:String,
}

#[derive(Serialize, Deserialize)]//上传文件时的接收数据格式
pub struct Fileuploader_param{
    //serialVersionUID: i32,
    path: String,
    fileName: String,
    //result: String,
    //devices: Value,
    fileType: String,
    nod: i32,
    noa: i32,
    fileSize: i32,   
    whose: String,
    //fileId: i32,
    //fragmentFolderPath:PathBuf,
    //fileFolderPath:PathBuf,
}


#[derive(Serialize, Deserialize)]//上传文件时的返回值数据格式
pub struct Fileuploader_return {
    result: String,
    devices: Value,
    fileId:i32,
    html:String,
}


#[derive(Serialize, Deserialize)]
struct Return_string {
    result: String,
}



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
async fn downloadreg(params: web::Json<FileDownloader_param>) -> web::Json<FileDownloader> {
    println!("hhh");
    println!("path: {0} ,name:{1}",params.path,params.name);
    
    //println!("{}", result);
    web::Json(FileDownloader::downloadRegister(params.path.clone(),params.name.clone()))
}


#[post("/GetFileList")]
async fn getfilelist(params: web::Json<GetFileList_param>) -> web::Json<Return_string> {
    println!("whose:{0}, Querypath: {1} ",params.whose,params.QueryPath);
    let result:String = GetFileList::execute(params.whose.clone(),params.QueryPath.clone());
    //println!("{}", result);
    web::Json(Return_string {
        result,//返回的是html代码的字符串
    })
}

/*
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
*/

#[post("/FileDelete")]
async fn filedelete(params: web::Json<FileDelete_param>) -> web::Json<Return_string> {
    println!("name: {0} ,path:{1}",params.namelist[0],params.pathlist[0]);
    GetFileList::filedelete(params.namelist.clone(),params.pathlist.clone(),params.whose.clone());
    let result:String = GetFileList::execute(params.whose.clone(),params.pathlist[0].clone());
    println!("{}", result);
    web::Json(Return_string {
        result,//"Error"或"OK"
    })
}


#[post("/FileRename")]
async fn filerename(params: web::Json<FileRename_param>) -> web::Json<Return_string> {
    //println!("path: {0} ,name:{1}",params.path,params.name);
    GetFileList::filerename(params.Filename.clone(),params.Filepath.clone(),params.newname.clone(),params.whose.clone());
    let result:String = GetFileList::execute(params.whose.clone(),params.Filepath.clone());
    println!("{}", result);
    web::Json(Return_string {
        result,
    })
}

#[post("/CreateDir")]
async fn create_dir(params: web::Json<NewFolder_param>) -> web::Json<Return_string> {
    //println!("path: {0} ,name:{1}",params.path,params.name);
    GetFileList::create_dir(params.Filename.clone(),params.path.clone(),params.whose.clone());
    let result:String = GetFileList::execute(params.whose.clone(),params.path.clone());
    println!("{}", result);
    web::Json(Return_string {
        result,
    })
}

#[post("/uploadRegister")]
async fn uploadregister(params: web::Json<Fileuploader_param>) -> web::Json<Fileuploader_return> {
    
    //println!("path: {0} ,name:{1}",params.path,params.name);
    //let device_str = "";
    let mut fileuploader=FileUploader {
        serialVersionUID: 1,
        path: params.path.clone(),
        fileName: params.fileName.clone(),
        result: String::new(),
        devices: json!({"1":1}),//serde_json::from_str(device_str).unwrap(),//?用空字符串来初始化
        fileType: params.fileType.clone(),
        fileSize: params.fileSize.clone(),
        noa: params.noa.clone(),
        nod: params.nod.clone(),
        whose: params.whose.clone(),
        fileId: 0,
    };
    fileuploader.uploadRegister();
    //println!("hhh");
    let mut result = fileuploader.getResult();
    let mut devices = fileuploader.getDevices();
    let mut fileId = fileuploader.getFileID();
    let html:String=GetFileList::execute(params.whose.clone(),params.path.clone());
    println!("HTML:{}",html);
    web::Json(Fileuploader_return{
        result,
        devices,
        fileId,
        html,
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
            .service(filedelete)
            .service(filerename)
            .service(create_dir)
            .service(uploadregister)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
//.service(progresscheck)    .service(decodefile)