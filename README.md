<p align="center">
	<img src='./.github/logo.svg' width='200px'/>
</p>

<p align="center">
	<h1 align="center">mealpedant backup server</h1>
</p>

<p align="center">
	A simple backup service for mealpedant from a server, powered by <a href='https://www.staticpi.com' target='_blank' rel='noopener noreferrer'>staticpi.com</a>
</p>

<p align="center">
	Built in <a href='https://www.rust-lang.org/' target='_blank' rel='noopener noreferrer'>Rust</a>,
	on <a href='https://docker.com' target='_blank' rel='noopener noreferrer'> Docker</a>
</p>

### Required software

1) <a href='https://www.staticpi.com/' target='_blank' rel='noopener noreferrer'>staticPi</a> - simple, secure, messaging service
2) <a href='https://docker.com/' target='_blank' rel='noopener noreferrer'>Docker</a> - container runtime
3) <a href='https://gnupg.org/' target='_blank' rel='noopener noreferrer'>gpg</a> - enable encryptions of database backups


| directory | reason|
| --- | --- |
|```/srv/backup/mealpedant```			| Location of backups |
|```~/mealpeant_backup/```				| Location of node client|
|```~/mealpedant_backup/client/.env```	| enviromental variables, make sure in production mode|
|```~/mealpeant_backup/logs```			| Location of logs |


## Run step
1) ```./run.sh``` build, or re-build, docker container


# Readme

## Build for pi

```bash
# ubuntu [docker]
cross build --target arm-unknown-linux-gnueabihf --release

# alpine docker - armv7-unknown-linux-gnueabihf aka pi zero w
cross build --target arm-unknown-linux-musleabihf --release

# alpine docker - x86 musl, aka server
cross build --target x86_64-unknown-linux-musl
```
## Cargo watch

```sh
cargo watch -q -c -w src/ -x 'run'
```

## Tests

```sh
cargo test -- --test-threads=1 --nocapture


# Watch for test that start some_prefix
cargo watch -q -c -w src/ -x 'test some_prefix_ -- --test-threads=1 --nocapture'
```