## 给当前用户赋予数据库用户权限

以你的普通 Linux 用户名，在数据库中创建同名的用户和数据库，如 uName，然后就可以本机直接连接到数据库 uName 了。

~> su - postgres
Password:
Last login: Wed Mar 1 13:19:02 CST 2017 on pts/1
-bash-4.2$ psql
psql (9.2.18)
Type "help" for help.

postgres=# create user uName with password '**\*\***';
CREATE ROLE
postgres=# create database uName owner uName;
CREATE DATABASE
postgres=# grant all privileges on database uName to uName;
GRANT
postgres=# \q
-bash-4.2$ exit
logout
~> psql
psql (9.2.18)
Type "help" for help.

uName=>

至此，就在数据库 uName 中了。

> select session_user, current_user;

session_user: 链接进数据库的用户

current_user: 当前操作的用户
