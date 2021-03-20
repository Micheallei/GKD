## 测试目标

* 多个小文件、大文件上传下载速度与ceph对比
* 局域网（vlab）、VPN上测试
* 找论文，性能测试通用指标，有目标地优化（论文不需要看懂）

## 项目运行debug记录

* setup.ini中第四行需要跟代码中的websocket端口一致
* delete缺陷
* 不能下载文件夹
* 不能一次上传多个文件/上传文件夹
* 访问控制：服务器/浏览器和存储结点交互时没有token验证，安全性问题；一般用公钥加密就可以；调用现成的库
* 编解码多线程优化，最好多核（这条重要）
* 尽量不要涉及内核态的操作，注意系统调用，用trace工具检查
* 根据用户需求（可用性指标几个九），定制化编码，根据概率（可用性要求、机器可用性、设备在线概率）算需要编码多少冗余块-是否需要增加数据库表项？需要用户输入要求？在编解码调用RSE时查数据库确定参数？

## 运行llw组webcontent的步骤

* 1. 将mysql相关的写死的密码改成本机密码（搜索替换“201314”），运行mysql，`mysql -u root -p < mysql.sql
`导入mysql.sql数据库
* 2. 将setup.ini中暂存文件碎片的文件夹改为本机上设置的文件夹。
* 3. 在intellij中打开webcontent。打开Run/Debug Configurations，添加local Tomcat Server，在Application Server Configure 中选择本机tomcat文件夹。将上方Name改为DFS_server。在Deploy中添加war exploded。
* 4. 依次运行server, client, WebContent。

## 尝试改写使用rust-wasm纠删码

### 目前问题

* 主要是typescript和javascript不兼容的问题。另外也不能成功import文件夹。详见“改写尝试过程”最后两条。

### 改写尝试过程

* 在majorpage_ajax.js首部加上：
~~~
import {ReedSolomonErasure} from "./reed-solomon-erasure.wasm";
const reedSolomonErasure = ReedSolomonErasure.fromCurrentDirectory();
~~~
参考自RSE.wasm Readme.md示例代码中的前两行。由于报错await关键字只有在异步函数中可以使用，将第二行代码中的await删去。
* encodeFile()更改。
  * 调用encode前将 numOfAppend 从2改为5 ，改为与numOfDivision相同
* decodeFile()更改。
* 由于import报错，在major_page.jsp中指明majorPage_ajax.js是一个module。`<script type="module" src="../js/majorPage_ajax.js"></script>`
* 浏览器console输出：
~~~
Loading module from “http://localhost:8080/WebContent_war_exploded/js/reed-solomon-erasure.wasm/” was blocked because of a disallowed MIME type (“text/html”).
~~~
查看代码得到，将我们需要import的内容export的文件是src/index.ts，将import的内容从文件夹改为这个文件
* 由于报错，在web.xml中声明.ts文件的MIME类型是application/javascript
* 报错interface is reserved，查资料发现javascript和typescript中interface的用法不同，由于MIME类型被声明成JS，这里的interface被识别成了代码块名字。
* 尝试将ts文件MIME类型改为text/x.typescript, application/x.typescript, application/prs.typescript, text/plain，都报错disallowed MIME type。
* 查资料看到，typescript是javascript的超集，一般不会直接使用，一般是编译为javascript后使用。
* 查import文件夹，得到这是Node.js模块系统的约定实现，实际import的是该文件夹下的package.json中main字段中的文件（没有package.json则在那个目录下寻找名为index.js的文件）。在reed-solomon-erasure.wasm/package.json中main文件是dist/index.js。本来没有dist/子目录。
* 另外在package.json中找到了一段编译的脚本，尝试按照该脚本生成dist/子目录。（这里wasm-build的那条指令，我在ubuntu中运行会报网络错误，在windows中却可以成功运行）在运行`tsc -b`编译typescript时，报错不存在-b选项，man tsc查看，确实没有。尝试运行了一下在手册中看到的比较接近的`tsc -d`，`tsc -p`（这里没有及时查看文件夹，导致不知道之后生成的dist子目录到底是哪条指令生成的）。
* 生成了dist子目录，将import的对象从src/index.ts改为dist/index.js，报错`Uncaught SyntaxError: import not found: ReedSolomonErasure`。查看发现index.js中已经没有export的内容。原先的index.ts中export的是ReedSolomonErasure类，查资料得到，javascript中没有类，类的概念是typescript相对JS增加的。
* 将文件夹名字最后的`.wasm`改为`-wasm`后再次尝试import文件夹，报同样的错。不知道为什么文件夹类型会是text/html。


