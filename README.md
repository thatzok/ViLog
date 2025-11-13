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
When the program starts, all messages still in the history of the heat pump will be displayed and logged.(sometimes not in perfect chronological order, but the timestamps are correct, therefore, the data reported to InfluxDB is always correct, and there is no duplicate data even if the program is started several times in succession.).

* 2025-11-02T06:58:38+00:00 (2025-11-02 08:58:38) 250A HPMU[100]: Warning warning A.100 RestoreEepromToDefault
* 2025-11-02T07:37:11+00:00 (2025-11-02 09:37:11) 250A HPMU[100]: Warning warning A.100 RestoreEepromToDefault
* 2025-11-12T18:55:59+00:00 (2025-11-12 19:55:59) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-12T21:25:37+00:00 (2025-11-12 22:25:37) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-12T23:43:17+00:00 (2025-11-13 00:43:17) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-13T01:45:38+00:00 (2025-11-13 02:45:38) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-13T03:56:33+00:00 (2025-11-13 04:56:33) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-13T06:10:39+00:00 (2025-11-13 07:10:39) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-13T06:13:37+00:00 (2025-11-13 07:13:37) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-13T06:29:38+00:00 (2025-11-13 07:29:38) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-13T08:15:19+00:00 (2025-11-13 09:15:19) 250A HPMU[124]: State debug S.124 HeatPumpPreRun
* 2025-11-13T08:17:20+00:00 (2025-11-13 09:17:20) 250A HPMU[125]: State debug S.125 HeatPumpHeatingActive
* 2025-11-13T08:19:13+00:00 (2025-11-13 09:19:13) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-13T09:11:50+00:00 (2025-11-13 10:11:50) 250A HPMU[129]: State debug S.129 HeatPumpPostRun
* 2025-11-13T09:13:51+00:00 (2025-11-13 10:13:51) 250A HPMU[123]: State debug S.123 HeatPumpOff
* 2025-11-13T09:13:54+00:00 (2025-11-13 10:13:54) 250A HPMU[134]: State debug S.134 FourThreeWayValveIdlePosition
* 2025-11-13T10:08:14+00:00 (2025-11-13 11:08:14) 250A HPMU[118]: State debug S.118 FourThreeWayValveInternalBufferPosition
* 2025-11-13T10:08:21+00:00 (2025-11-13 11:08:21) 250A HPMU[124]: State debug S.124 HeatPumpPreRun
* 2025-11-13T10:10:22+00:00 (2025-11-13 11:10:22) 250A HPMU[125]: State debug S.125 HeatPumpHeatingActive
* 2025-11-13T10:12:15+00:00 (2025-11-13 11:12:15) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-13T10:18:01+00:00 (2025-11-13 11:18:01) 250A HPMU[134]: State debug S.134 FourThreeWayValveIdlePosition
* 2025-11-13T10:18:02+00:00 (2025-11-13 11:18:02) 250A HPMU[115]: State debug S.115 FourThreeWayValveDomesticHotWaterPosition

And then new log entries are shown as soon as they appear in chronological order.

* 2025-11-13T10:56:03+00:00 (2025-11-13 11:56:03) 250A HPMU[134]: State debug S.134 FourThreeWayValveIdlePosition
* 2025-11-13T10:56:04+00:00 (2025-11-13 11:56:04) 250A HPMU[118]: State debug S.118 FourThreeWayValveInternalBufferPosition
* 2025-11-13T10:56:25+00:00 (2025-11-13 11:56:25) 250A HPMU[120]: Info info I.120 NoiseReductionModeActive
* 2025-11-13T11:14:56+00:00 (2025-11-13 12:14:56) 250A HPMU[129]: State debug S.129 HeatPumpPostRun
* 2025-11-13T11:16:57+00:00 (2025-11-13 12:16:57) 250A HPMU[123]: State debug S.123 HeatPumpOff
* 2025-11-13T11:17:00+00:00 (2025-11-13 12:17:00) 250A HPMU[134]: State debug S.134 FourThreeWayValveIdlePosition


If the InfluxDB option is enabled but a connection cannot be established, also error messages are displayed (no news is good news).


## Installation
If you simply want to use the program, download the binary for Windows or Linux from the latest release on the [releases page](https://github.com/thatzok/ViLog/releases) and create a `vilog.toml` configuration file that suits your situation.

To set the correct MQTT parameters, you should already have Open3E installed and working, so that you can then set the corresponding values.

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

## Contribute

Issues and pull requests are welcome. Please adhere to the existing style (rustfmt, clippy without warnings) and try to cover changes with tests where appropriate.

## License

Apache License, Version 2.0. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

Have fun!

