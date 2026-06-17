# find 详细使用手册

[toc]

## 1.命令格式

```bash
find [-H] [-L] [-P] [-Olevel] [-D debugopts] [path...] [expression]
```

| 组成部分         | 说明                                |
| ---------------- | ----------------------------------- |
| `[-H] [-L] [-P]` | 符号链接处理策略（三选一）          |
| `[-Olevel]`      | 查询优化级别（0-3）                 |
| `[-D debugopts]` | 调试输出                            |
| `[path...]`      | 搜索路径（默认当前目录 `.`）        |
| `[expression]`   | 表达式：操作符 + 选项 + 测试 + 动作 |

## 2. 符号链接

| 参数 | 全称         | 作用                             | 通俗理解                       |
| :--- | :----------- | :------------------------------- | :----------------------------- |
| `-P` | Physical     | **从不跟随符号链接**（默认行为） | 把链接当普通文件，只看链接本身 |
| `-L` | Logical      | **总是跟随符号链接**             | 穿透链接，看链接指向的实际内容 |
| `-H` | Half-logical | **仅对命令行传入的路径跟随链接** | 对起点特殊处理，递归时不跟随   |

```bash
# 创建测试环境
mkdir -p ./test/real_dir
touch ./test/real_dir/test.log
cd ./test
ln -s real_dir link_dir
cd ../
$ tree test
test
├── link_dir -> real_dir  （这不一定，确保链接地址正确就好了）
└── real_dir
    └── test.log

1 directory, 2 files

# find -P（默认）：找不到
find ./test/link_dir -name "*.log"        # 无输出

# find -L：可以找到
find -L ./test/link_dir -name "*.log"     # 输出：./test/link_dir/test.log

# find -H：只对命令行传入的路径跟随链接
find -H -name "*.log" 				  # 输出：./test/real_dir/test.log（没有link_dir目录下的）
find -H ./test/link_dir -name "*.log"     # 输出：./test/link_dir/test.log
```

## 3. 操作符（优先级从高到低）

| 操作符             | 说明                 | 示例                                              |
| :----------------- | :------------------- | :------------------------------------------------ |
| `( EXPR )`         | 括号分组，改变优先级 | `find . \( -name "*.c" -o -name "*.h" \) -type f` |
| `! EXPR`           | 逻辑非               | `find . ! -name "*.txt"`                          |
| `-not EXPR`        | 同 `!`               | `find . -not -empty`                              |
| `EXPR1 -a EXPR2`   | 逻辑与（默认）       | `find . -name "*.rs" -a -size +1k`                |
| `EXPR1 -and EXPR2` | 同 `-a`              | 同上                                              |
| `EXPR1 -o EXPR2`   | 逻辑或               | `find . -name "*.c" -o -name "*.cpp"`             |
| `EXPR1 -or EXPR2`  | 同 `-o`              | 同上                                              |
| `EXPR1 , EXPR2`    | 逗号操作符，依次执行 | `find . -name "*.tmp" , -delete`                  |

```bash
# 找 "*.c" 或 "*.h" 的文件
find . \( -name "*.c" -o -name "*.h" \) -type f 
# 找不是 "*.txt" 的文件
find . ! -name "*.txt"  #同 find . -not -name "*.txt"
# 找 "*.rs" 且 大小大于lk的文件
find . -name "*.rs" -a -size +1k

# 查找当前目录下的"*.h"文件，删除当前目录下的所有文件
find . -name "*.h" , -delete
# 查找当前目录下的"*.h"文件，再删除"*.h"文件
find . -name "*.h" -a -delete
```

## 4. 位置选项（always true，影响后续表达式）

| 选项              | 说明                                                         |
| :---------------- | :----------------------------------------------------------- |
| `-daystart`       | 时间计算从当天开始而非24小时前                               |
| `-follow`         | 同 `-L`（已废弃，用 `-L`）                                   |
| `-regextype TYPE` | 设置正则类型（`emacs/posix-awk/posix-basic/posix-egrep/posix-extended`） |

