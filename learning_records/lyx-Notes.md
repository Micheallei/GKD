# 10-17

## llw小组项目调研

### conclusion.md

#### 项目实现功能及思路

* 容器化服务器端：docker封装简化配置

  * 容器可以把应用及其依赖项都将打包成一个可以复用的镜像并与
    其他进程环境隔离。容器只虚拟化了文件系统、网络、运行环境等，虚拟化开销相对小 

  * 简化了目录节点的配置，同时还可以减少开发环境和部署环境不同带来的问题 ，便于项目推广

  * Docker-Compose:Docker-Compose 的 scale 功能还支持创建多个实例进行负载均衡反向代理。这可以在我们想进行用户群的扩展时，轻松解决目录节点高并发的问题，并把处理能力分布在多台主机上  

    .yml代码？？

* 前端美化：layui

* 多用户权限支持：目录节点身份验证，用户只能访问自己的文件

  * 数据库配置：同原项目，5个表

  * 改进技术：FILE表中加入WHOSE列，使得文件有了归属（与原FILE表区别见文档勾画）

  * 新的Web端设计：

    输入用户名、密码后检测对应模块，跳转至特定文件空间（涉及到由抓取所有文件列表到抓取特定文件列表的转变）

    核心是 sql 抓取语句的改进  

* 更高效的文件传输：
  
  * P2P实现点对点数据传输
  
  * WebSocket协议：实现浏览器和存储节点间直连来传输数据
  
    * 好处：
  
      * 存储端实现容易(建立在TCP协议上)
      * 默认端口80和443，与HTTP协议有良好兼容性，能通过各种HTTP代理服务器
      * 可发送文本或二进制数据
      * 无同源限制，浏览器可与任意服务器通信
  
    * 实现WebSocket API：JS有原生WebSocket客户端API，只需实现存储端的WebSocket服务端API
  
      实现细节：握手、解析/产生帧
  
* 减轻中央服务器负担
  * 服务器端：目录节点
    * 负责web管理界面
    * 协调各存储节点资源，如数据冗余备份提供身份验证等
  * 存储端：文件传输不经过目录节点
  
* 碎片分配策略

  * 考虑设备和用户的在线时间重合度
  * 一个设备或者一个用户一天中的在线时间表示成一个长度为 24 的 0/1 向量，在上传文件时尽可能地给覆盖上传者的在线时间段 x% 以上的存储结点分配碎片  
  * 分配策略也考虑了剩余容量，不会分给剩余容量达上限的节点
  * 计算图和数学公式展示效果
  * 考虑加入一些24h在线的可靠节点。以提高下载成功概率

* 提高文件安全性和可用性：
  * 冗余备份（纠删码）

    * 纠删码原理及其稳定性：里德-所罗门算法 +柯西矩阵作为编码矩阵  。且相比普通备份能节省大量空间

    * 用 JavaScript 和 WebAssembly 在浏览器上做纠删码 （其中 JavaScript 直接基于开源的实现进行了一些修改  ）

      *链接*

    * Go 语言的实现有较多的 Star量，内容也较为完善。为了在网页中应用项目中的函数，我们利用它编写了 Go 语言代码并编译成WebAssembly 格式。  

      *链接*

    * WebAssembly与JavaScript效率对比

    * 浏览器端实现文件编解码

      * HTML5中FileReader 获取本地文件 ，记录文件名、大小、设置的分块数等信息。转为Uint8Array格式（因为在 Go 接收 JavaScript 传递的数据时，需要通过 CopyBytesToGo 方法拷贝数据到 Go
        的对象中，这个方法要求传递 Uint8Array 类型的数据  ），Worker线程中调用导出的编码函数生成碎片的MD5摘要
      * Go 提供了专有 API syscall/js 包，使我们可以与 JavaScript 之间传递函数与数据。来源于 JavaScript 的数据在 Go 中会是 js.Value 类型，需要使用包里的函数进行转换。除了通过上一节提到的CopyBytesToGo 方法拷贝 JavaScript 数据到 Go 的对象中，我们还会用到 CopyBytesToJS 将运算结果
        返回给 JavaScript，以及 FuncOf 用于包装 Go 函数  

      * Go代码中接收Uint8Array类型数据，并提供三个函数给Js使用（Go函数传给Js调用时函数参数和返回值类型有固定要求）

        1. callEncoder编码：接收 JavaScript 中 Uint8Array 类型的原始文件数据，以及进行纠删码编码需要的原始数据块、冗余块**数目**两个参数，并传递给 goEncoder 以调用 Go 开源库的函数 (用到了柯西矩阵)，得到编码后数组(content)后,调用CopyBytes函数转为js.Value类型，可在Js中直接使用

        2. callDecoder解码：碎片的摘要发生变化，说明碎片可能损坏，在解码时应当认为碎片丢失。在 callDecoder 函数中，接收到的参数是 JavaScript 代码中由文件碎片组成的二维数组，其中我们会将摘要值不符合记录的碎片
           设为 null。  对应在goDecoder中调用开源库函数

        3. callMd5计算碎片的MD5值：从而在解码时判断碎片内容是否发生了改变 ，以忽略解码时已损坏的碎片。

           通过比较本地文件和云端文件的摘要也可以实现文件秒传功能 (未实现)

           *链接*

      * Go-WebAssembly编码性能测试

        *链接*

  * 普通设备运行客户端即可，跨平台兼容

  * 无缝扩容

