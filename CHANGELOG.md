# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.1.8'>v0.1.8</a>
### 2023-11-17

### Chores
+ .devcontainer updated, [cd7df505](https://github.com/mrjackwills/mealpedant_backup_server/commit/cd7df505ddd20e0a47cdfcec951d0f9e78616f37)
+ lints moved from main.rs to Cargo.toml, [108509c5](https://github.com/mrjackwills/mealpedant_backup_server/commit/108509c57089f6f1233f3348f25e0e7f3e896704)
+ Rust 1.73.0 linting, [e180b8f3](https://github.com/mrjackwills/mealpedant_backup_server/commit/e180b8f32163da8606f7d54e3edd678091cb5bc4)
+ dependencies updated, [f3742289](https://github.com/mrjackwills/mealpedant_backup_server/commit/f374228911999f3472fbea89036233a0289933cf)
+ Rust 1.74.0 linting, [93e813c3](https://github.com/mrjackwills/mealpedant_backup_server/commit/93e813c36214c6cd41ac830188d6a3c0288e626d)
+ GitHub workflow updated, [b45134a7](https://github.com/mrjackwills/mealpedant_backup_server/commit/b45134a70d4e402fe6459eec98b57c897cb0c463)

### Fixes
+ .gitattributes updated, [6c231d72](https://github.com/mrjackwills/mealpedant_backup_server/commit/6c231d72bdbc0548aa11babab40ddd7208aa574f)


# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.1.7'>v0.1.7</a>
### 2023-08-25

### Features
+ switch base-64 crate for data_encoding, [fda2b524](https://github.com/mrjackwills/mealpedant_backup_server/commit/fda2b524913894b04e91bbd77b9177d9129dc857)

# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.1.6'>v0.1.6</a>
### 2023-08-24

### Chores
+ dependencies updated, [041e6c56](https://github.com/mrjackwills/mealpedant_backup_server/commit/041e6c56794233f87cab66e3db43f6b3a277a3be)
+ rust 1.72.0 linting, [9fbb6215](https://github.com/mrjackwills/mealpedant_backup_server/commit/9fbb621585da29f5db44ab4aa047da71745e8933)
+ create_release 0.3.1, [abdc00d6](https://github.com/mrjackwills/mealpedant_backup_server/commit/abdc00d6918d0d1460731c0f71a928b91d88d88f)

### Refactors
+ time-tz and EnvTimeZone removed, [78997da1](https://github.com/mrjackwills/mealpedant_backup_server/commit/78997da156f2d1fdef348f48b229dc3c61419662)
+ `env/mod.rs` > `app_env.rs`, [180210c3](https://github.com/mrjackwills/mealpedant_backup_server/commit/180210c3189a3e116a78b8fccac6ce9182b944e8)

# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.1.5'>v0.1.5</a>
### 2023-07-28

### Chores
+ dependencies updated, [aaef9d6a](https://github.com/mrjackwills/mealpedant_backup_server/commit/aaef9d6a09a7d7707f254ef6dcd3eee74222c778)
+ create_release 0.3.0, [37a2ddc3](https://github.com/mrjackwills/mealpedant_backup_server/commit/37a2ddc3171e056c623506ea7dfc40f8f94bca02)

# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.1.4'>v0.1.4</a>
### 2023-06-07

### Chores
+ dependencies updated, [7c8d8f98](https://github.com/mrjackwills/mealpedant_backup_server/commit/7c8d8f98c8805e588eff11425ae6516f215c66d0), [01b00d99](https://github.com/mrjackwills/mealpedant_backup_server/commit/01b00d997cef435fda96405a4a05a32261ac1cfe), [ea13b5f4](https://github.com/mrjackwills/mealpedant_backup_server/commit/ea13b5f44f2377ac624adecd16bfaa697f14e81a), [4f28dffa](https://github.com/mrjackwills/mealpedant_backup_server/commit/4f28dffa1aca77b4bcc54c95574ed17156bcb1a7)
+ Dockerfile Alpine bump to 3.18, [1142c6b6](https://github.com/mrjackwills/mealpedant_backup_server/commit/1142c6b6e543e26e9b953995ca9b92637e08885f)

### Refactors
+ dead code removed, [ed825c69](https://github.com/mrjackwills/mealpedant_backup_server/commit/ed825c69cfa8ae3ce50ff3cac9ca654c2f77a621)

### Reverts
+ .devcontainer sparse protocol now default, [e2943fdf](https://github.com/mrjackwills/mealpedant_backup_server/commit/e2943fdf2f10a372c03fa92879b2f3d277c1236b)

# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.1.3'>v0.1.3</a>
### 2023-03-30

### Chores
+ dependencies updated, [4eab9a59](https://github.com/mrjackwills/mealpedant_backup_server/commit/4eab9a59cbe72f443d61a83b7ed2dfdfef2c9d09)

### Fixes
+ send_backup() check file_name doesn't contain 'PHOTO', [cfd29911](https://github.com/mrjackwills/mealpedant_backup_server/commit/cfd29911f7c78346dd446ef7db304a02e0473b5e)

# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.1.2'>v0.1.2</a>
### 2023-03-11

### Chores
+ Rust 1.68.0 linting, [ff111b8c](https://github.com/mrjackwills/mealpedant_backup_server/commit/ff111b8c0dfe426691afbf135fc3b6e5ef314043)
+ devcontainer sparse protocol index, [b6e5b7db](https://github.com/mrjackwills/mealpedant_backup_server/commit/b6e5b7dbb917aae5b8f5c6613bb2d88f69633f97)
+ dependencies updated, [2c6025c8](https://github.com/mrjackwills/mealpedant_backup_server/commit/2c6025c8c93f8dfafaac15879e791364b48038a7)

### Refactors
+ send_backup(), [b02cd06a](https://github.com/mrjackwills/mealpedant_backup_server/commit/b02cd06a5acf96c92092aeb4f989e2a9f51afa4f)

# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.1.1'>v0.1.1</a>
### 2023-02-11

### Chores
+ dev container updated, [95cd3ec0](https://github.com/mrjackwills/mealpedant_backup_server/commit/95cd3ec06a0df0499b87132420a3c411996824df)
+ dependencies updated, [7affdbce](https://github.com/mrjackwills/mealpedant_backup_server/commit/7affdbcef8173adf9e0b59c048f709744dc4523b)
+ create_release v0.2.7, [d02c0fba](https://github.com/mrjackwills/mealpedant_backup_server/commit/d02c0fbaa9ed287a8647d17dd434391d28dc066f)

### Fixes
+ backups now .age files, [5cabcbd7](https://github.com/mrjackwills/mealpedant_backup_server/commit/5cabcbd7a0fc899f705796dffe7c5bc8468f4b24)

### Refactors
+ get_auth_token single-liner, [41e82e99](https://github.com/mrjackwills/mealpedant_backup_server/commit/41e82e992a3fc94dfc30740b917ad31035055ce5)

# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.1.0'>v0.1.0</a>
### 2023-02-02

### Chores
+ update dependencies, [17bb5fc3](https://github.com/mrjackwills/mealpedant_backup_server/commit/17bb5fc3789b6121e1a0f877636df30eb6ace6f7)
+ create_release semver regex, [58071d7d](https://github.com/mrjackwills/mealpedant_backup_server/commit/58071d7db20faf7f6f463051cc2594e8431303fe)

### Features
+ scratch docker container, [7d2682e5](https://github.com/mrjackwills/mealpedant_backup_server/commit/7d2682e5e7a4c1206e9c2bcbc8fc6115408655e3)
+ openssl dependency removed, [20fec1d2](https://github.com/mrjackwills/mealpedant_backup_server/commit/20fec1d27ea13003a212b00548d327a3f641e63f)
+ check for typos in create_release, [64bc8e1d](https://github.com/mrjackwills/mealpedant_backup_server/commit/64bc8e1df16bd17aad4f00dbf20582e5e605a9dd)

### Refactors
+ tracing, [004cf6b1](https://github.com/mrjackwills/mealpedant_backup_server/commit/004cf6b1bfbcda8368ec22b7997753106af9d520)

# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.0.3'>v0.0.3</a>
### 2023-01-24

### Chores
+ update dependencies, [5262ea6f](https://github.com/mrjackwills/mealpedant_backup_server/commit/5262ea6ff11920386d1c6ebbda9ff3d51ff9abd9)

### Features
+ reqwest timeout & user agent, [0f4098b4](https://github.com/mrjackwills/mealpedant_backup_server/commit/0f4098b4371c273c772b5692b6606328eb73b997)

### Fixes
+ remove base64 depreciated methods, [c182e936](https://github.com/mrjackwills/mealpedant_backup_server/commit/c182e93665ea6d3cf6355a8e7f17512ab24c3448)

# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.0.2'>v0.0.2</a>
### 2023-01-07

### Chores
+ dependencies updated, [b203d2b9](https://github.com/mrjackwills/mealpedant_backup_server/commit/b203d2b99f498a9cf3ed3f2d9f01fef429962e47)
+ Rust 1.66 linting, [a95c7c7c](https://github.com/mrjackwills/mealpedant_backup_server/commit/a95c7c7c304ba463b10b47bd2c8152dc9ed2a2ed)

### Docs
+ typos, [9ff226ed](https://github.com/mrjackwills/mealpedant_backup_server/commit/9ff226ed1608752bc82615d74822e32cc4f1addf)

### Features
+ trace level into app_env, [96ed1875](https://github.com/mrjackwills/mealpedant_backup_server/commit/96ed1875ac4da2e011faeef7db0c42b043cdeef4)

### Fixes
+ run.sh linting, [96ed1875](https://github.com/mrjackwills/mealpedant_backup_server/commit/96ed1875ac4da2e011faeef7db0c42b043cdeef4)

# <a href='https://github.com/mrjackwills/mealpedant_backup_server/releases/tag/v0.0.1'>v0.0.1</a>
### 2022-11-24

### Features
+ init commit [513faa84](https://github.com/mrjackwills/mealpedant_backup_server/commit/513faa845690b721e58c72eea9ad7fe50e0e5301)
