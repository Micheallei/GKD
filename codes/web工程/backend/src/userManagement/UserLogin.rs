use super::super::database::Query::Query;
use std::cmp::Ordering;

pub struct UserLogin{
    serialVersionUID:i64,
    userName:String,
    userPasswd:String,
    result:String
}


impl UserLogin{
    pub fn new() -> UserLogin {
        UserLogin{
            serialVersionUID:1,
            userName:"".to_string(),
            userPasswd:"".to_string(),
            result:"".to_string()
        }
    }
    pub fn setResult(&mut self,result:String) {
        self.result = result;
    }

    pub fn getResult(&self) -> String {
        return self.result.clone();
    }

    pub fn setUserName(&mut self,name:String) {
        self.userName = name;
    }

    pub fn setUserPasswd(&mut self,Passwd:String) {
        self.userPasswd = Passwd;
    }

    pub fn getUserName(&self) -> String {
        return self.userName.clone();
    }

    pub fn getUserPasswd(&self) -> String {
        return self.userPasswd.clone();
    }

    // pub fn execute(&mut self) -> String {
    //     let query = Query::new();
    //     let passwdStandard:Option<String> = query.queryUserPasswd(Some(self.userName.clone()));
    //     //query.closeConnection();
        
    //     if passwdStandard == None {
    //        self.result = "登录失败：该用户不存在！".to_string();
    //        return "success".to_string();
    //     }
        
    //     match passwdStandard.unwrap().cmp(&self.userPasswd){
    //         Ordering::Equal => {
    //             self.result = "login sucessfully!".to_string();
    //             return "success".to_string();
    //         }
    //         Ordering::Greater => {
    //             self.result = "登录失败：密码错误！".to_string();
    //             return "success".to_string();
    //         }
    //         Ordering::Less => {
    //             self.result = "登录失败：密码错误！".to_string();
    //             return "success".to_string();
    //         }
    //     }
    // }
    pub fn execute(userName:String,userPasswd:String) -> String {
        let query = Query::new();
        let passwdStandard:Option<String> = query.queryUserPasswd(Some(userName));
        //query.closeConnection();
        
        if passwdStandard == None {
           //self.result = "登录失败：该用户不存在！".to_string();
           return "login fail!".to_string();
        }
        
        match passwdStandard.unwrap().cmp(&userPasswd){
            Ordering::Equal => {
                //self.result = "login sucessfully!".to_string();
                return "login sucessfully!".to_string();
            }
            Ordering::Greater => {
                //self.result = "登录失败：密码错误！".to_string();
                return "login fail!".to_string();
            }
            Ordering::Less => {
                //self.result = "登录失败：密码错误！".to_string();
                return "login fail!".to_string();
            }
        }
    }
}

// * UserReg.execute(params.userName,params.userPasswd);成功返回字符串"success",失败返回字符串"fail"
// * UserLogin.execute(params.userName,params.userPasswd); 成功返回字符串"login sucessfully!"，失败返回字符串"login fail!"