* 目的：提供良好的私人部署网盘服务

![image-20201016225504597](D:\Typora\photos\image-20201016225504597.png)

#### 相关项目调研

* IPFS（星际文件系统）：区块链技术+P2P文件传输模式
* NAS

#### 项目展望

* 再偏中心化一点：让目录节点承担文件中转工作
  * 预约离线下载
  * 代替低性能设备编解码
* 更高性能的纠删码模块设计：上传下载的文件以流的方式处理，以每块 512 B 为单位放入 RS 矩阵中进行运算，对文件进行分块处理，可以大幅度减少内存等资源的占用，并充分利用系统的网络性能进行传输  
  * 并行编码
  * 流水作业

项目完成度：P2p传输加购、目录节点端容器化、证明了优化分配碎片之后文件下载的可用性，以及 WebAssembly 实现的纠删码足以忽略编解码的额外时间







### feasibility.md

##### 理论依据

* Docker实现的原理介绍

  * Docker 使用了 CGroup 和 Namespace 去隔离不同容器内的进程。Docker 容器将应用和其依赖环境打包在一起，在不包含完整的操作系统时就能运行普通应用，更加轻量级，可移植性更好  
  * Docker 中的每一个镜像都是由一系列的只读层组成的，Dockerfile 中的每一个命令都会在已有的只读层上创建一个新的层。 docker run 可以在镜像的最上层添加一个可写的容器层，所有运行时容器的修改其实都是对这个容器读写层的修改。这样的特性使得 Docker 具有一定版本管理的功能  

* 多用户权限支持：RBAC，以角色为基础的访问控制，用户-角色-权限

* Reed-Solomon编码（纠删码的一类）
  * 编解码原理：是文件一共n+m块，然后任意的k块即可恢复是吧？？？
  
  * 编码矩阵：范德蒙德、柯西矩阵，后者更优：
  
    * 降低了矩阵求逆的运算复杂度  
  
    * 通过有限域转换，将 GF(2^w) 域中的元素转换成二进制矩阵，将乘法转换为逻辑与，降低了乘法
      运算复杂度  
  
      注：在范德蒙编码的时候，我们可以采用对数/反对数表的方法，将乘法运算转换成了加法运算，并且在迦罗华域中，加法运算转换成了 XOR 运算。
      柯西编解码为了降低乘法复杂度，采用了有限域上的元素都可以使用二进制矩阵表示的原理，将乘法运
      算转换成了迦罗华域“AND 运算”和“XOR 逻辑运算”，提高了编解码效率  
  
* 分离数据与控制链接
  * 中央服务器为目录，真正的下载是直连其他用户的服务器
  * 传统C/S模式：服务器是瓶颈
  * 集中式对等网络：避免了文件中转一次的开销，客户端请求数量的增加也不会对服务器造成太大负担

##### 技术依据

* Docker部署服务端

  * Apache 作为一个 Web 服务器，缺乏处理 JSP 的功能，为了能够处理 JSP 的请求，需要使用 JSP 容器如 Tomcat。mod_jk（JK）是Apache 与 Tomcat 的连接器，附带集群和负载均衡  就 Docker 而言，应该对每个服务使用单独容
    器，如 Web 服务器运行在一个容器上，应用程序服务器运行在另一个容器上。若采用 Apache 和Tomcat 方案分别部署 HTML 和 JSP 页面，则容易使用 Docker 分别管理 Apache 和 Tomcat，动静分离  
  * Docker创建容器时用docker-alpine-java镜像，体积小且提供JRE运行时

* 多用户权限支持的技术

  * 改进技术：

    * RBAC模式，设计新的数据表：感觉他们思路有点绕

      大致是用户角色+小组角色，各有各的访问空间，然后一个用户登陆访问，则看其是哪些角色，则对应抓取相应网盘空间

  * 新Web端设计

    * 注册时，判断用户名不重复，分配 ID 等等数据模块；
    * 登录时，输入用户名、密码，检测对应模块，跳转至特定文件空间（涉及到由抓取所有文件列表到
      抓取特定文件列表的转变）；
    * 原网页基础上添加创建小组、加入小组等功能（涉及到网页设计以及和数据库交互）。  

