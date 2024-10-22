# 项目介绍

一个使用rust+bevy引擎开发的2d开放世界沙盒游戏，此为极简demo版本。
![alt text](./assets/docs/intro-pic.png)

你可以前往release前往查找适合你的可执行文件，如果没用，你可以按照快速开始的方法编译。

此项目将 ~~不更新~~ 不定期更新

# 快速开始

1. clone此仓库。（如果需要启动器，还需要clone仓库launcher-2d，前往我的github主页查看）
2. 按照[rust官方教程](https://www.rust-lang.org/zh-CN/learn/get-started)下载并安装rust最新版本
3. 通过命令行打开到此项目根目录
4. 执行命令`cargo build --release`并等待编译完成
5. 在`根目录/target/release`中查找可执行文件

** 如果想要配置启动器，请参照以下方法 **

5. 通过命令行代开到launcher-2d项目根目录
6. 执行命令`cargo build --release`并等待编译完成
7. 将编译好的游戏本体的release目录重命名为bin并复制/移动到`此项目根目录/target/release`中
8. 在`此项目根目录/target/release`中查找launcher-2d可执行文件

* 关于如何更改可用方块数量，请参见0.3.1更新内容 *

# 玩法教程

1. 移动：a向左， d向右
2. 鼠标：使用esc键呼出/取消呼出鼠标
3. 鼠标左键：删除方块
4. 鼠标右键：放置方块
5. 跳跃：空格键
6. 潜行：左shift键（目前潜行没有任何用途，仅仅减慢移动速度，~~ 狗都不用 ~~）
7. 切换方块：鼠标滚轮/键盘1-5

# 重要更新内容
## 0.3.1更新内容
1. 可快速更改可用方块数量
 - 对于启动器：通过选择无限方块或64方块更改
 - 对于仅游戏：通过加入参数`--creative`可以开启无限方块模式，否则为64方块
## 0.3.0更新内容
1. 新增了启动器
2. 可通过键盘切换手持物品

# 未来更新

不知道，不清楚，不明白

# 已知bug

我要是都已知bug了我还能不修吗？
