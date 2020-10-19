$(document).ready(function(){
    //注册ａｊａｘ
      $("#regSubmitButton").click(function(){
        $.ajax({//传递给web后端的数据？怎么传呢
          url:"http://127.0.0.1:8000/UserReg",
          type:"POST",
          data:JSON.stringify({
            userName:$("#inputUsername_reg").val(),
            userPasswd:$("#inputPassword_reg").val()
          }),
          dataType:"json",
          contentType:"application/json; charset=utf-8",
          success:function(databack){//回调函数
              //var obj = $.parseJSON(databack);
              //var feedback = obj.result;
              var feedback = databack.result;
              alert(feedback);
              $("#statusFeedback").text(feedback);//输出到控制台的结果信息?
          }
        });
      });



     //登录ａｊａｘ 
      $("#loginSubmitButton").click(function(){
        var	str=new String("login sucessfully!");
        $.ajax({
          url:"http://127.0.0.1:8000/UserLogin",
          type:"POST",
          data:JSON.stringify({
            userName:$("#inputUsername_login").val(),
            userPasswd:$("#inputPassword_login").val()
          }),
          dataType:"json",
          contentType:"application/json; charset=utf-8",
          success:function(databack){
              //var obj = $.parseJSON(databack);
              //var feedback = obj.result;
              var feedback = databack.result;
              if(feedback==str)
                  window.location.href='jsp/majorPage.jsp';//相当于在回调函数中把当前页面更改为登录后的主页面(即显示图片和文件目录了)
              //格式是ｊｓｏｎ　输出反馈信息到ｃｏｎｓｏｌｅ
              else
                  alert(feedback);
                  $("#statusFeedback").text(feedback);
          }
        });
      });
      
    });