* 纠删码实现

  * WebAssembly

  * DOM API：文档对象模型（Document Object Model，简称 DOM），是 W3C 组织推荐的处理可扩展置标语言的
    标准编程接口。它是一种与平台和语言无关的应用程序接口（API），它可以动态地访问程序和脚本，更新其内容、结构和 www 文档的风格（目前，HTML 和 XML 文档是通过说明部分定义的）。要使 Go 代码与浏览器进行交互，我们需要一个 DOM API。我们有 syscall/js 库来帮助我们解决这个问题。它是一个非常简单却功能强大的 DOM API 形式，我们可以在其上构建我们的应用程序。  

    **对应考虑RUST操作DOM的实现？？？？**

* Token实现身份验证

  * 在存储节点实现对用户身份的验证  

  * 使用基于 Token 的身份验证方法，在服务端不需要存储用户的登录记录。大概的流程是这样的：

    1. 客户端使用用户名跟密码请求登录
    2. 服务端收到请求，去验证用户名与密码
    3. 验证成功后，服务端会签发一个 Token，再把这个 Token 发送给客户端
    4. 客户端收到 Token 以后可以把它存储起来，比如放在 Cookie 里或者 Local Storage 里
    5. 客户端每次向服务端请求资源的时候需要带着服务端签发的 Token
    6. 服务端收到请求，然后去验证客户端请求里面带着的 Token，如果验证成功，就向客户端返回请求的数据
  
* JWT标准实现Token：见pdf
  
* 非对称加密：公钥和私钥
  
  在 Token 的使用过程中采用非对称加密的方式：服务器生成一对密钥，用私钥签发 Token 给用户；公钥分发给各存储设备，用于验证用户 Token 的合法性。这样就可以在存储设备验证用户身份的合法性  
  
* 潜在风险：Token泄露
  
  解决途径
  
  1. 加密传输Token
    2. 对文件加密，保证收集到足够碎片也无法复原文件
  
* OpenVPN建立虚拟局域网

  中央服务器有公网IP，但是存储设备可能没有。

  采用 OpenVPN 构建虚拟的局域网，使得当存储设备和用户不在同一个局域网内时，也能够进行寻址  

  大致思路是 OpenVPN服务器有一个静态虚拟IP地址以及一个虚拟IP地址池，然后每有一个连接成功的客户端就为其分配一个虚拟IP地址池中未分配的地址。然后每次客户端要访问某个存储设备时，就通过。。。还需查阅资料（llw调研报告里似乎有）

  为了充分利用直接进行数据连接的传输效率，可以对以下情况分类处理：

  1. 存储设备已经有公网 IP，直接访问
  2. 存储设备和用户在一个物理局域网内，直接访问
  3. 1、2外的其他情况，采用 Op enVPN 的虚拟局域网。  









任务：

1. 看调研报告和答辩ppt，查漏补缺，顺便把有关Go+WebAssembly的链接啥提取出来
2. 调研Go+WebAssembly+纠删码
3. docker调研





# 10.21

### research.md

##### 项目简介

* 分布式文件系统简介

* 容器化技术

  * 容器是轻量级的操作系统级虚拟化，可以让我们在一个资源隔离的进程中运行应用及其依赖项。运行应用程序所必需的组件都将打包成一个镜像并可以复用。执行镜像时，它运行在一个隔离环境中，并且不会共享宿主机的内存、CPU 以及磁盘，这就保证了容器内进程不能监控容器外的任何进程。  

  * 容器化技术解决的问题：运行环境的改变会产生各种问题

  * 容器化技术的优势

    * 敏捷环境：比虚拟机开销小，是轻量级脚本
    * 提高生产力：移除了跨服务依赖和冲突，每个容器可看作一个微服务，可独立升级
    * 版本控制
    * 运行环境可移植：同一镜像可以在各种平台开发、测试
    * 安全：隔离性
    * 标准化

  * 容器技术代表：Docker

    * 目标：通过对应用组件的封装、分
      发、部署、运行等生命周期的管理， 使用户的 APP（可以是一个 WEB 应用或数据库应用等等）及其运行环境能够做到“一次封装，到处运行”。  

    * Docker 集版本控制(类似于git，运行环境可在多个版本间快速切换，自由选择用哪个版本对外提供服务)、克隆继承、环境隔离等特性于一身，提出一整套软件构件、部署和维护的解决方案。  
    * Docker使用方法：一些linux上的命令

