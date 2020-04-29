# File with heading

A small tcp server for performing common utility functions.

## Features

- Random ID generation - ALPHA, ALPHANUM, NUM, PASSWORD
- Encoding and Decoding - BASE64
- Calculating Checksum - CRC16 (CCITT), CRC32
- Hashing - SHA1, SHA256, SHA512

## Build

```shell
cargo build --release
```

## Run

```shell
./target/release/util-server
```

Server binds on port **8080** by default. You can change the server port by adding PORT=8085 as environment variable.

```shell
PORT=8085 ./target/release/util-server
```

## Usage

Connect to server using **netcat** or **telnet** and enter commands as line text. Press enter to send request to server.

```shell
nc 127.0.0.1 8085
```

### Commands

Commands are in the format - COMMAND MODE DATA/LENGHT. Usable commmand are RND, HASH, ENCODE, DECODE, CHECKSUM. See full example below:

- RND NUM 30
- RND ALPHA 40
- RND ALPHANUM 40
- RND PASSWORD 20
- HASH SHA1 Hello World
- HASH SHA256 Hello World
- HASH SHA512 Hello World
- CHECKSUM CRC16 0123456789
- CHECKSUM CRC32 0123456789
- ENCODE BASE64 Hello World
- DECODE BASE64 Hello World

**Note:** COMMAND and MODE are case insensitive.

### Operational Commands

- /exit
- /history

## License

MIT