* `-daystart` 会影响所有以天为单位和以分钟为单位的时间测试，一般搭配 `-atime、-ctime、-mtime、-amin、-cmin、-mmin`这些选项一起使用才会生效

```bash
# 不加 -daystart：查找过去24小时内修改的文件
find . -mtime -1

# 加 -daystart：查找今天（从00:00到现在）修改的文件
find . -daystart -mtime -1
```

* `-regextype TYPE`参数设置 `-regex` 和 `-iregex` 使用的正则表达式语法类型

| 类型             | 说明                    | 示例                       |
| :--------------- | :---------------------- | :------------------------- |
| `emacs`          | **默认**，Emacs风格正则 | `find . -regex ".*\.txt$"` |
| `posix-awk`      | AWK风格正则             | 支持 `\y` 词边界           |
| `posix-basic`    | 基本正则(BRE)           | 需要转义 `\(\)` `\{\}`     |
| `posix-egrep`    | 扩展正则(ERE)           | 不需要转义，更简洁         |
| `posix-extended` | 同posix-egrep           |                            |

```bash
# emacs 风格需要转义花括号
find . -regex ".*\.\(txt\|log\|md\)$"     # 需要转义 {,}

# 而扩展正则不需要转义
find . -regextype posix-egrep -regex ".*\.(txt|log|md)$"
```

## 5. 普通选项（always true，需放在表达式前面）

| 选项                     | 说明                                       | 示例                              |
| :----------------------- | :----------------------------------------- | :-------------------------------- |
| `-depth`                 | 深度优先遍历（先处理子目录）               | `find . -depth -type d`           |
| `-maxdepth LEVELS`       | 最大搜索深度                               | `find . -maxdepth 2 -name "*.rs"` |
| `-mindepth LEVELS`       | 最小搜索深度                               | `find . -mindepth 2 -type f`      |
| `-mount`                 | 不进入其他文件系统                         | `find / -mount -name "*.log"`     |
| `-noleaf`                | 针对无 `.` 和 `..` 的文件系统（如 CD-ROM） |                                   |
| `-xdev`                  | 同 `-mount`                                |                                   |
| `-xautofs`               | 不进入 autofs 挂载点                       |                                   |
| `-ignore_readdir_race`   | 忽略目录在读取时消失的错误                 |                                   |
| `-noignore_readdir_race` | 不忽略上述错误（默认）                     |                                   |

```bash
## -depth - 深度优先遍历
# 先处理子目录内容，再处理父目录
find . -depth -type d

# 实际场景：递归删除所有文件和目录（rm -r 的替代）
find . -depth -exec rm -rf {} \;
# 对比：不加 -depth 会先处理父目录
find . -type d          # 先输出 .，再输出 ./subdir
find . -depth -type d   # 先输出 ./subdir，再输出 .

## -maxdepth - 最大搜索深度
# 限制深度避免递归过深
find /home -maxdepth 3 -type d -name ".*"  # 查找隐藏目录

## -mindepth - 最小搜索深度
# 跳过当前目录，只查找子目录中的文件
find . -mindepth 2 -type f

# 查找至少两层深的配置文件
find /etc -mindepth 3 -name "*.conf"

# 结合使用：查找深度在2-4之间的文件
find . -mindepth 2 -maxdepth 4 -type f -name "*.log"

## -mount / -xdev - 不进入其他文件系统
# 只搜索根分区的日志，不进入 /home、/var 等独立分区
find / -mount -name "*.log" -size +1M
# 实际场景：清理当前分区日志，不影响其他分区
find /var -mount -name "*.log" -mtime +30 -delete
```

有 `.` 和 `..` 的文件系统，通常也提供可靠的 `d_type`，让 `find` 能直接识别文件类型而无需对每个文件调用 `stat()`，而没有 `.` 和 `..` 的文件系统 `d_type` 不可靠，所以要指定 `-noleaf` 参数，否则搜索结果会有误

