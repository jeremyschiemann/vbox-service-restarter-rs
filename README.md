# vbox-service-restarter-rs
Periodically restart the services of a vbox which sometimes stops. Rust version.

Specifically, this will send a request to strart the `STREAMER_SERVICE` and `UPNP_SERVER_SERVICE`

# Usage
Set these envvars:
- `VBOX_URL` (url or ip of your VBOX, e.g. `192.168.0.2`)
- `VBOX_USERNAME`
- `VBOX_PASSWORD`
- `VBOX_SLEEPTIME` (integer in seconds, optional, default: `600`)

=> start container

Python version: https://github.com/jeremyschiemann/vbox-service-restarter
