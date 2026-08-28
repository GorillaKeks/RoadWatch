# 🚛 RoadWatch

**RoadWatch** is a desktop application for monitoring your TruckersMP Virtual Trucking Company (VTC) in real time.

It allows you to see which VTC members are currently online, which game they are playing, where they are located, and—when possible—their distance from your own player.

RoadWatch currently supports both **Euro Truck Simulator 2 (ETS2)** and **American Truck Simulator (ATS)**.

---

## ✨ Features

### 👥 VTC Member Monitoring

- Load members directly from the TruckersMP VTC API
- Display all VTC members in a clear player list
- Detect which members are currently online
- Show online and offline status

### 🌐 Live Player Tracking

RoadWatch collects live player data from supported map providers.

Current support includes:

- ETS2 live player tracking
- ATS live player tracking
- TruckersMP server detection
- Automatic server mapping

### 🎮 ETS2 & ATS Support

RoadWatch supports both major TruckersMP games:

- Euro Truck Simulator 2
- American Truck Simulator

Players are automatically associated with their current game when live data is available.

### 📍 Player Locations

RoadWatch can process player coordinates and match them with available city data.

Current location datasets include:

- ETS2 cities
- ATS cities

### 📏 Distance Calculation

When your own TruckersMP player is online and has a known position, RoadWatch can calculate the straight-line distance to other online VTC members.

Distance calculations are only performed when both players are in the same game.

### 🖥️ Desktop Application

RoadWatch is built as a modern desktop application using:

- **Tauri**
- **Rust**
- **React**
- **TypeScript**
- **Vite**

This provides a lightweight and fast alternative to traditional desktop frameworks.

### 🌍 Multi-Language Support

The application is designed with internationalization support.

Currently included:

- English
- German

The application interface is currently primarily developed in English.

---

## 🚀 Current Status

RoadWatch is currently in active development.

The current development version includes:

- TruckersMP VTC member loading
- Live ETS2 player detection
- Live ATS player detection
- TruckersMP server detection
- ETS2Map integration
- ATSMap integration
- Online/offline member matching
- Player game detection
- Player position processing
- City location datasets
- Distance calculation
- Settings management
- English and German language resources

---

## 🛠️ Development Setup

### Requirements

You will need:

- Node.js
- npm
- Rust
- Cargo
- Tauri development environment

### Install dependencies

```bash
npm install