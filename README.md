<p align="center">
	<img src='./.github/logo.svg' width='200px'/>
</p>

<p align="center">
	<h1 align="center">mealpedant_backup server</h1>
</p>

<p align="center">
	A simple backup service for mealpedant from a server, powered by <a href='https://www.staticpi.com' target='_blank' rel='noopener noreferrer'>staticpi.com</a>
</p>

<p align="center">
	Built in <a href='https://www.rust-lang.org/' target='_blank' rel='noopener noreferrer'>Rust</a>,
	for <a href='https://docker.com' target='_blank' rel='noopener noreferrer'> Docker</a>
	<br>
	see the accompanying <a href='https://www.github.com/mrjackwills/mealpedant_backup_pi' target='_blank' rel='noopener noreferrer'>pi client</a>
</p>

### Required software

1) <a href='https://www.staticpi.com/' target='_blank' rel='noopener noreferrer'>staticPi</a> simple, secure, messaging service
2) <a href='https://docker.com/' target='_blank' rel='noopener noreferrer'>Docker</a> - container runtime


| directory | reason|
| --- | --- |
|```/srv/backup/mealpedant```			| Location of backups |
|```~/mealpeant_backup/```				| Location of client|
|```~/mealpedant_backup/.env```			| environmental variables|
|```~/mealpeant_backup/logs```			| Location of logs |


## Run step
1) ```./run.sh``` build, or re-build, docker container

## Build

```bash
# alpine docker - x86 musl
cross build --target x86_64-unknown-linux-musl --release

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