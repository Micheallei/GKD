use super::super::database::Query::Query;
pub struct UserReg {
    serialVersionUID:i64,
    userName:String,
    userPasswd:String,
    result:String
}

impl UserReg {
    pub fn new() -> UserReg {
        UserReg{
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
    //     let ID:i32 = query.addUser(self.userName.clone(),self.userPasswd.clone());
    //     //query.closeConnection();
    //     if ID == -1 {
    //         self.result = "注册失败!".to_string();
    //     }
    //     else {
    //         self.result = "恭喜你，注册成功!".to_string();
    //     }
    //     return "success".to_string();
    // }

    pub fn execute(userName:String,userPasswd:String)  -> String{
        let query = Query::new();
        let ID:i32 = query.addUser(userName,userPasswd);
        if ID == -1 {
            return "fail".to_string();
        }
        else {
            return "success".to_string();
        }
    }
}

// * UserReg.execute(params.userName,params.userPasswd);成功返回字符串"success",失败返回字符串"fail"
// * UserLogin.execute(params.userName,params.userPasswd); 成功返回字符串"login sucessfully!"，失败返回字符串"login fail!"