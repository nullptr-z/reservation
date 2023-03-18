## 给当前用户赋予数据库用户权限

普通 Linux 用户名，在数据库中创建同名的用户和数据库，如 zheng，然后就可以本机直接连接到数据库 zheng 了。

```sql
> su - postgres

> psql

postgres=# create user zheng with password 'z';
CREATE ROLE
postgres=# create database zheng owner zheng;
CREATE DATABASE
postgres=# grant all privileges on database zheng to zheng;
GRANT
postgres=# \q

> psql zheng

zheng=>
```

## 赋予超级用户权限

ALTER USER zheng WITH SUPERUSER;

```sql
 select session_user, current_user;

-- session_user: 链接进数据库的用户
-- current_user: 当前操作的用户
```

## deepin 安装 postgres 15 或更高版本

1. sudo apt remove postgresql --purge 删除旧版本
2. sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt buster-pgdg main" > /etc/apt/sources.list.d/pgdg.list' 创建官方源配置文件
3. wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add - 导入签名
4. sudo apt update 更新仓库
5. sudo 安装

## postgres 账户被锁定

```sh
sudo -u postgres psql postgres

# password postgres

Enter new password:
```

稍微解释一下.登录 PostgreSQL 服务器通常有两种默认方式：

> 通过以 UNIX 用户身份运行“psql”命令(所谓的 IDENT / PEER 身份验证),例如：sudo -u postgres psql.请注意,sudo -u 不会解锁 UNIX 用户.
