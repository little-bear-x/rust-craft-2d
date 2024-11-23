# 项目介绍

一个使用rust+bevy引擎开发的2d开放世界沙盒游戏，此为极简demo版本。
![alt text](./assets/docs/intro-pic.png)

启动器仓库: https://github.com/little-bear-x/rust-craft-2d-launcher

你可以前往release前往查找适合你的可执行文件，如果没用，你可以按照快速开始的方法编译。

此项目可能不会积极更新

# 快速开始

## 方案1:查找可执行文件

请前往release界面查找是否有适合你的可执行文件。如果没有，请自行编译

## 自行编译

1. clone此仓库。
2. 按照[rust官方教程](https://www.rust-lang.org/zh-CN/learn/get-started)下载并安装rust最新版本
3. 通过命令行打开到此项目根目录
4. 执行命令`cargo build --release`并等待编译完成
5. 在`path/to/game/target/release`中查找game可执行文件（你可以选择将该目录中的文件移动到其他目录下）
6. 将项目中的`SavedGames`文件夹和`assets`文件夹复制到game可执行文件的目录下
7. 你将可以执行文件

**在自行编译时，对于启动器，请使用以下方法配置**

1. 在完成上述操作的基础上，clone rust-craft-2d-launcher仓库[此处查看](https://github.com/little-bear-x/rust-craft-2d-launcher)

2. 前往[python官网](python.org)下载安装python（推荐3.12）

3. 安装完成后，使用pip安装pyinstaller

4. 在项目根目录下执行以下命令，你将可以在`path/to/launcher/dist`下找到launcher可执行文件

```bash
pyinstaller -F -w main.py
```

5. 将打包好的launcher可执行文件复制到game可执行文件的目录下

6. 你将可以使用启动器

**注: 你也可以使用其他打包方式打包启动器**

# 玩法教程

1. 移动：a向左， d向右
2. 鼠标：使用esc键呼出/取消呼出鼠标
3. 鼠标左键：删除方块
4. 鼠标右键：放置方块
5. 跳跃：空格键
6. 潜行：左shift键（目前潜行没有任何用途，仅仅减慢移动速度，~~ 狗都不用 ~~）
7. 切换方块：鼠标滚轮/键盘1-5

# 重要更新内容

## 0.4.0 更新内容
1. [新增][重要]新增随即地形生成, 现在, 请通过以下参数设置随即地形种子(打开游戏时无需设置)
```bash
--seed=<seed>  # 地形生成种子, 种子类型应为i32, 当seed<0时, 为超平坦地形
```
2. [更新]启动器更新
3. [警告]旧版本存档可能无法应用到新存档中!

## 0.3.2更新内容

1. [新增][重要]完成游戏保存与读取功能，请使用下列命令创建/读取游戏。

   ```bash
   /path/to/game --new game-name  # 新建游戏，务必添加--gamemode参数
   /path/to/game --open game-name  # 读取游戏
   ```

   注意：
- 建议使用启动器启动，以免出现意外错误
- 游戏存档为json格式，但不建议修改游戏存档，避免存档损坏
2. [更新][重要]使用`--gametype`代替`--creative`你可以使用如下命令（如果你不使用启动器）来启动游戏：

   ```bash
   /path/to/game --gametype sandbox  # 启动沙盒模式
   /path/to/game --gametype survival  # 启动有限方块模式
   ```
3. [更新]完善了游戏日志输出
4. [更新]游戏启动器使用python重写
5. [修复]修复了游戏关闭时的panic

# 未来更新

不知道，不清楚，不明白

# 已知bug

我要是都已知bug了我还能不修吗？
