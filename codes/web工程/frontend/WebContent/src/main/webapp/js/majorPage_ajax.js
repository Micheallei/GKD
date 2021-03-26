var curr_path_array = new Array();
/*
function createAndDownloadFile(fileName, fileType, content) {
	var aTag = document.createElement('a');
	var blob = new Blob([content], { type: fileType, name: fileName });
	aTag.download = fileName;
	aTag.href = URL.createObjectURL(blob);
	aTag.click();
	URL.revokeObjectURL(blob);
}*/
/*
function waitForSocketConnection(socket, callback){
	setTimeout(
		function () {
			if (socket.readyState === 1) {
				console.log("Connection is made")
				if (callback != null){
					callback();
				}
			} else {
				console.log("wait for connection...")
				waitForSocketConnection(socket, callback);
			}

		}, 10); // wait 5 milisecond for the connection...
}
function syncSleep(time) {
	const start = new Date().getTime();
	while (new Date().getTime() - start < time) {}
}*/
function WebSocketDownload(ip,port,fragmentName,content,digest,fragmentId)
{
	var ret_bytes;
	var ret_digest;
	if ("WebSocket" in window)
	{
		let ws = new WebSocket("ws://"+ip+":"+port, 'websocket');
		ws.binaryType="arraybuffer";
		ws.onopen = function()//发送信息
		{
				//alert("Sending Message...");
				ws.send("D");
				ws.send(fragmentName);
				console.log('send filename');
		};

		ws.onmessage = function (evt)
		{
			let received_data = evt.data;
			//alert("received");
			//if(evt.data instanceof Blob ){
			if(evt.data instanceof ArrayBuffer ){
				//alert("Received arraybuffer");
				console.log('Blob');
				ret_bytes= received_data;
				console.log('recv bytes');
			}
			if(typeof(evt.data) =='string') {
				//alert("Received data string");
				console.log('string');
				ret_digest= received_data;
				console.log('recv digest');
			}
		};

		ws.onclose = function()
		{
			// sí websocket
			//alert("Connection Closed...");
			content[fragmentId]=ret_bytes;
			digest[fragmentId]=ret_digest;
			console.log('closed connection');
		};
	}
	else
	{
	}
	/*
	alert("Start...");
	syncSleep(2000);
	//alert("Finish...");
	console.log(ret_bytes);
	*/
	return ret_bytes;
}
/*
function decodeFile(fileName,fileType,nod,noa,content,digest,fileSize)
{
	console.log(fileName);
	console.log(fileType);
	console.log((fileSize));
	console.log(nod);
	console.log(noa);

	for(var i=0;i<noa+nod;i++){
		console.log(content[i]);
		console.log(digest[i]);
	}
}*/
function decodeFile(fileName, fileType, numOfDivision, numOfAppend, content, digest, fileSize,file_blocks) {
	//clean wrong parts
	var errors = 0;
	var queue=[];//分块任务队列
	var blocksize=1024*1024;//每个文件块<1MB
	var decoded=[];
	var total = Math.round((numOfDivision+numOfAppend)/file_blocks);
	/*
	for (var i = 0; i < content.length; i++) {
		if (digest[i] != objectHash.MD5(content[i])) {
			errors += 1;
			content[i] = new Uint8Array(content[i].length);//摘要值不符合记录的记为null
		}
	}*/

	//console.log(content);
	//const t5 = Date.now();//Decode timing start

	var contentView=new Array(content.length);
	for(var i=0;i<content.length;i++){
		contentView[i]=new Uint8Array(content[i]);//对文件碎片内容格式转换，以便在Go中运行
	}
	//var decoded = erasure.recombine(contentView, fileSize, numOfDivision, numOfAppend);
	for(i=0;i<file_blocks;i++){
		queue.push(new Promise(function (resolve, reject) {
			//console.log(i);
			var shards=new Array(total);
			for(j=0;j<total;j++) {
				shards[j] = contentView[i * total + j];
			}
			var result;
			if(i < file_blocks-1){//为中间块，大小固定，2MB
				//console.log("shards:"+shards);
				result=callDecoder(shards, Math.round(numOfDivision/file_blocks),Math.round(numOfAppend/file_blocks));
				//console.log("result: "+result);
				if (result.length > blocksize){
					result = result.subarray(0, blocksize);
				}
			}
			else {//<=1MB的文件块的nod和noa部分
				//console.log("shards: "+shards);
				//console.log("shards[0]:"+shards[0]);
				result=callDecoder(shards, Math.round(numOfDivision/file_blocks), Math.round(numOfAppend/file_blocks));
				//console.log("result:"+result);
				result = result.subarray(0, fileSize-i*blocksize);
			}
			//console.log("result[0]"+result[0]);//49
			//console.log(result.length);//12
			resolve(result);
		}));
	}
	Promise.all(queue).then(function (results) {
		console.log("results.length  "+results.length); // 获得一个Array: ['P1', 'P2']
		console.log("results0.length  "+results[0].length);
		//console.log("results  "+results);
		decoded = Array.from(results[0]);
		for(i=1;i<results.length;i++){
			//console.log("results i length  "+results[i].length);
			//decoded.push.apply(decoded,results[i]);
			decoded = decoded.concat(Array.from(results[i]));
		}
		//console.log("decoded length: "+decoded.length);
		//const t6 = Date.now();//Decode timing end
		console.log("decoded length: "+decoded.length);
		// after decoded, download the file and show info(time, errors)
		createAndDownloadFile(fileName, fileType, new Uint8Array(decoded));

		// if (document.getElementById("decode") != null)
		// 	document.getElementById("decode").innerHTML += "Decode with " + errors + " errors succeeded in " + (t6 - t5) + "mS</br>";
		// console.log("Erasure decode took " + (t6 - t5) + " mS");
		return Promise.resolve(true);
	});
}
function WebSocketUpload(ip,port,fragmentName,fragmentContent,digest)
{
	if ("WebSocket" in window)
	{
		var ws = new WebSocket("ws://"+ip+":"+port,'websocket');

		ws.onopen = function()
		{
			ws.send("U");
			ws.send(fragmentName);
			//console.log(fragmentName);
			ws.send(digest);
			//console.log(digest);
			ws.send(fragmentContent);
			//console.log(fragmentContent);
			//ws.close();
		};

		ws.onmessage = function (evt)
		{
			let respondMsg = evt.data;
			//console.log(respondMsg);//success or not
		};

		ws.onclose = function()
		{
			console.log("upload closed");
		};
	}
	else
	{
		alert("");//TODO
	}
}
function encodeFile(selectedFile) {
	/* After file selected, get info(name, type, size) as global,
     * and read filestream As ArrayBuffer
     * use FileReader, seemingly usable for Chrome & Firefox
     * turn to upLoader()
     * handleFileSelect(this) -> upLoader
     * */
	// sendFragments((str)fileName,(str)fileType,(int)numOfDivision,(int)numOfAppend,(byte[][])content(content),(string[])digest,(int)fileSize);

	let numOfDivision = 4;
	let numOfAppend = 4;
	var fileType = [];
	var fileName = [];
	var fileSize;
	// TODO temp fix
	var content = [];
	var digest = [];
	/*
     * user choose a file, and trigger handleFileSelect -> upLoader(this)
     * upLoader get the file in *.result as raw, then create a worker to do encoding
     * evt : from                  ¨L not this evt
     *  function handleFileSelect(evt) {
            ...
            var reader = new FileReader();
            ...                 ¨L Maybe this evt(I'm not sure)
            reader.onload = upLoader;
        }
     * */
	function upLoader(evt) {
		/*
		if (document.getElementById("tips") != null)
			document.getElementById("tips").innerHTML = "<h3>Please wait during erasure code profiling...</h3></br>";
		/*receive file*/

		var fileString = evt.target.result;
		/*
		if (document.getElementById("info") != null)
			document.getElementById("info").innerHTML = "loaded as Uint8Array...</br>";
		if (document.getElementById("encode") != null)
			document.getElementById("encode").innerHTML = "";
		if (document.getElementById("decode") != null)
			document.getElementById("decode").innerHTML = "";*/
		let raw = new Uint8Array(fileString);
		var queue=[];//分块任务队列
		var blocksize=1024*1024;//每个文件块<1MB
		var file_blocks = Math.ceil(raw.length/(blocksize));//文件分了多少块
		/*if (document.getElementById("info") != null)
			document.getElementById("info").innerHTML +=
				"<h3>file name</h3> " + fileName
				+ "</br><h3>file type</h3> " + fileType
				+ "</br><h3>file size</h3> " + fileSize / 1024 + " KB"
				+ "</br>Division " + numOfDivision
				+ " Append " + numOfAppend
				+ "</br></br>";
*/
		// create a worker to do the erasure coding
		/*
		var blob = new Blob(["onmessage = function(e) { postMessage(e.data); }"]);
		// Obtain a blob URL reference to our worker 'file'.
		var blobURL = window.URL.createObjectURL(blob);
		var worker = new Worker(blobURL);
		worker.onmessage = function (e) {
			alert("waiting for worker");
			console.log(e.data);*/
			/*fileEncoder*/
			console.log('uploader raw: '+raw);
			console.log('fileblocks: '+file_blocks);
			for(i=0;i<file_blocks;i++){
				queue.push(new Promise(function (resolve, reject) {
					console.log("raw slice: "+raw.slice(i*blocksize,(i+1)*blocksize));
					var result=callEncoder(raw.slice(i*blocksize,(i+1)*blocksize),numOfDivision,numOfAppend);
					resolve(result);
				}));
			}
			Promise.all(queue).then(function (results) {
				console.log(results); // 获得一个Array: ['P1', 'P2']
				for(i=0;i<results.length;i++){
					content.push.apply(content,results[i]);
				}
				console.log('content: '+content);
				for (var i = 0; i < content.length; i++) {
					digest[i] = objectHash.MD5(content[i]);
				}
				encodeCallBack({
					fileName: fileName,
					fileType: fileType,
					numOfDivision: numOfDivision*file_blocks,
					numOfAppend: numOfAppend*file_blocks,
					content: content,
					digest: digest,
					fileSize: fileSize,
					fileblocks:file_blocks
				})
			});
			//const t1 = Date.now();//Encode timing start
			//TODO
			//content = erasure.split(raw, numOfDivision, numOfAppend);
			
			//const t2 = Date.now();//Encode timing end
			//console.log("Erasure encode took " + (t2 - t1) + " mS");
			//if (document.getElementById("encode") != null)
			//	document.getElementById("encode").innerHTML += "Encode took " + (t2 - t1) + "mS to generate " + content.length + " fragments</br>";
			//TODO
			//var digest = new Array();
			//const t3 = Date.now();//Hash timing start
			
			//const t4 = Date.now();//Hash timing end
			//if (document.getElementById("encode") != null)
			//	document.getElementById("encode").innerHTML += "Hash took " + (t4 - t3) + "mS to generate " + content.length + " digests</br>";
			/* Next we can use sendFragments() to send the results to the backend,
             * hopefully content[][] remain as 2d array
             * */
			// Here we use decodeFile to test if encode and decode both work properlly.
			//decodeFile(fileName, fileType, numOfDivision, numOfAppend, content, digest, fileSize);
			//console.log("Success");

			//console.log(content);
			
		//};
		//console.log(raw);
		//worker.postMessage({ input: raw });
	}
	if (selectedFile) {
		//console.log(selectedFile);
		fileType = selectedFile.type;
		fileName = selectedFile.name;
		fileSize = selectedFile.size;
		var reader = new FileReader();
		//reader.readAsBinaryString(files[0]);
		reader.onload = upLoader;
		reader.readAsArrayBuffer(selectedFile);//当reader把文件读入后，会调用upLoader函数
		console.log("uploder reading");
	}
}
function encodeCallBack(fileInfo){


	//var uploadForm = new FormData();
	var deviceArray;
	var fileId;
	var path1 = "/";
	if(curr_path_array.length>1)
		path1="";
	for(var i=1;i<curr_path_array.length;i++)
		path1 = path1 + curr_path_array[i] + "/" ;
	//uploadForm.append("path", path);//string
	//uploadForm.append("fileName", fileInfo.fileName);
	//uploadForm.append("fileType", fileInfo.fileType);
	//uploadForm.append("nod", fileInfo.numOfDivision);
	//uploadForm.append("noa", fileInfo.numOfAppend);
	//uploadForm.append("fileSize", fileInfo.fileSize);
	//uploadForm.append("whose", $.cookie("username"));
	$.ajax({
		url: "http://127.0.0.1:8000/uploadRegister",
		type: "POST",
		data:JSON.stringify({
			path:path1,//string
			fileName:fileInfo.fileName,//string
			fileType:fileInfo.fileType,//string
			nod:fileInfo.numOfDivision,//int
			noa:fileInfo.numOfAppend,//int
			fileSize:fileInfo.fileSize,//int
			fileblocks:fileInfo.fileblocks, //int
			whose:$.cookie("username"),//string
		}),
		dataType:"json",
		contentType:"application/json; charset=utf-8",
		async: false,								//此处采用同步查询进度
		success: function (databack) {
			var retFileInfo = databack;
			let result = retFileInfo.result;
			deviceArray = retFileInfo.devices.forms;
			fileId=retFileInfo.fileId;
			var new_file_list = databack.html;//html字符串
			$("#file_list_body").html(new_file_list);
			//console.log(result);
			//console.log(deviceArray);
		}
	});

	//console.log(deviceArray);
	//alert(deviceArray);
	//alert("Before upload");
	for (var i = 0; i < deviceArray.length; i++) {
		WebSocketUpload(deviceArray[i].ip, deviceArray[i].port, (fileId * 100 + i).toString(), fileInfo.content[i], fileInfo.digest[i]);
	}
}
function fileUpload() {

	let selectedFile = document.getElementById('files').files[0];//TODO multisel file
	encodeFile(selectedFile);

}
function fileDownload() {
	var path1;
	var name1;

	//重置时钟
    //window.clearInterval(int);
	//millisecond = second = minute = hour = 0;
	
	var item=$("#file_list_body").children();
	item = item.next();
	while(item.length!=0)//可对打钩的每个文件同时下载
	{
		name1 = "";
		path1 = "";
		//如果ｉｔｅｍ不为空，则进行处理
		var children=item.children();
		if( (children[1].children[1].className=="glyphicon glyphicon-file") && (children[1].children[0].children[0].checked) )
		{
			//文件路径
			path1 = path1 + "/";
			/*********/	if(curr_path_array.length>1)
			path1="";
			for(var i=1;i<curr_path_array.length;i++)
				path1 = path1 + curr_path_array[i] + "/" ;
			//文件名
			name1 = name1 + $.trim(children[1].innerText);
			//alert(path + "  " + name);

/*
//从向后端发送请求开始计时
			function timer() {
				millisecond = millisecond + 50;
				if(millisecond>=1000)
				{
					millisecond=0;
					second=second+1;
				}
				if(second>=60)
				{
					second=0;
					minute=minute+1;
				}

				if(minute>=60)
				{
					minute=0;
					hour=hour+1;
				}
				//document.getElementById('timetext').value=hour+'时'+minute+'分'+second+'秒'+millisecond+'毫秒';

			}
			//int = setInterval(timer, 50);

*/
			/*
             *
             * 此处应当利用ａｊａｘ　远程调用　downloadRegister(String path, String name)；
             *
             * */
			//利用ａｊａｘ　远程调用　downloadRegister(String path, String name)；
			var result;
			//var	form=new FormData();
			var deviceArray;
			var fileInfo;
			//form.append("path",path);
			//form.append("name",name);
			$.ajax({
				url:"http://127.0.0.1:8000/DownloadReg",
				type:"POST",
				data:JSON.stringify({
					path:path1,
					name:name1,
				}),
				dataType:"json",
				contentType:"application/json; charset=utf-8",
				async: false,							//此处采用同步查询进度
				success:function(databack){
					fileInfo = databack;//返回过来的文件的一些信息
					//alert(result);
				}
			});
			result = fileInfo.result;//向服务器发送信息，返回对应的文件信息以及存储点的信息
			deviceArray = fileInfo.devices.forms;
			console.log(result);

			//错误处理
			if(result=="NotEnoughFragments")
			{
				$("#statusFeedback").text("在线碎片数目不足！");
				return;
			}
			else if(result == "Error")
			{
				$("#statusFeedback").text("服务器响应该请求内部出错！");
				return;
			}
			var content= new Array(fileInfo.noa+fileInfo.nod);//存储文件碎片
			var digest= new Array(fileInfo.noa+fileInfo.nod);//存储文件碎片对应的MD5值
			for(var i=0;i<deviceArray.length;i++)//对每一个设备，调用WebSocketDownload函数下载文件碎片
			{
				console.log(deviceArray[i]);
				let received_bytes=WebSocketDownload(deviceArray[i].ip,deviceArray[i].port,deviceArray[i].filename,content,digest,deviceArray[i].fragmentId);
				//console.log(received_bytes);
				console.log('Back');
				//console.log(content[deviceArray[i].fragmentId];
				//createAndDownloadFile(deviceArray[i].filename, 'jpg', received_bytes)
			}
			let downloadTimeoutId =setTimeout(function(){
				decodeFile(fileInfo.name,fileInfo.fileType,fileInfo.nod,fileInfo.noa,content,digest,fileInfo.fileSize,fileInfo.fileblocks);
			}, 10000)

            //添加进度条
			/*
            var ratio1 = 0;
            var progress_bar='<div class="progress progress-striped active"><div class="progress-bar progress-bar-success" role=\"progressbar" style="width: '
                +ratio1+'%;">'
                +path1+name1+'</div></div>';
            $("#download_progress_area").append(progress_bar);

			 */
		}
		//
		item = item.next();
	}
}

