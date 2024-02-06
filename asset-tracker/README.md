# Asset Tracker

## A project by Datacord

> Tracks new assets from Discord

## Setup and run with Docker

```docker
docker build -t datacord-asset-tracker:latest .
```

```docker
docker volume create datacord-asset-tracker
```

```docker
docker run \
-e WEBHOOK_URL='REPLACE_ME' \
-e ROLE_ID='REPLACE_ME' \
-e DB_URL='sqlite://db/asset.db' \
-e DRY_RUN='false' \
--mount type=volume,src=datacord-asset-tracker,target=/asset-tracker/db \
--name asset-tracker \
datacord-asset-tracker:latest
```