* 基于OpenVPN的局域网

  * 未看

* 纠删码

  主要是Reed-Solomon Code

  * 编码：n+m

  * 解码：任意n块恢复

  * RS编解码中涉及实数四则运算：引入伽罗华群

  * 编码原理

    * 编码矩阵*输入数据=编码后数据

    * 为方便数据存储，编码矩阵上部是单位阵（n 行 n 列），下部是 m 行 n 列矩阵。下部矩阵可以选择范
      德蒙德矩阵或柯西矩阵  ；编码矩阵需满足任意n*n子矩阵可逆

  * 数据恢复原理:报告里图示说明

##### 前瞻性/重要性分析

##### 相关工作

* Google FIle System
* Cooperative File System（P2P）
  * 节点的对称与负载的均衡
  * 节点十分不可靠
  * 可观的通讯次数与延迟
  * 安全性
* Interplanetary File System
* Ceph
* Hadoop Distributed File System
* Network File System





### Go+纠删码流程

* 上传文件

  * HTML5中的File API让用户在web端选择本地文件并读取；之后记录其文件名、大小、设置的分块数等信息；

  * 转为Uint8Array格式，然后传递给Go

    Js与Go之间传递函数和数据：

    * CopyBytesToGo
    * CopyBytesToJS
    * FuncOf：包装Go函数

  * 创建Worker线程，调用Go-WebAssembly导出的纠删码函数进行文件编码，生成MD5摘要

    Go代码中的函数:在main()声明，并阻止Go程序退出

    1. callEncoder：
       * 接收JS中Uint8Array类型的原始文件数据，以及纠删码编码所需原始数据块、冗余块两个数目参数；
       * 传递给goEncoder调用开源库函数；
       * 得到编码后数组，调用CopyBytesToJS并返回JS
       * 由JS把文件碎片通过WebSocket一个个发给存储端
    2. callMd5：为碎片生成MD5摘要，用于检验碎片内容是否发生变化

* 下载文件

  * web端请求文件下载，会向目录节点（server）发送请求，然后返回需要直连的存储端IP地址
  * JS里收集存储端发过来的数据碎片
  * callDecoder：接收碎片并调用Go函数解码
    * 接收的参数是JavaScript代码中由文件碎片组成的二维数组，摘要值不符合记录的碎片设为null
    * 对每一块碎片判空后转成Go类型，然后调用开源库函数解码，解码后再类型转换返回给JS，然后实现下载到本地

* webapp文件夹结构分析

  * Jquery.cookie.js:现成的JQuery的包，里面有关于cookie设置的一些函数

    * removeCookie

  * index.html:

    * 新增了对登录用户cookie的记录
    * 新增了当把用户名、密码输入后，按回车并释放也能触发登录/注册事件
    * 与后端的信息传递还是通过index_ajax.js，没啥变化

  * majorPage.jsp:

    * 新增对ec目录下erasure.js、funcs.js、object_hash.js的引用

    * 新增 ../js/wasm/wasm_exec.js、../js/wasm/mycoder.wasm的引用，见代码底部(192行)

    * 新增对用户cookie的检查：

      * 若没有cookie记录，那么会跳转回index.html

      * 有cookie记录，那么会对应用该用户名去查数据库，返回该用户的文件夹目录

        对应变化`query.queryFileList(username, "/");`

  * majorPage_ajax.js

    * 新增加函数：
      * WebSocketDownload：实现碎片下载，返回值为文件碎片
      * decodeFile：调用Go函数callDecoder；以及createAndDownloadFile函数(未找到定义)
      * WebSocketUpload
      * encodeFile
      * encodeCallBack

    * 新封装函数
      * fileUpload
      * fileDownload：点击下载按钮时调用
    * 大致框架没变：见418行
    * 功能
      * 初始显示根目录
      
      * 文件下载：fileDownload函数
        * 对文件打钩，点下载可以同时下载所有打钩文件，但是是串行的（一个文件下好才下另一个文件）
        * 同样还是要先发信息给server端，得到文件info以及存储碎片的设备节点信息
        * 对每个设备节点，调用WebSocketDownload函数实现碎片下载
      * 收集到一个文件所有碎片值后，调用decodeFile函数解码
      
      * 文件上传：未实现一次上传多个文件
        
          * 点击按钮，调用fileUpload函数取到用户上传的文件(在jsp文件里通过<input>项实现上传)
          
          * 调用encodeFile函数，在其内调用uploader函数-》调用Go的callEncoder函数来编码，并返回给Js一个文件碎片的数组content
          
            对content调用objectHash.MD5编码，值存到digest
          
          * 再调用encodeCallBack函数，以表单uploadForm向server端传递文件信息，server端则存储相关信息，并向js端返回分配的存储节点信息
          
          * 循环调用WebSocketUpload把文件碎片分发到各个节点
          
      * 动态刷新文件目录：(包括下一级或上一级)
        
        * 会通过cookie得到username并赋值给Whose项，传回server端，返回该用户的新一级文件目录
      * 定时刷新预下载进度
      
        * 现在就不需要再与server端交互了，直接js里查询收到的digest数组里有效的个数。不过似乎并没放到具体实现中？没找到"#download_progress_area"节点
    
  * ec文件夹下
  
    * funcs.js:
  
      * createAndDownloadFile函数，实现自产生文件的URL并实现文件下载
      * 以及一些其他自定义的函数
  
    * erasure.js：纠删码的js版本实现https://github.com/ianopolous/ErasureCodes
  
      只需调用split、recombine方法。
  
      但需注意The recombine method assume the fragments are in the same order as they were initially after encode
  
    * object_hash.js
  
  

