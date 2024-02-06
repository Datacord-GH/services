# Desktop Build Manager

## A project by Datacord

> Fetches the Stable, PTB and Canary Discord versions and the hashes


## Setup and run with Docker

```docker
docker build -t datacord-desktop-build-tracker:latest .
```

```docker
docker volume create datacord-desktop-build-tracker
```

```docker
docker run \
-e DESKTOP_BUILD_WEBHOOK_URL='REPLACE_ME' \
-e CANARY_ROLE_ID='REPLACE_ME' \
-e PTB_ROLE_ID='REPLACE_ME' \
-e STABLE_ROLE_ID='REPLACE_ME' \
-e DB_URL='./db/desktop-build.db' \
--mount type=volume,src=datacord-desktop-build-tracker,target=/desktop-build-tracker/db \
--name desktop-build-tracker \
datacord-desktop-build-tracker:latest
```
