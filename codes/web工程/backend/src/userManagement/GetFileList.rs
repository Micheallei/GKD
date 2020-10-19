use super::super::database::Query::Query;

pub struct GetFileList {
    status: String,
    html: String,
    QueryPath: String,
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



    pub fn execute(Querypath1:String) -> String {
        let query = Query::new();
        let tpath: Option<String> = Some(Querypath1);
        let mut file_array = query.queryFile_Bypath(tpath);

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
}