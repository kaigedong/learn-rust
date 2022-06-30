# 命令行执行：
# psql -U postgres

CREATE DATABASE testdb;

\l # 查看已存在的数据库
\c testdb; # 进入数据库

drop table if exists course;

create table course
(
    id serial primary key,
    teacher_id INT not null,
    name varchar(140) not null,
    time TIMESTAMP default now(),
    description varchar(2000),
    format varchar(30),
    structure varchar(200),
    duration varchar(30),
    price INT,
    language varchar(30),
    level varchar(30)
);

insert into course
       (id, teacher_id, name, time)
values(1, 1, 'First course', '2022-01-17 05:40:00');
insert into course
       (id, teacher_id, name, time)
values(2, 1, 'Second course', '2022-01-18 05:45:00');

\d # 查看表格是否创建成功
\d course; # 查看表格信息
SELECT * FROM course; # 查询表格数据