function filedelete(){
	var filename_list=[];
	var filepath_list=[];
	var item=$("#file_list_body").children();
	item = item.next();
	while(item.length!=0)//对打钩的每个文件都作删除操作
	{
		name1 = "";
		path1 = "";
		//如果ｉｔｅｍ不为空，则进行处理
		var children = item.children();
		if ((children[1].children[1].className == "glyphicon glyphicon-file") && (children[1].children[0].children[0].checked)) {
			//文件路径
			path1 = path1 + "/";
			/*********/
			if (curr_path_array.length > 1)
				path1 = "";
			for (var i = 1; i < curr_path_array.length; i++)
				path1 = path1 + curr_path_array[i] + "/";
			//文件名
			name1 = name1 + $.trim(children[1].innerText);
			//alert(path1 + "  " + name1);
			filename_list.push(name1);
			filepath_list.push(path1);
		}
		item = item.next();
	}
	console.log(filename_list);
	console.log(filepath_list);
	$.ajax({
		url:"http://127.0.0.1:8000/FileDelete",
		type:"POST",
		data:JSON.stringify({//一次性传过去所有选中文件的信息
			namelist:filename_list,
			pathlist:filepath_list,
			whose:$.cookie("username"),
		}),
		dataType:"json",
		contentType:"application/json; charset=utf-8",
		success:function(databack){
			var obj = databack;
			var new_file_list = obj.result;//html字符串
			//alert(new_file_list);
			$("#file_list_body").html(new_file_list);
		}
	});
	$("#statusFeedback").text("文件删除成功！");
}


