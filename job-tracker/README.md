# Job Tracker

## A project by Datacord

> Fetches the Discord Jobs page and checks for new ones, removed ones and updated ones.

## Setup and run with Docker

```docker
docker build -t datacord-job-tracker:latest .
```

```docker
docker volume create datacord-job-tracker
```

```docker
docker run \
-e WEBHOOK_URL='REPLACE_ME' \
-e ROLE_ID='REPLACE_ME' \
-e DB_URL='./db/job.db' \
--mount type=volume,src=datacord-job-tracker,target=/job-tracker/db \
--name job-tracker \
datacord-job-tracker:latest
```