```bash
# 在 CD-ROM 上，不加 -noleaf
find /mnt/cdrom -name "*.mp3"
# find 的错误假设：
# 1. 认为每个目录都有 . 和 ..
# 2. 认为 d_type 可靠
# 3. 用 nlink 优化（但 CD-ROM 的 nlink 是 1，不是预期的 2+）

# 可能的结果：
# - 某些目录被跳过
# - 把普通文件当成目录（或反之）
# - 提前结束扫描（因为 nlink 不对）
# - 返回不完整或错误的结果

find /mnt/cdrom -noleaf -name "*.mp3"
# 告诉 find：
# 1. 不要假设有 . 和 ..
# 2. 不要用 nlink 优化
# 3. 对每个条目都 stat 一下，老老实实识别类型

# 结果：正确但慢
```

## 6. 比较测试（返回 true/false）

### 6.1 时间相关

| 测试           | 说明                       | 示例                                 |
| :------------- | :------------------------- | :----------------------------------- |
| `-amin N`      | 访问时间在 N 分钟前        | `find . -amin +60`（60分钟前访问过） |
| `-atime N`     | 访问时间在 N*24 小时前     | `find . -atime -7`（7天内访问过）    |
| `-cmin N`      | 状态改变时间在 N 分钟前    | `find . -cmin -10`                   |
| `-ctime N`     | 状态改变时间在 N*24 小时前 | `find . -ctime +30`                  |
| `-mmin N`      | 修改时间在 N 分钟前        | `find . -mmin +5`                    |
| `-mtime N`     | 修改时间在 N*24 小时前     | `find . -mtime -1`（今天修改的）     |
| `-newer FILE`  | 比指定文件更新             | `find . -newer reference.txt`        |
| `-anewer FILE` | 访问时间比指定文件新       |                                      |
| `-cnewer FILE` | 状态改变时间比指定文件新   |                                      |

> `N` 可以是 `+N`（大于N）、`-N`（小于N）、`N`（等于N）

```bash
# 查看文件状态
$ stat Cargo.lock
  文件：Cargo.lock
  大小：11223           块：24         IO 块：4096   普通文件
Device: 253,0   Inode: 1594891     Links: 1
权限：(0644/-rw-r--r--)  Uid：( 1001/postgres)   Gid：( 1001/postgres)
环境：unconfined_u:object_r:user_home_t:s0
最近访问：2026-06-11 10:31:40.597898528 +0800  # atime
最近更改：2026-06-11 10:30:18.290624863 +0800  # mtime
最近改动：2026-06-11 10:30:18.290624863 +0800  # ctime
创建时间：2026-06-11 10:30:18.290624863 +0800
# 假设当前时间：2026-06-12 14:30:00

# 查找60分钟前访问过的文件
find . -amin +60 # 距离现在约 28小时 = 1680分钟前, 1680 > 60，所以会被找到

# 查找1天内访问过的文件
find . -atime -1 # 6月11日10:31不在6月11日14:30 - 6月12日14:30 范围内,不会被找到

# 查找10分钟内状态改变的文件
find . -cmin -10

# 查找30天前状态改变的文件
find . -ctime +30

# 查找5分钟前修改的文件
find . -mmin +5

# 查找今天修改的文件
find . -mtime -1

# 查找比 reference.txt 新的文件
find -maxdepth 1 -newer reference.txt

# 查找访问时间比 reference.txt 新的文件
find . -anewer reference.txt

# 查找状态改变比 reference.txt 新的文件
find . -cnewer reference.txt

# 反向查找：查找比 Cargo.lock 旧的文件
find . -not -newer Cargo.lock
```

### 6.2 文件属性