function filerename(new_name){
	var item=$("#file_list_body").children();
	item = item.next();
	while(item.length!=0)//对打勾的第一个文件重命名
	{
		name1 = "";
		path1 = "";
		//如果ｉｔｅｍ不为空，则进行处理
		var children=item.children();
		if( (children[1].children[1].className=="glyphicon glyphicon-file") && (children[1].children[0].children[0].checked) )
		{
			//文件路径
			path1 = path1 + "/";
			/*********/	if(curr_path_array.length>1)
			path1="";
			for(var i=1;i<curr_path_array.length;i++)
				path1 = path1 + curr_path_array[i] + "/" ;
			//文件名
			name1 = name1 + $.trim(children[1].innerText);
			//alert(path + "  " + name);

			$.ajax({
				url:"http://127.0.0.1:8000/FileRename",
				type:"POST",
				data:JSON.stringify({
					Filename:name1,
					Filepath:path1,
					newname:new_name,
					whose:$.cookie('username'),
				}),
				dataType:"json",
				contentType:"application/json; charset=utf-8",
				success:function(databack){
					//var obj = $.parseJSON(databack);
					var new_file_list = databack.result;//html字符串
					//alert(new_file_list);
					$("#file_list_body").html(new_file_list);
				}
			});
			$("#statusFeedback").text("成功创建新的文件夹！");

			break;
		}
		item = item.next();
	}
}

