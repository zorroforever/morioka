一个简单的RPG服务器。
* RUST开发
* mysql数据库
* redis缓存
* make sea-orm entity:

```shell
sea-orm-cli generate entity -u mysql://morioka:moe@localhost/morioka -o entity/src -l
```
服务器地址：
HOST=127.0.0.1
服务器端口：
PORT=8000
数据库地址：
DATABASE_URL="mysql://morioka:moe@localhost/morioka"
加密key：
ENCRYPTION_KEY="8ea8593bb2e44ccda1ccbb1fa07db5b6"
redis地址：
REDIS_URL="redis://127.0.0.1/"