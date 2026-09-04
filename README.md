# 🚛 RoadWatch

**RoadWatch** is a modern desktop application for monitoring Virtual Trucking Companies (VTCs) and their members on TruckersMP.

It combines live player data with VTC information to provide a clear overview of members who are currently online and active on the road.

RoadWatch is designed to be lightweight, fast, and easy to use.

> 🚧 **Current version: v0.1.4**

---

# ✨ Features

## 👥 VTC & Live Player Monitoring

RoadWatch connects VTC member information with live player data.

Current functionality includes:

- Loading and processing VTC members via the TruckersMP API
- Displaying VTC member information
- Matching VTC members with live player data
- Detecting online and offline members
- Live player detection and position processing
- ETS2 and ATS detection
- TruckersMP server detection and mapping

---

## 📍 Locations & Distance System

RoadWatch processes player positions and location information to provide distance-based monitoring.

Features include:

- Player position processing
- Location and city matching
- Game and server information
- Distance calculation between compatible players
- Distance display and sorting
- Distance filtering

### Available Distance Filters

- All distances
- Under 50 km
- Under 100 km
- Under 250 km
- Under 500 km

> ℹ️ Distance calculations depend on available position data and are only performed when players are compatible for comparison.

---

## 🔎 Search & Filtering

RoadWatch provides tools to quickly find relevant players.

- Player search
- Online/offline filtering
- Distance filtering
- Distance-based sorting

---

## 🔄 Live Refresh

RoadWatch supports automatic and manual live data updates.

Features include:

- Manual refresh
- Automatic refresh
- Player position updates
- Distance updates
- Live status indicator
- Last update information
- Refresh loading indicator
- Refresh error handling

---

## ⚙️ Settings & Updates

RoadWatch includes:

- Integrated application settings
- Supported service configuration
- Automatic update checking
- Update downloading and installation
- Signed updater artifacts

The automatic update system has been successfully tested with:

**RoadWatch v0.1.2 → v0.1.4**

---

# 🖥️ Technology

RoadWatch is built using:

- **Tauri**
- **Rust**
- **React**
- **TypeScript**
- **Vite**

This architecture provides a lightweight, fast, and modern desktop application.

---

# 🚀 Current Development Status

## RoadWatch v0.1.3

### Implemented

- [x] TruckersMP VTC member integration
- [x] Live player data integration
- [x] ETS2Map integration
- [x] Online/offline member matching
- [x] ETS2 and ATS detection
- [x] Player position and location processing
- [x] Distance calculation, sorting, and filtering
- [x] Automatic distance updates
- [x] TruckersMP server detection
- [x] Player search and status filtering
- [x] Manual and automatic refresh
- [x] Live status and last update information
- [x] Loading and error handling
- [x] Settings management
- [x] Automatic application updates
- [x] Signed updater artifacts

---

# 🗺️ Roadmap

## 🚛 RoadWatch V1.0 – Complete UI Rebuild

The next major version will introduce a complete redesign of RoadWatch.

Instead of continuing to extend the current interface, **RoadWatch V1.0** will rebuild the user interface cleanly from the ground up.

### Planned Areas

- 🎨 Complete new application design
- 📊 Redesigned Dashboard
- 🗺️ Live Map with map material
- 👥 Redesigned Players section
- 🏢 Dedicated VTC section
- 🖥️ Dedicated Server section
- 📍 Advanced player location visualization
- 👤 Player detail view
- 🎯 Improved navigation and user experience
- 🧩 Cleaner component architecture
- 🎨 Separate styles for major application areas

> **The goal is a clean V1.0 architecture without legacy UI patches.**

---

## 🔮 Future Ideas

Possible future features include:

- Expanded player monitoring
- Advanced player details
- Improved VTC statistics
- Additional filtering options
- Map-based player visualization
- Savegame-related tools
- Additional TruckersMP data integration

---

# 🏗️ Architecture

```text
RoadWatch
│
├── Frontend
│   ├── React
│   ├── TypeScript
│   ├── Vite
│   ├── Components
│   ├── Services
│   ├── Types
│   └── Utilities
│
├── Backend
│   ├── Rust
│   ├── Tauri
│   └── Application Commands
│
└── Data Sources
    ├── TruckersMP API
    └── ETS2Map
```

---

# 🌐 Data Sources

RoadWatch uses external services to retrieve VTC and live player information.

## TruckersMP API

Used for:

- VTC information
- VTC member data
- Player-related information

## ETS2Map

Used for:

- Live player data
- Player positions
- Online player detection
- Server information

> ℹ️ RoadWatch does not own or control these external services. Data availability and accuracy depend on the respective services.

---

# 📦 Installation

Download the latest RoadWatch release from the project's **GitHub Releases** page and install the application using the provided Windows installer.

RoadWatch supports automatic updates for future releases.

---

# 🛠️ Development Setup

## Requirements

Before building RoadWatch, make sure the following tools are installed:

- Node.js
- npm
- Rust
- Cargo
- Tauri development environment

## Install Dependencies

```bash
npm install
```

## Start Development Mode

```bash
npm run tauri dev
```

## Build the Application

```bash
npm run tauri build
```

The Windows installer is generated in:

```text
src-tauri/target/release/bundle/nsis/
```

---

# 🔄 Updating

RoadWatch uses the **Tauri updater system**.

A release update requires:

- A new application version
- A signed installer
- A `.sig` signature file
- An updated `latest.json`
- A GitHub Release containing the required update artifacts

---

# ⚠️ Disclaimer

RoadWatch is an independent community project.

It is **not affiliated with, endorsed by, or officially connected to**:

- TruckersMP
- SCS Software
- ETS2Map

All trademarks, game names, logos, and related assets belong to their respective owners.

---

# 🚧 Development

RoadWatch v0.1.x establishes the technical foundation of the application.

The next major milestone is:

# 🚛 RoadWatch V1.0

## Complete UI Rebuild

**Track your VTC. See who's on the road. 🚛**
