# Store Tracker

## A project by Datacord

> Fetches the Discord store and checks for new products, price updaets and name changes

## Setup and run with Docker

```docker
docker build -t datacord-store-tracker:latest .
```

```docker
docker volume create datacord-store-tracker
```

```docker
docker run \
-e WEBHOOK_URL='REPLACE_ME' \
-e ROLE_ID='REPLACE_ME' \
-e DB_URL='./db/store.db' \
--mount type=volume,src=datacord-store-tracker,target=/store-tracker/db \
--name store-tracker \
datacord-store-tracker:latest
```
