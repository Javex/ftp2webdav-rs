# ftp2webdav-rs

A small tool that starts an FTP server and translates it into WebDAV requests.
Suitable for places where your client only speaks FTP (e.g. scanner) but you
want to upload files to a server that only speaks WebDAV (e.g. Nextcloud).

Set the `WEBDAV_SERVER` environment variable (see `.envrc.example`) and run the
program via `cargo run`. Then connect with an FTP client on port 2121. Use your
WebDAV username & password, it will be transparently forwarded.

## Docker

To start this project as a docker container use
