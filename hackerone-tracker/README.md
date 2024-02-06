# HackerOne

## A project by Datacord

> Tracks the reputation from the Discord HackerOne

## Setup and run with Docker

```docker
docker build -t datacord-hackerone-tracker:latest .
```

```docker
docker volume create datacord-hackerone-tracker
```

```docker
docker run \
-e HACKERONE_WEBHOOK_URL='REPLACE_ME' \
-e ROLE_ID='REPLACE_ME' \
-e DATABASE_URL='sqlite://db/hackerone.db' \
--mount type=volume,src=datacord-hackerone-tracker,target=/hackerone-tracker/db \
--name hackerone-tracker \
datacord-hackerone-tracker:latest
```