### JS

* Cookie
  * 每个载入浏览器的 HTML 文档都会成为 Document 对象。

    Document 对象使我们可以从脚本中对 HTML 页面中的所有元素进行访问。

    **提示：**Document 对象是 Window 对象的一部分，可通过 window.document 属性对其进行访问。

  * Cookie 用于存储 web 页面的用户信息

  * 当 web 服务器向浏览器发送 web 页面时，在连接关闭后，服务端不会记录用户的信息。

    Cookie 的作用就是用于解决 "如何记录客户端的用户信息":

    * 当用户访问 web 页面时，他的名字可以记录在 cookie 中。
    * 在用户下一次访问该页面时，可以在 cookie 中读取用户访问记录。
    * 当浏览器从服务器上请求 web 页面时， 属于该页面的 cookie 会被添加到该请求中。服务端通过这种方式来获取用户的信息。

  *  **document.cookie** 属性来创建 、读取、及删除 cookie

    * 创建：过期时间、path参数(告诉浏览器cookie的路径，默认情况下，cookie 属于当前页面)

    * 读取：

      document.cookie 将以字符串的方式(且为名/值对的形式)返回所有的 cookie，类型格式： cookie1=value; cookie2=value; cookie3=value;

    * 修改：修改 cookie 类似于创建 cookie，会把旧的cookie覆盖

    * 删除：设置 expires 参数为以前的时间即可，如下所示，设置为 Thu, 01 Jan 1970 00:00:00 GMT:

      `document.cookie = "username=; expires=Thu, 01 Jan 1970 00:00:00 GMT";`

      注意，当您删除时不必指定 cookie 的值。

    * 如果您需要查找一个指定 cookie 值，您必须创建一个JavaScript 函数在 cookie 字符串中查找 cookie 值。

  * Cookie也可通过java Servlet来设置

    https://www.runoob.com/servlet/servlet-cookies-handling.html

    * Servlet 为创建基于 web 的应用程序提供了基于组件、独立于平台的方法，可以不受 CGI 程序的性能限制。Servlet 有权限访问所有的 Java API，包括访问企业级数据库的 JDBC API

* JQuery事件：当按钮被松开时，发生 keyup 事件。它发生在当前获得焦点的元素上。

  keyup() 方法触发 keyup 事件，或规定当发生 keyup 事件时运行的函数。

* ```document.getElementById(id)
  document.getElementById(id)
  ```

  返回对拥有指定 ID 的第一个对象的引用。