### llw组原go-wasm相关代码阅读

* 没有找到callMD5函数的调用，但有 objectHash.MD5() 调用

* /WebContent/webapp/jsp/major_page.jsp 中加载wasm_exec.js ，该文件(Go官方 wasm-Executor 文件)完成 JS 与Go之间的数据交换。之后通过 WebAssembly.instantiateStreaming(fetch()) 载入了 wasm 二进制文件，实例化 wasm 模块。

  major_page.jsp 中也载入了 /js/ec/ 中三个文件。

* /WebContent/webapp/js/majorPage_ajax.js中 callEncoder 等调用，encodeFile 等函数定义。

  本文件中使用 HTML5 API FileReader 读取文件。本文件被载入 major_page.jsp 。

  

* /WebContent/webapp/js/majorPage_ajax.js encodeFile() 函数

  读取文件，将文件内容转化为 Uint8Array，调用 callEncoder()

  ~~~js
  var fileString = evt.target.result;
  let raw = new Uint8Array(fileString);
  content = callEncoder(raw,numOfDivision,numOfAppend);
  ~~~

  

预计：主要修改 majorPage_ajax.js，将 callEncoder 等函数调用改为 rust-wasm 对应接口的函数调用；少量修改 major_page.jsp，将其中 Go 语言对应的 wasm 实例化改为 rust-wasm 的实例化。

对于.js文件加载、依赖关系、wasm_exec.js 具体功能、/ec下的 js 文件的具体功能还有不清楚的地方。major_page.jsp 为什么先调用 Go 函数，后实例化 .wasm 文件？

rust-wasm 示例中 import from @subspace 是否和 npm 相关，直接在 npm 文件夹中查找文件？能否复制文件，然后改为相对路径import？


## Rust+WebAssembly+ErasureCode

Go 代码中，我们接收 Uint8Array 类型数据，并提供三个函数给 JavaScript 使用：
callEncoder 用于编码；
callDecoder 用于解码；
callMd5 用于计算碎片的 MD5 值，从而在解码时判断碎片内容是否发生了改变

完成数据类型的转换和 Go 函数的调用 。



https://github.com/subspace/reed-solomon-erasure.wasm

js中的调用：

* fromCurrentDirectory()

  fromCurrentDirectory(source: Response)

  fromBytes(bytes: BufferSource)

  都是初始化函数，载入.wasm二进制文件。预计按照示例使用第一个函数就可以了。

  它完成了 WebAssembly.instantiateStreaming() 的功能。

* encode(shards: Uint8Array, dataShards: number, parityShards: number): number

  编码。参数为包含data, parity shards的Uint8Array，两个部分的长度。shards数组包括数据以及为冗余码预留的空间。在shards空间内原地编码，成功则返回`ReedSolomonErasure.RESULT_OK`

* ReedSolomonErasure.reconstruct(shards: Uint8Array, dataShards: number, parityShards: number, shardsAvailable: boolean[]): number

  解码。需要获取可用碎片的编号。

java-(wasm-)rust数据传输内容，格式





## llw组项目笔记

### IPFS、NAS对比借鉴

* IPFS：创建了分布式存储和共享文件的网络传输协议

  区块链技术：相比中心化服务，文件更难被篡改和封禁

  文件传输：P2P

  无身份验证，拿到哈希值就能拿到文件；数据可靠性不足

* NAS：可以通过网络访问的专用数据存储服务器

  系统使用专有设备，兼容性不足

### P2P

* 1、集中式对等网络（Napster、QQ）

  中央目录服务器为网络中各节目提供目录查询服务，传输内容无需再经过中央服务器。

  本项目可能属于这一类别。