$(document).ready(function(){
	//var curr_path_array = new Array();
	var hour = 0;
	var minute = 0;
	var second = 0;
	var millisecond = 0;
	var int;
	curr_path_array[0] = "/";
	curr_path_html = "<li>ROOT</li>";
	
	//面包屑式访问路径显示  初始化
	$("#curr_path").html(curr_path_html);
	
	//文件下载
	$("#button_download").click(function(){
		fileDownload();
	});
	/*
		<tr id="file_list_first">
		<td> </td>
 		<td> <label><input type="checkbox">&emsp;&emsp;</label><span class="glyphicon glyphicon-folder-open"></span>&emsp;../</td>
 		<td></td>
 		<td></td>
		</tr>

*/
	
	//文件上传
	$("#button_upload").click(function() {
		$("#files").click();
	});
	
	//文件重命名
	$("#button_rename").click(function() {
		$('#my_dialog_rename').dialog({
            modal:true,
            width:"400",
            height:"223"
            });
        document.getElementById("my_dialog_rename").style.display="block";
	});

	//文件重命名的弹出框中的取消函数
	$("#rename_dir_cancel").click(function(){
		console.info("取消");
		$("#filename").val("");
		document.getElementById("my_dialog_rename").style.display="none";
	});

	//文件重命名的弹出框中的确定函数
	$("#rename_dir_save").click(function(){
		document.getElementById("my_dialog_rename").style.display="none";
		var new_name = document.getElementById("filename").value;
		$("#filename").val("");
		filerename(new_name);
	});

	//文件夹创建
    $("#button_create_dir").click(function(){
        $('#my_dialog').dialog({
            modal:true,
            width:"400",
            height:"223"
            });
        document.getElementById("my_dialog").style.display="block";
    });
    

    //文件夹创建的弹出框中的取消函数
    $("#create_dir_cancel").click(function(){
        console.info("取消");
        $("#foldername").val("");
		document.getElementById("my_dialog").style.display="none";
        //$('#my_dialog').dialog("close");
    });

    //文件夹创建的弹出框中的确定函数
    $("#create_dir_save").click(function(){
        //$('#my_dialog').dialog("close");
		document.getElementById("my_dialog").style.display="none";
        var create_name = document.getElementById("foldername").value;
        console.log(create_name);
		$("#foldername").val("");
		//当前路径
		var path1 = "/";
		if(curr_path_array.length>1)
			path1="";
		for(var i=1;i<curr_path_array.length;i++)
			path1 = path1 + curr_path_array[i] + "/" ;
        //将文件夹名传回服务器，然后那边保存
		$.ajax({
			url:"http://127.0.0.1:8000/CreateDir",
			type:"POST",
			data:JSON.stringify({
				Filename:create_name,
				path:path1,
				whose:$.cookie('username'),
			}),
			dataType:"json",
			contentType:"application/json; charset=utf-8",
			success:function(databack){
				//var obj = $.parseJSON(databack);
				var new_file_list = databack.result;//html字符串
				//alert(new_file_list);
				$("#file_list_body").html(new_file_list);
			}
		});
		$("#statusFeedback").text("成功创建新的文件夹！");
	});
	
	//文件夹删除
	$("#button_delete").click(function() {
		filedelete();
	});


	//点击文件目录进入其子目录　　刷新文件目录列表
	$("#file_list_body").on("click","tr.file_list_go",
			function()
			{
				//如果是文件而不是文件夹，点击不刷新目录，提示信息
				if(this.children[1].children[1].className=="glyphicon glyphicon-file")
				{
					$("#statusFeedback").text("您所点击的是文件而不是文件夹，无法进入该目录！");
					return;
				}
				else if(!this.children[1].children[0].children[0].checked){
					//更新路径显示
					curr_path_array = curr_path_array.concat( $.trim(this.children[1].innerText) );			//此处用$.trim去除空格
					curr_path_html = "<li>ROOT</li>";
					for(var i=1;i<curr_path_array.length;i++)
						curr_path_html = curr_path_html + "<li>" + curr_path_array[i] + "</li>";
					$("#curr_path").html(curr_path_html);
					//ajax
					var QueryPath1="/";
					/*********/		if(curr_path_array.length>1)
						QueryPath1="";
					for(var i=1;i<curr_path_array.length;i++)
					{
						QueryPath1 = QueryPath1 + curr_path_array[i] + "/" ;
					}
					var whose1 = $.cookie("username");
					//alert(queryPath);
					$.ajax({
						url:"http://127.0.0.1:8000/GetFileList",
						type:"POST",
						data:JSON.stringify({
							whose:whose1,
							QueryPath:QueryPath1,
						}),
						dataType:"json",
						contentType:"application/json; charset=utf-8",
						success:function(databack){
							//var obj = $.parseJSON(databack);
							var new_file_list = databack.result;//html字符串
							//alert(new_file_list);
							$("#file_list_body").html(new_file_list);
						}
					});
					$("#statusFeedback").text("成功进入该目录！");
				}
				else{
					$("#statusFeedback").text("请先取消打勾！");
				}
			}
	);
	
	//点击的是返回上一层的文件项
	$("#file_list_body").on("click","tr.file_list_back",
			function()
			{
				//如果是顶层目录，点击上级目录无操作，提示信息
				if(curr_path_array.length==1)
				{
					$("#statusFeedback").text("已经是根目录了，无法返回上一层！");
					return; 
				}
				//更新路径显示
				curr_path_array.pop();
				curr_path_html = "<li>ROOT</li>";
				for(var i=1;i<curr_path_array.length;i++)
				curr_path_html = curr_path_html + "<li>" + curr_path_array[i] + "</li>";
				$("#curr_path").html(curr_path_html);	
				
				//ajax
				var QueryPath1="/";
/*********/		if(curr_path_array.length>1)
					QueryPath1="";
				for(var i=1;i<curr_path_array.length;i++)
				{
					QueryPath1 = QueryPath1 + curr_path_array[i] + "/" ;
				}

				var whose1 = $.cookie("username");
				//alert(queryPath);
				$.ajax({
						url:"http://127.0.0.1:8000/GetFileList",
						type:"POST",
						data:JSON.stringify({
							whose:whose1,
							QueryPath:QueryPath1,
						}),
						dataType:"json",
						contentType:"application/json; charset=utf-8",
						success:function(databack){
							//var obj = $.parseJSON(databack);
							var new_file_list = databack.result;//html字符串
							//alert(new_file_list);
							$("#file_list_body").html(new_file_list);
						}
				});
				$("#statusFeedback").text("成功返回上层目录！");
			}
	);

	
	//定时刷新预下载进度
	function refresh_progress(){
		var progressArray = $("#download_progress_area").children();
		var str="";
		var ratio=100;
		for(var i=0;i<progressArray.length;i++)
		if(progressArray[i].className=="progress progress-striped active")
		{
			//alert("here length="+progressArray.length + "i="+i);
			var path1="";
			var name1="";
			var strArray;
			strArray = progressArray[i].innerText.split('/');
			for(var j=0;j<strArray.length-1;j++)
				path1 = path1 + strArray[j] + "/";
			name1 = strArray[strArray.length-1];
			//str = str + path + name + "    ";
			//alert(name+" "+path)
			/*
			 * 
			 * 此处应远程调用　public static int progressCheck(String path, String name)　　返回进度
			 * 
			 * */
			/*
			$.ajax({
					url:"http://127.0.0.1:8000/progressCheck",
					type:"POST",
					data:JSON.stringify({
						path:path1,
						name:name1,
					}),
					dataType:"json",
					contentType:"application/json; charset=utf-8",
					async: false,								//此处采用同步查询进度
					success:function(databack){
						//var obj = $.parseJSON(databack);
						var result = databack.result;
						if(result == "Error")
						{
							ratio = 0;
							$("#statusFeedback").text("查询进度出错！");
						}
						else
							ratio = parseInt(result);

					}
			});
			*/


			var countFrag=0;
			for(var j=0;j<digest.length;++j){
				if(digest[j]!==undefined)
					countFrag++;
			}
			if(countFrag<nod)
				ratio=100*countFrag/nod;
			else
				ratio=100;
			//////////////////////////////////////////////////////////////////
			//进度条的ｈｔｍｌ代码
			var progress_bar='<div class="progress progress-striped active"><div class="progress-bar progress-bar-success" role=\"progressbar" style="width: '
				+ratio+'%;">'
				+path1+name1+'</div></div>';
			//如果预下载完成
			if(ratio==100)
			{
				/*
				 * 
				 * 
				 * 此处应当调用远程函数　　public static int decodeFile(String path, String name)
				 * 
				 * */
				/*
				$.ajax({
						url:"http://127.0.0.1:8000/decodeFile",
						type:"POST",
						data:JSON.stringify({
							path:path1,
							name:name1,
						}),
						dataType:"json",
						contentType:"application/json; charset=utf-8",
						async: false,								//此处采用同步查询进度
						success:function(databack){
							//var obj = $.parseJSON(databack);
							var result = databack.result;

							//停止计时
							window.clearInterval(int);

							if(result == "Error")
								$("#statusFeedback").text("解码拼接出错！");

							else{
								var time_temp = hour + "时" + minute + "分" + second + "秒" + millisecond + "毫秒";
								$("#statusFeedback").text("解码拼接文件成功,用时").append(time_temp);
							}



						}
				});*/
				
				
				var clickToGetFile = '<a href="#" download="' + name + '">' + progress_bar + '</a>';
				//alert(temp);
				progressArray[i].outerHTML = clickToGetFile;
				
			}
			else
			{
				//修改进度条进度
				//alert(progress_bar);
				progressArray[i].outerHTML = progress_bar;				
			}
			///////////////////////////////////////////////////////
		}
		
	}
	//设置进度刷新间隔
	window.setInterval(function(){refresh_progress();},3000);

	
	
	
	//自动删除下载过的文件链接和进度条
	$("#download_progress_area").on("click","a",
			function()
			{
				this.outerHTML = "";
			}

	);
	
	
	
//总的结束符	
});

/*
   			<tr id="file_list_first">
      			<td> </td>
         		<td> <label><input type="checkbox">&emsp;&emsp;</label><span class="glyphicon glyphicon-folder-open"></span>&emsp;../</td>
         		<td></td>
         		<td></td>
      		</tr>
 
 */
