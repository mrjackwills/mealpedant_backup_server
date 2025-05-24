#########
# SETUP #
#########

FROM alpine:3 AS setup

ARG DOCKER_GUID=1000 \
	DOCKER_UID=1000 \
	DOCKER_APP_USER=app_user \
	DOCKER_APP_GROUP=app_group

ENV VIRT=".build_packages"

RUN addgroup -g ${DOCKER_GUID} -S ${DOCKER_APP_GROUP} \
	&& adduser -u ${DOCKER_UID} -S -G ${DOCKER_APP_GROUP} ${DOCKER_APP_USER} \
	&& apk --no-cache add --virtual ${VIRT} ca-certificates \
	&& apk del ${VIRT}

WORKDIR /app

# This gets automatically updated via create_release.sh
ARG MEALPEDANT_BACKUP_SERVER_VERSION=v0.2.2

RUN wget "https://github.com/mrjackwills/mealpedant_backup_server/releases/download/${MEALPEDANT_BACKUP_SERVER_VERSION}/mealpedant_backup_server_aarch64_musl.tar.gz" \
	&& tar xzvf mealpedant_backup_server_aarch64_musl.tar.gz mealpedant_backup_server \
	&& rm mealpedant_backup_server_aarch64_musl.tar.gz

##########
# RUNNER #
##########

FROM scratch

ARG DOCKER_APP_USER=app_user \
	DOCKER_APP_GROUP=app_group

COPY --from=setup /app/ /app
COPY --from=setup /etc/group /etc/passwd /etc/
COPY --from=setup /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

USER ${DOCKER_APP_USER}

ENTRYPOINT ["/app/mealpedant_backup_server"]
