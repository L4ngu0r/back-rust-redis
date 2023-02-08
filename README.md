## Use docker-compose to launch stack :

```
docker-compose up -d
```
Or use the following commands.

### Launch redis with docker

```
docker run --rm --name some-redis -p 6379:6379 -d redis
```
### Launch redis-commander with docker

```
docker run --rm --name redis-commander -p 8000:8081 -d rediscommander/redis-commander:latest
```
## cURL API

```
curl -X POST -H "Content-Type:application/json" -d '{"name":"test", "timestamp":"1579882155507"}' localhost:3030/add
```
