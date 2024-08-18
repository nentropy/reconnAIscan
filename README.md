# 📡 ReconAIScanner
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

## 📜 Summary

ReconAIscanner is a tool designed for the reconnaissance of nearby Wi-Fi networks and the devices connected to them. It is primarily intended for research purposes, allowing users to monitor network activity, identify devices, and analyze wireless traffic. **This tool is for research purposes only and should be used responsibly.**

## 🚀 Features

### Implemented Features
- 🗺️ **Map Nearby Devices:** Visualize connected devices using [Netjson](https://github.com/netjson/netjsongraph.js).
- 🔢 **People Counter:** Count the number of people around you based on mobile device probe requests.
- 🛑 **Graceful Exit:** Stop scanning with `CTRL-C` and print all collected results.

### Planned Features
- ⚠️ **Autonomous Mode:** Agentic workflows that manage continuous scanning. The default is cli driven and 120seconds
- 📋 **NEN (Network Exit Notification):** Extensive and varied reporting delivered via WhatsApp and in JSON for further analysis
- 📲 **WhatsApp Notifications:** Get real-time alerts via WhatsApp.
- 🔄 **Continuous Monitoring:** Track specific targets continuously.
- ⚠️ **Suspicious Activity Monitoring:** Monitor for unusual wireless network activities.
- 🌍 **GeoLocation:** Track usage of less common SSIDs.
- 📧 **MAC Address Alerts:** Watch specific MAC addresses and send email alerts.
- 📍 **BLE Indoor Positioning:** Implement BLE-based indoor positioning.

## 🛠️ Build Instructions

### Prerequisites
Ensure you have the necessary dependencies installed. On Debian-based Linux distributions, you can install the required library with:
```bash
sudo apt-get install libpcap-dev
```

### Building the Project
To build ReconAIscanner, use the following command:
```bash
cargo build --release
```

## 🧑‍💻 Usage

### Scanning Nearby Devices

**Root access** is required for ReconAIscanner to set the wireless interface to `Monitor Mode`.

1. **Identify Network Interfaces:**
   List network interfaces using:
   ```bash
   ip link show
   ```
   or use `iwconfig` to identify your wireless adapter (e.g., wlan0).

2. **Start Scanning:**
   ```bash
   sudo target/release/ReconAIscanner -i your_wireless_adapter
   ```

3. **Generate Network Visualization:**
   To generate `networks.json` for visualization:
   ```bash
   sudo target/release/ReconAIscanner -i your_wireless_adapter --netjson
   ```

4. **Start Webserver for Visualization:**
   ```bash
   target/release/ReconAIscanner --graph
   ```

### Counting People Around You

Use the following command to generate `people.json` by watching Probe Requests:
```bash
sudo target/release/ReconAIscanner -i your_wireless_adapter --people
```

### ⚠️ Notes
- **Scan Duration:** The default scan time is 120s. If the program stops working after a short period with the error message `libpcap error: The interface went down`, it might be caused by another process (e.g., `network-manager`). Consider stopping the conflicting process:
   ```bash
   sudo service network-manager stop
   ```

## 🔌 WiFi Adapter Requirements

#### Your WiFi adapter must support monitor mode. Some recommended adapters include:
- Alfa AWUS036NHA (Author tested)
- Alfa AWUS036NEH
- TP-Link TL-WN722N (ONLY Version 1)

## ⚖️ Disclaimer

It is the end user's responsibility to comply with all applicable local, state, and federal laws. Developers assume no liability and are not responsible for any misuse or damage caused by this program.

## 👤 Author

- **New Author:** Nentropy [nentropy.x@gmail.com](mailto:nentropy.x@gmail.com)

## 🙏 Credits

This project is a modernization and extension of the original [Nearby](https://github.com/wisespace-io/nearby) project by [wisespace-io](https://github.com/wisespace-io).

--- 