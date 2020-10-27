# Notes

## I llw组项目总结

### 1.1 结题报告摘录笔记

1. 结题报告中有与IPFS、NAS等分布式文件系统的对比

2. 项目结构：

![](C:\Users\12935\Pictures\Screenshots\屏幕截图 2020-10-14 204450.png)

3. webassembly: 并行编码、流水作业？
4. websocket：建立在TCP上，兼容HTTP，通信高效
### 1.2 Code 笔记

#### 1.2.1 client 存储端

**websocket**

1. 摘自（conclusion）js带有原生的 websocket API，因此llw组只实现用于存储节点（client）的 WebSocket 服务端 API，即 `client-Websocket` 部分代码

   ```
   //WebSocket.java
   public class WebSocket{
   	init()
   	WebSocket(){
   		client = server.accept();
           accept();
   	}
   	close()
   	sendPong()
   	catchBytes()
   	recv()
   	recvFile()
   	createHead()
   	sendText()
   	sendBin()
   	sendMessage()
   	sendFile()
   	echo()
   	accept(){
   		//收到浏览器发来的request，返回给一个response
   	}
   }
   ```

2. client中的 `connect--fragmentManager、RequestManager` 两部分用到了llw组自己写的websocket API

   调用过程：

   ```
   client.java--begin()--new requestManager(selfDataPort, selfIp)--start() ->
   requestManager中run()--user = new Websocket()->
   FragmentManager fragmentManager = new FragmentManager(user) -> fragmentManager.start() ->
   FragmentManager.java--String msg = new String(user.recv());->
   根据msg为'D'/'U'->
   fragmentmanager .sendFragment()sendDigest()/recvDigest()recvFragment()
   
   以fragmeManager.recvFragment()为例
   recvFragment()->调用websocket的recvfile()和sendMessage()
   recvfile()通过文件流将从websocket接收到的byte（使用自己实现的recv（））写入指定文件
   
   recv()牵扯到head的处理等等
   ```

   

   .digest文件：Digest认证是为了修复[基本认证](https://www.cnblogs.com/xiaoxiaotank/p/11009796.html)协议的严重缺陷而设计的，秉承“绝不通过明文在网络发送密码”的原则，通过“密码摘要”进行认证，大大提高了安全性。

#### 1.2.2 web

Go-webassembly

syscall/js 包用于在 javascript 和 Go 之间传数据，主要用在web--webapp--js--wasm--wasm_exec.js

## II 进度记录

### client 改写

- client

  - client.rs：

    1. 变量的改变：

       不再使用uploadFolders和uploadAddrs两个变量（因为客户与存储段分离），使用selfIp,selfDataPort,rs。从 setup.ini 中依次读入serverIp,servercontrolPort,selfIp,selfdataPort,clientId,fragmentFolder,rs。tmpFragmentFolder有定义，但与原项目不同，未从setup.ini中读取。

    2. 调用方法参数的的改变：

       调用serverConnecter和FragmentManager的init方法参数改变

     3. 创建线程改变

        new serverconnecter时参数个数变化

        不在创建FolderScanner线程，改为创建RequestManager线程

    4. 潜在bug

       由于原rs代码没有使用类的结构，因此rs的定义在main内部，但现在需要通过函数 getRs() 得到此值，可能有问题

  - SynItem.rs：原本也没用上

- com

  - Encoder、Decoder部分没有区别

- connect

  - FileTransporter：没有区别

  - SeverConnecter：

    1. serverconnecter的成员变量增加了selfIp，selfDataPort，对应new时将两个值作为参数传入

    2. run中，增加了向服务器发送“3”开头报文的步骤

       ```rust
       socket.write_fmt(format_args!("3 {} {} {}\n", self.client_id.to_string(),self.selfIp,self.selfDataPort);
       ```

    3. unread_request一部部分被删除
    
  - FragmentManager:

    1. FragmentManager内增加了websocket类型的user字段，原java中通过构造方法对其实例化，此处将方法名称改为new_user(iUser:Websocket),之后调用时请注意
    2. 增加了全局变量selfPort，但似乎java文件中也没有对其有后续的定义
    3. run()函数中增加了很多，主要是recv()到了msg之后根据其为‘U’or‘D’or‘E’，做出相应的处理
    4. sendFragment和recvFragment中删除了大量原来与服务器通信的部分
    5. 增加了sendDigest()\recvDigest()，删除了deleteFragment()中与服务器通信的部分
    
  - RequestManager

- fileDetector：无区别

### server 改写

- controlConnect

  - ClientThread
    1. readsentence中 ip,port通过报文传入，原来直接从clientsocket中读
    2. readsentence增加了一些queryDevice错误后的处理代码，原来也有但没写
    3. readsentence增加了收到报文“3”开头后的处理部分：收到id,ip和 port。根据id到数据库中queryDevice，若找到则修改其 ip，port，然后返回报文`set ip={}, port={} successfully`
  - ServerThread：无区别

- database：无区别

  - DeviceItem
  - FileItem
  - RequestItem
  - Query

- dataConnect：无区别

  - ClientThread：无区别
  - FileTransporter：无区别
  - ServerThread：无区别

- DFS_server

  1. 增加了查询onlineDevice的部分，将所有device的is_on_line改为false
  2. 删除了根据dataport端口创建dataconnect的serverThread的部分

  3. 原来的rust的代码中在main结束前加了while true 以防进程退出，是否有更好的方法？

### WebSocket

java.util：https://docs.oracle.com/javase/8/docs/api/java/util/package-summary.html

**java.util.Arrays**

```
static boolean[] copyOf(char[] original, int newLength)
```

Copies the specified array, truncating or padding with null characters (if necessary) so the copy has the specified length.

**java.util.Base64**

-> Rust base64: https://docs.rs/base64/0.13.0/base64/ 似乎更复杂一些

```java
//用到的java代码
Base64.getEncoder().encodeToString(MessageDigest.getInstance("SHA-1").digest((match.group(1) + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").getBytes("UTF-8")))
```

```
static Base64.Encoder getEncoder()
```

Returns a [`Base64.Encoder`](https://docs.oracle.com/javase/8/docs/api/java/util/Base64.Encoder.html) that encodes using the [Basic](https://docs.oracle.com/javase/8/docs/api/java/util/Base64.html#basic) type base64 encoding scheme.

```
public static class Base64.Encoder
extends Object
```

This class implements an encoder for encoding byte data using the Base64 encoding scheme as specified in RFC 4648 and RFC 2045.

Instances of [`Base64.Encoder`](https://docs.oracle.com/javase/8/docs/api/java/util/Base64.Encoder.html) class are safe for use by multiple concurrent threads.

Unless otherwise noted, passing a `null` argument to a method of this class will cause a [`NullPointerException`](https://docs.oracle.com/javase/8/docs/api/java/lang/NullPointerException.html) to be thrown.

`String encodeToString(byte[] src)`Encodes the specified byte array into a String using the [`Base64`](https://docs.oracle.com/javase/8/docs/api/java/util/Base64.html) encoding scheme.

**java.util.Scanner**

-> 在rust中未找到关于delimiter相应的方法

关于 java scanner 中定界符作用的解释：https://www.itranslater.com/qa/details/2582828352121865216

```
Scanner useDelimiter(String pattern)
```

Sets this scanner's delimiting pattern to a pattern constructed from the specified `String`.

**java.util.regex**

关于matcher和pattern的解释：https://www.cnblogs.com/wang-zai/p/7802622.html