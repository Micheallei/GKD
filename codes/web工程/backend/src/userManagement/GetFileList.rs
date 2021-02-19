use super::super::database::Query::Query;
use super::super::database::FileItem::FileItem;

pub struct GetFileList {
    status: String,
    html: String,
    QueryPath: String,
    whose:String,
    serialVersionUID: i64,
}

impl GetFileList {

    pub fn init (&mut self){
        self.serialVersionUID = 1;
    }

    pub fn new () -> GetFileList {
        GetFileList {
            status: String::new(),
            html: String::new(),
            QueryPath: String::new(),
            whose:String::new(),
            serialVersionUID: 1,
        }
    }

    pub fn setStatus (&mut self, nstatus: String){
        self.status = nstatus;
    }

    pub fn getStatus (&self) -> String {
        self.status.clone()
    }

    pub fn setQueryPath (&mut self, path: String) {
        self.QueryPath = path;
    }

    pub fn getQueryPath (&self) -> String {
        self.QueryPath.clone()
    }

    pub fn setHtml (&mut self, nhtml: String) {
        self.html = nhtml;
    }

    pub fn getHtml (&self) -> String {
        self.html.clone()
    }

    pub fn setWhose (&mut self, nwhose: String) {
        self.whose = nwhose;
    }

    pub fn getWhose (&self) -> String {
        self.whose.clone()
    }



    pub fn execute(whose:String,Querypath1:String) -> String {
        let query = Query::new();
        let tpath: Option<String> = Some(Querypath1);
        let mut file_array = query.query_file_list(Some(whose),tpath);

        let mut html:String = String::new();
        html = html + 
            "<tr class=\"file_list_back\">"+
                "<td> </td>"+
                "<td> <label><input type=\"checkbox\">&emsp;&emsp;</label><span class=\"glyphicon glyphicon-folder-open\"></span>&emsp;../</td>"+
                "<td> </td>"+
                "<td> </td>"+
                "</tr>";

        //let mut return_val = String::new();

        if file_array.len() == 0 {
            let status = String::from("false");
            //return_val = String::from("success");
            //return return_val;
        }
        else {
            let status = String::from("true");
        }

        for i in 0..file_array.len() {
            html = html + "<tr class=\"file_list_go\">";
            html = html + "<td> </td>";
            if file_array[i].is_folder() {
                html = html + 
                    "<td> <label><input type=\"checkbox\"></label> 　　<span class=\"glyphicon glyphicon-folder-open\"></span>　"+
                        &file_array[i].get_name()+ 
                    "</td>";
            }
            else {
                html = html + 
                    "<td> <label><input type=\"checkbox\"></label> 　　<span class=\"glyphicon glyphicon-file\"></span>　"+
                        &file_array[i].get_name()+
                    "</td>";
            }
            html = html +    
                "<td>"+
                    &file_array[i].get_attribute()+
                "</td>"+
                "<td>"+
                    &file_array[i].get_time()+
                "</td>"+
			"</tr>";
        }

        html
    }


    pub fn filedelete(namelist:Vec<String>,pathlist:Vec<String>,whose:String){
        let query = Query::new();
        for i in 0..namelist.len(){
            query.deleteFile_Byname(namelist[i].clone(),pathlist[i].clone(),whose.clone());
            //把数据库中对于数据项删除，参数为文件名、文件路径、whose
        }
    }

    pub fn filerename(Filename:String,Filepath:String,newname:String,whose:String){
        let query = Query::new();
        //把数据库中对应文件项修改，参数为原文件名，文件路径，新名字，whose
        query.RenameFile(Filename.clone(),Filepath.clone(),newname.clone(),whose.clone());
    }


    pub fn create_dir(Filename:String,Filepath:String,whose:String){
        let query = Query::new();
        //在数据库中加入新表项
        query.addFile(FileItem::init_2(
            Filename.clone(),
            Filepath.clone(),
            String::new(),//attribute
            String::new(),//time
            0,//nod
            0,//noa
            true,//is_folder
            String::new(),//filetype
            0,//file_size
            whose.clone()
        ));
    }
}