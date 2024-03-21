My simple RPG server.
* written by RUST.
* mysql database
* redis 
* make sea-orm entity:

```shell
sea-orm-cli generate entity -u mysql://morioka:moe@localhost/morioka -o entity/src -l
```
* configure
```shell
server host：  
HOST=127.0.0.1  
server port：  
PORT=8000  
mysql database url:  
DATABASE_URL="mysql://morioka:moe@localhost/morioka"  
encryption key：  
ENCRYPTION_KEY="8ea8593bb2e44ccda1ccbb1fa07db5b6"  
redis url：  
REDIS_URL="redis://127.0.0.1/" 
```
 