* 使用 **`type="file"`** 的 <input>元素使得用户可以选择一个或多个元素以[提交表单](https://developer.mozilla.org/zh-CN/docs/Learn/HTML/Forms)的方式上传到服务器上，或者通过 Javascript 的 [File API](https://developer.mozilla.org/zh-CN/docs/Web/API/File/Using_files_from_web_applications) 对文件进行操作

* FileReader对象允许Web应用程序异步读取存储在用户计算机上的文件（或原始数据缓冲区）的内容，使用 [`File`](https://developer.mozilla.org/zh-CN/docs/Web/API/File) 或 [`Blob`](https://developer.mozilla.org/zh-CN/docs/Web/API/Blob) 对象指定要读取的文件或数据。

  其中File对象可以是来自用户在一个<input>元素上选择文件后返回的[`FileList`](https://developer.mozilla.org/zh-CN/docs/Web/API/FileList)对象

  当 **FileReader** 读取文件的方式为 [readAsArrayBuffer](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsArrayBuffer), [readAsBinaryString](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsBinaryString), [readAsDataURL](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsDataURL) 或者 [readAsText](https://developer.mozilla.org/en-US/docs/Web/API/FileReader/readAsText) 的时候，会触发一个 `load` 事件。从而可以使用 **`FileReader.onload`** 属性对该事件进行处理。

  https://developer.mozilla.org/zh-CN/docs/Web/API/FileReader/onload

  https://developer.mozilla.org/zh-CN/docs/Web/API/FileReader

  https://developer.mozilla.org/zh-CN/docs/Web/API/File/Using_files_from_web_applications

* onchange 事件会在域的内容改变时发生。

* onload 事件会在页面或图像加载完成后立即发生。(如onload="f(x)"则会执行该函数)

### Go语言

* Go 语言被设计成一门应用于搭载 Web 服务器，存储集群或类似用途的巨型中央服务器的系统编程语言。

  对于高性能分布式系统领域而言，Go 语言无疑比大多数其它语言有着更高的开发效率。它提供了海量并行的支持

* 安装后文件夹

  api — 目录，包含所有API列表，方便IDE使用

  bin— 目录，存放编译后的可执行文件

  blog— 目录，

  doc— 目录，帮助文档

  lib— 目录，

  misc— 目录，

  pkg— 目录，存放编译后的包文件。pkg中的文件是Go编译生成的

  src— 目录，存放项目源文件

  注：一般，bin和pkg目录可以不创建，go命令会自动创建（如 go install），只需要创建src目录即可。

  Authors— 文件，作者列表，用记事本打开

  CONTRIBUTING.md— 文件，

  CONTRIBUTORS— 文件，

  favicon.ico— 文件，

  LICENSE— 文件，license，用记事本打开

  PATENTS— 文件，

  README.md— 文件，

  robots.txt— 文件，使用robots.txt阻止对网址的访问，详情查看https://support.google.com/webmasters/answer/6062608?hl=zh-Hans

  VERSION— 文件，版本信息，用记事本打开

* GOROOT: go的安装路径,官方包路径根据这个设置自动匹配

  GOPATH: 工作路径

  本机package寻找位置：E:\Go\src\ (from $GOROOT)
          E: \go_workspace\src\(from GOPATH)

* Go 语言的基础组成有以下几个部分：

  - 包声明
    - 第一行代码 *package main* 定义了包名。你必须在源文件中非注释的第一行指明这个文件属于哪个包，如：package main。package main表示一个可独立执行的程序，每个 Go 应用程序都包含一个名为 main 的包。
  - 引入包
    - 下一行 *import "fmt"* 告诉 Go 编译器这个程序需要使用 fmt 包（的函数，或其他元素），fmt 包实现了格式化 IO（输入/输出）的函数。
  - 函数
  - 变量：当标识符（包括常量、变量、类型、函数名、结构字段等等）以一个大写字母开头，如：Group1，那么使用这种形式的标识符的对象就可以被外部包的代码所使用（客户端程序需要先导入这个包），这被称为导出（像面向对象语言中的 public）；标识符如果以小写字母开头，则对包外是不可见的，但是他们在整个包的内部是可见并且可用的（像面向对象语言中的 protected ）
  - 语句 & 表达式
  - 注释
  - 注：需要注意的是 **{** 不能单独放在一行
  - 同一个文件夹下的文件只能有一个包名，否则编译报错；但包名与文件夹名没有直接关系
  - Go语言接口：把所有的具有共性的方法定义在一起，任何其他类型只要实现了这些方法就是实现了这个接口。

* 使用Go和Js计算md5性能对比

* 文件秒传的实现：秒传的实现其实很简单，就是利用文件的md5来跟云端的文件的md5做对比，如果相同，说明你要上传的这个文件，云端已经存在了，那么这个时候，就不需要上传了，直接标识上传完成就行，后面如果你需要下载，就提供云端的文件给你下载就好了

* 考虑利用Go来计算文件的md5，通过前端页面来选择文件，然后将文件给到Go编译成的wasm去计算，算完之后，返回给到js使用。

  js中：

  * 我们监听了input的change事件，并且在这个事件的回调函数中，可以通过this.files访问到选择的文件对象，这里我们直接取了files[0],表示获取第一个文件对象，如果你需要获取多个文件对象，可以自己改一下
  * 为什么需要将ArrayBuffer对象转换为Uint8Array对象呢？因为在Go接收js传递给它的数据的时候，我们需要通过一个叫做CopyBytesToGo的方法，来拷贝数据到go的对象中，这样在go里面才可以使用js的数据，这个方法要求的我们必须传递Uint8Array类型的数据过去
  * 这里我们调用了target对象上的calcMd5方法，然后将bytes作为第一个参数传递过去，注意，这里的calcMd5方法，是在Go里面声明的，并且挂载到了target对象上面，你可以看到我们的js代码，并没有任何地方给target对象声明一个calcMd5方法。

  go中：

  * 声明了一个calcMd5函数，注意这里的函数跟普通的go函数不太一样，需要使用js.FuncOf包裹一下
  * %USERPROFILE%\go
  * set GOARCH=amd64；set GOOS=windows
  * SET GOARCH=wasm
    SET GOOS=js    
    go build -o main.wasm

### Docker安装

* docker是运行在linux上的容器技术。
  - win10安装docker 步骤：第一步 启用win10自带的虚拟化技术Hyper-V，第二步是安装docker，第三步是自定义docker 虚拟机 数据存放路径。
  - win10运行docker 原理：启动docker，docker 会通过Hyper-V生成一个linux虚拟机 DockerDesktopVM ，然后在 DockerDesktopVM 上运行。
  - docker镜像和容器数据存放：docker 的镜像文件、容器数据 都是存放在 DockerDesktopVM 的虚拟硬盘里面，也因此，这个虚拟硬盘会占用较大的空间，所以最好是自定义docker 虚拟机 数据存放路径





## 10.25 

#### 后端java代码

* database/AnotherRequestItem.java：定义了一个类以及一些get，set方法

  ```java
  		this.ip=ip;
  		this.port=port;
  		this.filename=filename;
  		this.fragmentId=fragmentId;
  		this.fileType=fileType;
  ```

* userManagement/FileUploader.java

  * 函数getAllocateDeviceList()

    * query.queryOnlineDevice();返回在线设备:DeviceItem[]存储

    * 计算相似度：

      query.queryUserTime(whose);

      onlineDevice[i].getTime();

      看24维向量重合度有多高？算法（155行for 循环，但返回值还需看看）

    * 差距足够小，且getLeftrs（）>fragmentSize,即至少可以分配一个碎片，则将该id加入distanceId

    * 根据碎片数量和有效在线主机数，确定结果

    * 有效在线主机数大于等于文件碎片数量：每个主机分一块

    * 小于：循环平均分配碎片

    * 不过没有做总剩余空间不够时的处理

  * 函数uploadRegister()

    * 由前端majorPage_ajax.js的encodeCallBack函数发送请求时调用此函数，可查看ajax返回后的属性使用情况来看出用了哪些返回值

  * 函数progressCheck(),但感觉没用上

  * 函数decodeFile()，但感觉没用上

  * 重申ajax返回值：在java里，因为有Tomcat的存在，利用`extends ActionSupport`可以直接把该类的示例传回前端js代码里，`var obj = $.parseJSON(databack);`而且obj对象的属性即是java类里实现了get和set方法的所有属性

    但若用rust返回，则需考虑直接返回数据，那么需要在web后端/mian.rs里考虑构造该结构体(还得考虑能否`\#[derive(Serialize, Deserialize)]`),即能否Json序列化

#### 文件系统功能实现

* 文件夹创建

  * 前端：按钮、响应动作(在当前的文件目录层级)

    点击创建：在当前层级下创建文件夹：弹出表单输入文件名，然后ajax传给server在数据库记录，然后就动态刷新当前目录。

    框架代码：https://blog.csdn.net/diyinqian/article/details/83691464

  * 后端：main函数里接收，然后调用数据库函数来往数据库加东西。最后调用GetFileList函数返回字符串

  * 问题：

    * 传的时候当前目录层级怎么清楚？
    * 多用户各自文件对应文件夹管理？
    * 管理上还是对每个文件/文件夹用FILE表，不过里面有whose和path项，每次动态刷新只需要同时对这两项筛选即可

* 文件/文件夹删除

  * 对打勾选中的文件、文件夹，删除操作：即把对应的数据库里的表项删掉。

* 文件/文件夹重命名

  * 点按钮，找到打勾选中的文件/文件夹，类似创建那样弹出表单输入新名字，然后ajax传给server在数据库记录，然后就动态刷新当前目录。



## 11.3

#### 前后端改动记录

##### GKD中前端

* index.html, majorPage.jsp基本照搬llw组

* index_ajax.js不变

* majorPage_ajax.js

  * fileDownload()：从后端要返回的信息变多了，格式包括string,int,JSONObject
  
  包括：  
  
    ```
    **private** **String** path;
    
      **private** **String** name;
    
      //用来返回结果给前端
    
      private** **String** result;
    
      **private** **JSONObject** devices;
    
      **private** **String** fileType;
    
      **private** **int** fileSize;
    
      **private** **int** noa;
    
      **private** **int** nod;
    
    
    ```
  
    
  
  * encodeCallBack()函数里还有一组ajax，从后端返回的信息参见llw组后端返回代码
  
  * filedelete():对选中的文件一次性删除；注意ajax传到后端的数据是两个数组(包含字符串)以及一个whose的cookie
  
  * filerename():对选中的第一个文件重命名，并将旧名、路径、whose、新名传回后端

##### 后端改写

* FileDownloader.rs: 

  downloadRegister()函数：

  * llw代码60-68

  * devices变量：JSONObject

    `JSONArray jsonArray = new JSONArray();`里放多个JSONObject数据

    最后再`devices.put("forms", jsonArray);`

  * 考虑直接用FileDownloader结构体的一个实例作为返回值

  progressCheck()和decodeFile()函数还没改，因为目前用不上，这些操作移到前端了。

* FileUploader.rs：pqz写的，用的是结构体方法的形式，与我写的直接用关联函数还是有一定区别，调试时需注意

* 在GetFileList下新增了filedelete，filerename，create_dir三个函数，用于对应前端三种功能的实现
* main.rs里对应接收函数都做了补充，包括数据类型
* 但感觉对一些错误处理或是NULL的处理不是很合理，考虑后续做一定优化，如Result，Option等







##### Serde_json包

* A string of JSON data can be parsed into a `serde_json::Value` by the [`serde_json::from_str`](https://docs.serde.rs/serde_json/de/fn.from_str.html) function

  ```rust
  let data = r#"
          {
              "name": "John Doe",
              "age": 43,
              "phones": [
                  "+44 1234567",
                  "+44 2345678"
              ]
          }"#;
  
      // Parse the string of data into serde_json::Value.
      let v: Value = serde_json::from_str(data)?;
  
      // Access parts of the data by indexing with square brackets.
      println!("Please call {} at the number {}", v["name"], v["phones"][0]);
  
  ```

  * v["name"]等都是借用，为&Value，打印时会带引号，去掉引号需加上.as_str(); 错误时返回Value:Null

* Serde provides a powerful way of mapping JSON data into Rust data structures largely automatically.

  ```rust
  use serde::{Deserialize, Serialize};
  use serde_json::Result;
  
  #[derive(Serialize, Deserialize)]
  struct Person {
      name: String,
      age: u8,
      phones: Vec<String>,
  }
  
  fn typed_example() -> Result<()> {
      // Some JSON input data as a &str. Maybe this comes from the user.
      let data = r#"
          {
              "name": "John Doe",
              "age": 43,
              "phones": [
                  "+44 1234567",
                  "+44 2345678"
              ]
          }"#;
  
      // Parse the string of data into a Person object. This is exactly the
      // same function as the one that produced serde_json::Value above, but
      // now we are asking it for a Person as output.
      let p: Person = serde_json::from_str(data)?;
  
      // Do things just like with any other Rust data structure.
      println!("Please call {} at the number {}", p.name, p.phones[0]);
  
      Ok(())
  }
  ```

  * 所有实现了#[derive(Serialize, Deserialize)]的rust类型都可以直接由    let p: Person = serde_json::from_str(data)?;来赋值

* Constructing Json Values

  ```rust
  use serde_json::json;
  
  fn main() {
      // The type of `john` is `serde_json::Value`
      let john = json!({
          "name": "John Doe",
          "age": 43,
          "phones": [
              "+44 1234567",
              "+44 2345678"
          ]
      });
  
      println!("first phone number: {}", john["phones"][0]);
  
      // Convert to a string of JSON and print it out
      println!("{}", john.to_string());
  }
  ```

  ```rust
  let full_name = "John Doe";
  let age_last_year = 42;
  
  // The type of `john` is `serde_json::Value`
  let john = json!({
      "name": full_name,
      "age": age_last_year + 1,
      "phones": [
          format!("+44 {}", random_phone())
      ]
  });
  ```

* A data structure can be converted to a JSON string by [`serde_json::to_string`](https://docs.serde.rs/serde_json/ser/fn.to_string.html). There is also [`serde_json::to_vec`](https://docs.serde.rs/serde_json/ser/fn.to_vec.html) which serializes to a `Vec<u8>` and [`serde_json::to_writer`](https://docs.serde.rs/serde_json/ser/fn.to_writer.html) which serializes to any `io::Write` such as a File or a TCP stream.

  ```rust
  use serde::{Deserialize, Serialize};
  use serde_json::Result;
  
  #[derive(Serialize, Deserialize)]
  struct Address {
      street: String,
      city: String,
  }
  
  fn print_an_address() -> Result<()> {
      // Some data structure.
      let address = Address {
          street: "10 Downing Street".to_owned(),
          city: "London".to_owned(),
      };
  
      // Serialize it to a JSON string.
      let j = serde_json::to_string(&address)?;
  
      // Print, write to a file, or send to an HTTP server.
      println!("{}", j);
  
      Ok(())
  }
  ```

  