#########
# SETUP #
#########

FROM alpine:3.19 as SETUP

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
RUN wget https://github.com/mrjackwills/mealpedant_backup_server/releases/download/v0.1.10/mealpedant_backup_server_linux_x86_musl.tar.gz \
	&& tar xzvf mealpedant_backup_server_linux_x86_musl.tar.gz mealpedant_backup_server \
	&& rm mealpedant_backup_server_linux_x86_musl.tar.gz

##########
# RUNNER #
##########

FROM scratch

ARG DOCKER_APP_USER=app_user \
	DOCKER_APP_GROUP=app_group

COPY --from=SETUP /app/ /app
COPY --from=SETUP /etc/group /etc/passwd /etc/
COPY --from=SETUP /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

USER ${DOCKER_APP_USER}

ENTRYPOINT ["/app/mealpedant_backup_server"]