* 2、无结构分布式网络（Gnutella）

  没有中央服务器，所有结点通过与相邻节点间的通信，接入整个网络。节点采用查询包机制来搜索需要的资源，查询包以扩散的方式在网络中蔓延，有TTL。

* 3、结构化分布式网络（第三代P2P Pastry、Tapestry、Chord、CAN）

  基于分布式哈希表。

  将网络中所有的资源整理成一张巨大的表，表内包含资源的关键字和所存放结点的地址，然后将这张表分割后分别存储到网络中的每一结点中去。

* BitTorrent、迅雷、Skype

另：参见计算机网络课程内容。

### 项目结构-与17级项目相比、去中心化

一个目录节点和若干存储节点组成。P2P 的传输架构，目录节点后端的容器化。

目录节点：MySQL，Tomcat-Java-web，docker部署。提供 web 管理界面，以及协调各存储节点的资源。

目录节点对访问者进行身份验证，对数据的冗余备份进行协调。

### docker

容器把应用及其依赖项都将打包成一个可以复用的镜像并与其他进程环境隔离。

容器使开发环境和运行环境统一。同时容器并不像虚拟机那样模拟全部硬件，只虚拟化了文件系统、网络、运行环境等，在核心本地运行指令，不需要任何专门的接口翻译和系统调用替换机制，减少虚拟化开销。

### websocket

* 建立在TCP协议之上。与 HTTP 协议有着良好的兼容性。
* 双向信息传输、避免客户端获取信息时轮询。

### 文件分配策略、用户权限设置

* 提高下载成功概率：考虑设备和用户在线时间重合度

  将一个设备或者一个用户一天中的在线时间表示成一个长度为 24 的 0/1 向量，在上传文件时尽可能地给覆盖上传者的在线时间段 x% 以上的存储结点分配碎片。

* 碎片不会再分给剩余容量到达上限的节点，避免分配出现严重的倾斜。

* 实现私有文件系统：

  表 FILE 增加 WHOSE 列，记录文件所属。

  登录时，输入用户名、密码，检测对应模块，跳转至特定文件空间（涉及到由抓取所有文件列表到抓取特定文件列表的转变）



文件编解码部分、浏览器实现

兼容性：JVM，docker解决

一键部署？



## 关于 token

在/dontpanic仓库文件夹中搜索 token，出现过的两个文件判断都不是自己写的文件：

/src/web/WebContent/src/main/webapp/js/bootstrap-3.3.7/js/bootstrap.js （一万多行）

/src/web/WebContent/src/main/webapp/js/jquery/jquery.js （两千多行）

搜索base64，jwt等内容没有相应结果（base64的一个搜索结果是关于http握手的），我认为jwt没有实现。



seed-realworld中token：

在src/coder/decoer/viewer结构体中有token字段，src/entity/viewer中有auth_token字段，都为String，但没有生成算法和代码。使用到的地方基本都是初始化的代码。src/request中可能是生成/传递token的代码：

```rust
use seed::fetch;

pub fn new(path: &str, viewer: Option<&Viewer>) -> fetch::Request {
    let mut request = fetch::Request::new(format!("{}/{}", BASE_API_URL, path)).timeout(TIMEOUT);

    if let Some(viewer) = viewer {
        let auth_token = viewer.auth_token.as_str();
        request = request.header("authorization", &format!("Token {}", auth_token));
    }
    request
}
```

src/coder/decoer/viewer中tests模块中，token字段值写死为"John's token"。



session和jwt：

session：当用户首次与Web服务器建立连接的时候，服务器会给用户分发一个 SessionID作为标识。SessionID是一个由24个字符组成的随机字符串。

客户端接收到用户名和密码后并判断，如果正确就将本地获取sessionID作为Token返回给客户端，客户端以后只需带上请求数据即可。当session过期后，客户端必须重新登录。

https://blog.csdn.net/java_faep/article/details/78082802

访问量不是非常大时，cookie-session存储足够，jwt不是必须的。

https://www.jianshu.com/p/af8360b83a9f


