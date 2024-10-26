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

# 玩法教程

1. 移动：a向左， d向右
2. 鼠标：使用esc键呼出/取消呼出鼠标
3. 鼠标左键：删除方块
4. 鼠标右键：放置方块
5. 跳跃：空格键
6. 潜行：左shift键（目前潜行没有任何用途，仅仅减慢移动速度，~~ 狗都不用 ~~）
7. 切换方块：鼠标滚轮/键盘1-5

# 重要更新内容
## 0.3.2更新内容
1. [新增][重要]完成游戏保存与读取功能，请使用下列命令创建/读取游戏。
```bash
/path/to/game --new game-name  # 新建游戏，务必添加--gamemode参数
/path/to/game --open game-name  # 读取游戏
```
注意：
- 建议使用启动器启动，以免出现意外错误
- 游戏存档为json格式，但不建议修改游戏存档，避免存档损坏
2. [更新][重要]使用`--gamemode`代替`--creative`你可以使用如下命令（如果你不使用启动器）来启动游戏：
```bash
/path/to/game --gamemode sandbox  # 启动沙盒模式
/path/to/game --gamemode survival  # 启动有限方块模式
```
3. [更新]完善了游戏日志输出
4. [更新]游戏启动器使用python重写
5. [修复]修复了游戏关闭时的panic
## 0.3.0更新内容
1. 新增了启动器
2. 可通过键盘切换手持物品

# 未来更新

不知道，不清楚，不明白

# 已知bug

我要是都已知bug了我还能不修吗？
