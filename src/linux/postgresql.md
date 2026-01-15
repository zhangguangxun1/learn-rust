# 安装 postgresql

## Arch Linux

在 Arch Linux 上安装 PostgreSQL 的过程与大多数发行版略有不同，因为它在安装后不会自动初始化数据库

```bash
sudo pacman -S postgresql
```

Arch Linux 不会自动创建数据库实例。你必须先切换到自动生成的 postgres 用户并手动初始化

```bash
# 切换到 postgres 用户
sudo -iu postgres

# 初始化数据库集群 (默认路径为 /var/lib/postgres/data)
initdb -D /var/lib/postgres/data

# 退出 postgres 用户回到普通用户
exit
```

初始化完成后，使用 systemctl 启动数据库服务

```bash
sudo systemctl enable --now postgresql
```

替换为 start 后不用开机就启动, 需要手动起服务即可

默认情况下，你需要以 postgres 用户的身份进入 psql 交互界面

```bash
sudo -iu postgres psql
```

数据目录： /var/lib/postgres/data/

主配置文件： /var/lib/postgres/data/postgresql.conf

权限配置文件： /var/lib/postgres/data/pg_hba.conf

创建一个用户和数据库

```sql
-- 1. 创建数据库
CREATE DATABASE open_tiku_test;

-- 2. 创建用户并设置密码（请将 'your_password' 替换为你的实际密码）
CREATE USER tiku_rw WITH ENCRYPTED PASSWORD 'your_password';

-- 3. 将数据库的所有权转让给该用户（这样该用户默认拥有所有读写权限）
ALTER DATABASE open_tiku_test OWNER TO tiku_rw;

-- 4. 授予该用户连接数据库的权限
GRANT CONNECT ON DATABASE open_tiku_test TO tiku_rw;
```

如果你不想让该用户成为 Owner，只想授予特定 Schema 的读写权限，请在连接到该数据库后执行

```sql
\c open_tiku_test
GRANT ALL PRIVILEGES ON SCHEMA public TO tiku_rw;
```

使用新用户登录该数据库，验证权限是否正确

```bash
psql -d open_tiku_test -U tiku_rw -W
```

修改密码

```bash
-- 将 tiku_rw 替换为你要修改的用户名，'new_password' 替换为新密码
ALTER USER tiku_rw WITH PASSWORD 'new_password';
```

## Mac OS

安装

```
brew install postgresql@18
```

配置环境变量

```
echo $SHELL
```

如果是 zsh 则类似拷贝安装完成后的提示

```
echo 'export PATH="/usr/local/opt/postgresql@18/bin:$PATH"' >> ~/.zshrc
```

手动启动

```
brew servces start postgresql@18
```

连接数据库服务

```
psql postgres
```

创建用户并设置密码

```
CREATE USER tiki_rw WITH PASSWORD '你的安全密码';
```

创建数据库并指定所有者为新用户

```
CREATE DATABASE open_tiku_test OWNER tiki_rw;
```

赋予管理权限

虽然 OWNER 已经拥有很大权限，但为了确保该用户可以完全管理该数据库（如创建模式等），建议执行

```
GRANT ALL PRIVILEGES ON DATABASE open_tiku_test TO tiki_rw;
```

使用新用户连接测试

```
psql -U tiki_rw -d open_tiku_test -h localhost
```