| 测试           | 说明           | 示例                         |
| :------------- | :------------- | :--------------------------- |
| `-empty`       | 空文件或空目录 | `find . -empty`              |
| `-false`       | 总是 false     |                              |
| `-true`        | 总是 true      |                              |
| `-fstype TYPE` | 文件系统类型   | `find . -fstype ext4`        |
| `-gid N`       | 组 ID 为 N     | `find . -gid 1000`           |
| `-group NAME`  | 组名为 NAME    | `find . -group developers`   |
| `-uid N`       | 用户 ID 为 N   | `find . -uid 0`（root文件）  |
| `-user NAME`   | 用户名为 NAME  | `find . -user postgres`      |
| `-nouser`      | 无有效用户     | `find . -nouser`（孤儿文件） |
| `-nogroup`     | 无有效组       | `find . -nogroup`            |

### 6.3 名称匹配

| 测试                 | 说明                       | 示例                             |
| :------------------- | :------------------------- | :------------------------------- |
| `-name PATTERN`      | 文件名匹配（区分大小写）   | `find . -name "*.rs"`            |
| `-iname PATTERN`     | 文件名匹配（不区分大小写） | `find . -iname "*.RS"`           |
| `-path PATTERN`      | 全路径匹配                 | `find . -path "*/src/*.rs"`      |
| `-ipath PATTERN`     | 全路径匹配（不区分大小写） |                                  |
| `-wholename PATTERN` | 同 `-path`                 |                                  |
| `-regex PATTERN`     | 正则匹配全路径             | `find . -regex ".*\.(rs|toml)$"` |
| `-iregex PATTERN`    | 正则匹配（不区分大小写）   |                                  |
| `-lname PATTERN`     | 符号链接目标匹配           | `find . -lname "/tmp/*"`         |
| `-ilname PATTERN`    | 同上（不区分大小写）       |                                  |

### 6.4 文件特征

| 测试               | 说明                  | 示例                               |
| :----------------- | :-------------------- | :--------------------------------- |
| `-inum N`          | inode 号为 N          | `find . -inum 123456`              |
| `-links N`         | 硬链接数为 N          | `find . -links +1`（有多个硬链接） |
| `-size N[bcwkMG]`  | 文件大小              | `find . -size +100M`（大于100MB）  |
| `-type [bcdpflsD]` | 文件类型              | `find . -type d`（目录）           |
| `-xtype [bcdpfls]` | 同 `-type` 但跟随链接 |                                    |

**`-size` 单位**：

| 后缀 | 含义              |
| :--- | :---------------- |
| `b`  | 512字节块（默认） |
| `c`  | 字节              |
| `w`  | 双字节字          |
| `k`  | 千字节（KB）      |
| `M`  | 兆字节（MB）      |
| `G`  | 吉字节（GB）      |

**`-type` 类型**：

| 字符 | 类型             |
| :--- | :--------------- |
| `b`  | 块设备           |
| `c`  | 字符设备         |
| `d`  | 目录             |
| `p`  | 命名管道（FIFO） |
| `f`  | 普通文件         |
| `l`  | 符号链接         |
| `s`  | 套接字           |
| `D`  | 门（Solaris）    |

### 6.5 权限相关

| 测试          | 说明             | 示例                                  |
| :------------ | :--------------- | :------------------------------------ |
| `-perm MODE`  | 精确权限匹配     | `find . -perm 644`                    |
| `-perm -MODE` | 包含所有指定权限 | `find . -perm -222`（所有用户可写）   |
| `-perm /MODE` | 包含任一指定权限 | `find . -perm /111`（任一用户可执行） |
| `-readable`   | 当前用户可读     | `find . -readable`                    |
| `-writable`   | 当前用户可写     | `find . -writable`                    |
| `-executable` | 当前用户可执行   | `find . -executable`                  |

### 6.6 其他

| 测试            | 说明                                   |
| :-------------- | :------------------------------------- |
| `-used N`       | 文件在 N 天前被访问过（atime < ctime） |
| `-context TEXT` | SELinux 安全上下文匹配                 |

## 7. 动作（对匹配结果执行操作）

