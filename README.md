[![Build](https://img.shields.io/github/actions/workflow/status/thatzok/vilog/build.yml?label=build&logo=github)](https://github.com/thatzok/vilog/actions/workflows/build.yml)
[![Tests](https://img.shields.io/github/actions/workflow/status/thatzok/vilog/test.yml?label=tests&logo=github)](https://github.com/thatzok/vilog/actions/workflows/test.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![rustc](https://img.shields.io/badge/rustc-stable-blue.svg)](https://www.rust-lang.org/)

# ViLog

A lightweight logger/forwarder written in Rust that regularly retrieves status and diagnostic messages from a Viessmann Vitocal heat pump (and others based on the Viessmann OneBase platform), displays them on the console, and optionally writes them to InfluxDB. It runs on Windows and Linux.

The **Viessmann API** in the Viessmann Cloud **is not used**; the data is retrieved directly from the system via the CAN bus.

The CAN bus query is performed by an Open3E server connected to the system's CAN bus, which communicates with ViLog via an MQTT server.
For more information about Open3E, see [Open3E](https://github.com/open3e/open3e).

Sounds a bit complicated at first? But in the end, it's very simple and reliable. HOWEVER: **Use at your own ris**k. I am not responsible if your heat pump suddenly stops working in the dead of winter.

Optionally, the log data can be stored in an InfluxDB database for later analysis.

The design philosophy is based on the Unix principle: a program should focus on one task and perform it well, and even complex problems can be solved by combining small, specialized programs.

Since communication only takes place via an MQTT server and an InfluxDB server, ViLog can be run on any server/computer that can establish a connection to the servers.

## Sample Output on Console
First, all log entries still in the system's history are displayed (sometimes not in perfect chronological order, but the timestamps are correct).

If the InfluxDB option is enabled but a connection cannot be established, error messages are displayed (no news is good news).

* 2025-11-12 11:10:36 250A HPMU[125]: debug HeatPumpHeatingActive
* 2025-11-12 12:08:23 250A HPMU[129]: debug HeatPumpPostRun
* 2025-11-12 12:10:24 250A HPMU[123]: debug HeatPumpOff
* 2025-11-12 12:10:27 250A HPMU[134]: debug FourThreeWayValveIdlePosition
* 2025-11-12 13:07:04 250A HPMU[118]: debug FourThreeWayValveInternalBufferPosition
* 2025-11-12 13:07:11 250A HPMU[124]: debug HeatPumpPreRun
* 2025-11-12 13:09:12 250A HPMU[125]: debug HeatPumpHeatingActive
* 2025-11-12 13:52:17 250A HPMU[129]: debug HeatPumpPostRun
* 2025-11-12 13:54:18 250A HPMU[123]: debug HeatPumpOff
* 2025-11-12 13:54:21 250A HPMU[134]: debug FourThreeWayValveIdlePosition
* 2025-11-11 04:51:11 250A HPMU[120]: info NoiseReductionModeActive
* 2025-11-11 09:14:14 250A HPMU[120]: info NoiseReductionModeActive
* 2025-11-11 09:30:15 250A HPMU[120]: info NoiseReductionModeActive
* 2025-11-11 11:59:08 250A HPMU[120]: info NoiseReductionModeActive
* 2025-11-11 13:49:56 250A HPMU[120]: info NoiseReductionModeActive
* 2025-11-11 15:30:13 250A HPMU[120]: info NoiseReductionModeActive
* 2025-11-11 23:13:08 250A HPMU[120]: info NoiseReductionModeActive
* 2025-11-12 08:17:22 250A HPMU[120]: info NoiseReductionModeActive
* 2025-11-12 11:12:28 250A HPMU[120]: info NoiseReductionModeActive
* 2025-11-12 13:11:03 250A HPMU[120]: info NoiseReductionModeActive
* 2025-10-20 08:58:38 250A HPMU[100]: warning RestoreEepromToDefault
* 2025-10-20 09:37:11 250A HPMU[100]: warning RestoreEepromToDefault

And then new log entries are shown as soon as they appear.

* 2025-11-12 14:47:39 250A HPMU[118]: debug FourThreeWayValveInternalBufferPosition
* 2025-11-12 14:47:46 250A HPMU[124]: debug HeatPumpPreRun
* 2025-11-12 14:49:47 250A HPMU[125]: debug HeatPumpHeatingActive
* 2025-11-12 14:51:39 250A HPMU[120]: info NoiseReductionModeActive



## Installation
If you simply want to use the program, download the binary for Windows or Linux from the latest release on the release page and create a `vilog.toml` configuration file that suits your situation.

You can start the program manually; it will run continuously (waiting in an event loop) without using any resources and can be stopped with CTRL-C.

You can also run the program continuously as a system service, but you should know how to do that on your operating system.


## Build Release

You need: 
- Rust „stable“ Toolchain
- For Linux: `pkg-config` and `libssl-dev`

```bash
cargo build --release
```

## Configuration

The app reads its configuration from the `vilog.toml` file in the project or working directory. A different directory can be specified using the environment variable `VILOG_CONFIG`.

Example (adjust values accordingly):

```toml
# vilog.toml
# ViLog Configuration File with examples and defaults
# Note: A .toml file is like a .ini file, only better standardized.
#
# - An alternative path can be set using the environment variable VILOG_CONFIG.
#
# - All fields are optional; missing values will be replaced with appropriate defaults where applicable.

[mqtt]
# Client-ID
client_id = "vilogger"
# Broker-Host/IP
host = "127.0.0.1"
# Broker-Port
port = 1883
# authentification
username = "vilogger"
password = ""
# Keep-Alive of mqtt-connection (in seconds)
keep_alive_secs = 30

[topics]
# topics where open3e sends data IN JSON (depends on your open3e-config)
error = "open3e/680_266_ErrorDtcHistory"
warning = "open3e/680_264_WarningDtcHistory"
service = "open3e/680_262_ServiceDtcHistory"
info = "open3e/680_260_InfoDtcHistory"
status = "open3e/680_258_StatusDtcHistory"
# Name/ID of your Heat-Pump for the log/InfluxDB.
systemid = "250A"
# Name/ID of your ECU  for the log/InfluxDB.
ecuid = "HPMU"
# open3e command topic/channel (depends on your open3e-config)
command_topic = "open3e/cmnd"
# command-structure in json (here: read the logs and send data in json)
# "read-json" is important because we can only process json-payload
command_payload = "{\"mode\": \"read-json\", \"data\":[258,260,262,264,266]}"
# interval in seconds the command is sent and responses are received
# determines how quickly new log entries are processed.
# The interval shouldn't be too small, because a relatively large
# amount of data can be transferred with each request. 30 to 60 seconds
# should be fast enough to see new log entries in a timely manner.
command_interval_secs = 60

[influxdb]
# Enable/Disable writing to InfluxDB
enabled = true
# Base URL of InfluxDB server
url = "http://127.0.0.1:8086"
# Organization name
org = "vilog"
# Bucket to write to
bucket = "vilog"
# Auth token (use an InfluxDB API token with write permission on the bucket)
token = ""
# Measurement name to use for syslog-like entries
measurement = "syslog"
# HTTP request timeout in seconds
timeout_secs = 5

```

281 / 5.000
## Contribute

Issues and pull requests are welcome. Please adhere to the existing style (rustfmt, clippy without warnings) and try to cover changes with tests where appropriate.

## License

Apache License, Version 2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

Have fun!

