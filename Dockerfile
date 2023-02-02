#########
# SETUP #
#########

FROM alpine:3.17 as SETUP

ARG DOCKER_GUID \
    DOCKER_UID \
    DOCKER_TIME_CONT \
    DOCKER_TIME_CITY \
    DOCKER_APP_USER=app_user \
    DOCKER_APP_GROUP=app_group

ENV VIRT=".build_packages"
ENV TZ=${DOCKER_TIME_CONT}/${DOCKER_TIME_CITY}


RUN addgroup -g ${DOCKER_GUID} -S ${DOCKER_APP_GROUP} \
    && adduser -u ${DOCKER_UID} -S -G ${DOCKER_APP_GROUP} ${DOCKER_APP_USER} \
    && apk --no-cache add --virtual ${VIRT} tzdata ca-certificates \
    && cp /usr/share/zoneinfo/${TZ} /etc/localtime \
    && update-ca-certificates \
    && echo ${TZ} > /etc/timezone \
    && apk del ${VIRT}

WORKDIR /app

# This gets automatically updated via create_release.sh
RUN wget https://github.com/mrjackwills/mealpedant_backup_server/releases/download/v0.0.3/mealpedant_backup_server_linux_x86_musl.tar.gz \
	&& tar xzvf mealpedant_backup_server_linux_x86_musl.tar.gz mealpedant_backup_server \
	&& rm mealpedant_backup_server_linux_x86_musl.tar.gz

##########
# RUNNER #
##########

FROM scratch AS RUNNER

ARG DOCKER_TIME_CONT \
    DOCKER_TIME_CITY \
    DOCKER_APP_USER=app_user \
    DOCKER_APP_GROUP=app_group

ENV TZ=${DOCKER_TIME_CONT}/${DOCKER_TIME_CITY}

COPY --from=SETUP /app/ /app
COPY --from=SETUP /etc/group /etc/passwd /etc/
COPY --from=SETUP /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

USER ${DOCKER_APP_USER}

ENTRYPOINT ["/app/mealpedant_backup_server"]