| 动作                    | 说明                                       | 示例                                          |
| :---------------------- | :----------------------------------------- | :-------------------------------------------- |
| `-print`                | 打印路径（默认动作）                       | `find . -name "*.rs" -print`                  |
| `-print0`               | 打印路径，以 `\0` 分隔（处理含空格文件名） | `find . -print0 \| xargs -0 rm`               |
| `-fprint FILE`          | 输出到文件                                 | `find . -fprint results.txt`                  |
| `-fprint0 FILE`         | 输出到文件，`\0` 分隔                      |                                               |
| `-printf FORMAT`        | 格式化输出                                 | `find . -printf "%p %s %t\n"`                 |
| `-fprintf FILE FORMAT`  | 格式化输出到文件                           |                                               |
| `-ls`                   | 类似 `ls -dils`                            | `find . -ls`                                  |
| `-fls FILE`             | 类似 `-ls` 但输出到文件                    |                                               |
| `-delete`               | 删除文件                                   | `find . -name "*.tmp" -delete`                |
| `-prune`                | 跳过当前目录不进入                         | `find . -path ./skip -prune -o -print`        |
| `-quit`                 | 找到第一个匹配后立即退出                   | `find . -name "target" -quit`                 |
| `-exec COMMAND \;`      | 对每个匹配执行命令                         | `find . -name "*.rs" -exec cat {} \;`         |
| `-exec COMMAND {} +`    | 批量执行（效率更高）                       | `find . -name "*.rs" -exec cat {} +`          |
| `-ok COMMAND \;`        | 同 `-exec` 但执行前确认                    | `find . -name "*.tmp" -ok rm {} \;`           |
| `-execdir COMMAND \;`   | 在匹配文件所在目录执行                     | `find . -name "*.sh" -execdir chmod +x {} \;` |
| `-execdir COMMAND {} +` | 批量在所在目录执行                         |                                               |
| `-okdir COMMAND \;`     | 同 `-execdir` 但执行前确认                 |                                               |

**`-printf` 常用格式**：

| 格式符 | 含义                              |
| :----- | :-------------------------------- |
| `%p`   | 文件路径                          |
| `%f`   | 文件名（不含路径）                |
| `%s`   | 文件大小（字节）                  |
| `%t`   | 修改时间                          |
| `%T+`  | 修改时间（完整格式）              |
| `%u`   | 用户名                            |
| `%g`   | 组名                              |
| `%m`   | 权限（八进制）                    |
| `%i`   | inode 号                          |
| `%n`   | 硬链接数                          |
| `%y`   | 文件类型（同 `ls -l` 第一个字符） |

## 8. 调试选项（`-D`）

| 选项        | 说明                     |
| :---------- | :----------------------- |
| `-D exec`   | 显示 `-exec` 执行的命令  |
| `-D opt`    | 显示表达式优化信息       |
| `-D rates`  | 显示目录遍历速率         |
| `-D search` | 显示搜索策略             |
| `-D stat`   | 显示 `stat`/`lstat` 调用 |
| `-D time`   | 显示时间计算细节         |
| `-D tree`   | 显示表达式树             |
| `-D all`    | 显示所有调试信息         |
| `-D help`   | 显示调试选项帮助         |

## 9. 常用组合示例

| 需求                     | 命令                                                        |
| :----------------------- | :---------------------------------------------------------- |
| 查找并删除 30 天前的日志 | `find /var/log -name "*.log" -mtime +30 -delete`            |
| 查找大于 100MB 的文件    | `find . -size +100M -ls`                                    |
| 查找空目录并删除         | `find . -type d -empty -delete`                             |
| 修改所有 .sh 权限        | `find . -name "*.sh" -exec chmod +x {} +`                   |
| 查找今天修改的文件       | `find . -mtime -1 -printf "%T+ %p\n"`                       |
| 排除某目录搜索           | `find . -path ./node_modules -prune -o -name "*.js" -print` |
| 安全删除（确认）         | `find . -name "*.tmp" -ok rm {} \;`                         |
| 处理含空格的文件名       | `find . -name "*.txt" -print0 \| xargs -0 cat`              |
| 查找硬链接文件           | `find . -links +1 -ls`                                      |
| 查找孤儿文件             | `find . -nouser -o -nogroup`                                |

