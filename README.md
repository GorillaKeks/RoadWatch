# 🚛 RoadWatch

**RoadWatch** is a modern desktop application for monitoring Virtual Trucking Companies (VTCs) and their members in real time.

The application combines live player data with information from the TruckersMP API to provide an overview of VTC members who are currently online.

RoadWatch is designed to be lightweight, fast, and easy to use.

> 🚧 **RoadWatch is currently in active development. Features and functionality may change as development progresses.**

---

## ✨ Features

### 👥 VTC Member Monitoring

RoadWatch can load and process members directly from the TruckersMP VTC API.

Current functionality includes:

* Loading VTC members
* Displaying VTC member information
* Matching VTC members with live player data
* Detecting currently online members
* Displaying online and offline status

---

### 🌐 Live Player Tracking

RoadWatch retrieves live player information from supported tracking services.

Current functionality includes:

* Live player detection
* Player position processing
* Online player matching
* TruckersMP server detection
* Server mapping

---

### 🎮 Game Detection

When live data is available, RoadWatch can associate players with their current game.

The project supports TruckersMP environments including:

* **Euro Truck Simulator 2**
* **American Truck Simulator**

Game-specific functionality may vary depending on the available live data source.

---

### 📍 Player Locations

RoadWatch processes player coordinates and prepares them for location-based features.

Current and planned functionality includes:

* Player position information
* City and location matching
* More detailed player location information
* Server information

Location accuracy depends on the data provided by external services.

---

### 📏 Distance System

RoadWatch includes distance-based player monitoring.

When compatible position data is available, the application can calculate distances between players.

Distance calculations are only performed when both players are in the same game.

The distance system is being expanded with additional features such as sorting and filtering.

---

### 🔄 Live Refresh

RoadWatch includes a foundation for refreshing live player data.

The refresh system is being expanded to provide a more complete real-time experience.

Planned improvements include:

* Improved automatic refresh
* Live status indicators
* Last update timestamps
* Loading indicators
* Improved error handling

---

### 🖥️ Modern Desktop Application

RoadWatch is built as a modern desktop application using:

* **Tauri**
* **Rust**
* **React**
* **TypeScript**
* **Vite**

This technology stack provides a lightweight and fast alternative to traditional desktop frameworks.

---

## 🚀 Current Development Status

RoadWatch is currently in active development.

### Currently Implemented

* TruckersMP VTC member loading
* VTC member processing
* Live player data integration
* ETS2Map integration
* Online/offline member matching
* Player game detection
* Player position processing
* Location data processing
* Distance calculation
* TruckersMP server detection
* Server mapping
* Settings management
* Modern Tauri desktop architecture

---

## 🗺️ Roadmap

The following features are planned or currently being developed for RoadWatch.

### 🔄 Live Refresh

* [x] Manual refresh functionality
* [x] Automatic player data refresh foundation
* [x] Player position updates
* [x] Distance calculation updates
* [ ] Complete automatic refresh system
* [ ] Add live status indicators
* [ ] Add last update timestamps
* [ ] Improve loading indicators
* [ ] Improve error handling

### 📏 Distance System

* [x] Distance calculation foundation
* [x] Player distance processing
* [ ] Distance-based player sorting
* [ ] Distance filtering
* [ ] Automatic distance updates
* [ ] Improved distance accuracy

### 👤 Player Details

* [ ] Dedicated player detail view
* [ ] More detailed location information
* [ ] Game information
* [ ] Server information
* [ ] Additional TruckersMP player details

### 🌐 Live Player Data

* [x] TruckersMP VTC member loading
* [x] Live player data integration
* [x] Online/offline member matching
* [x] Player game detection
* [x] Player position processing
* [x] TruckersMP server detection
* [x] Server mapping

### 🔮 Future Ideas

* [ ] Expanded player monitoring
* [ ] Additional filtering options
* [ ] Improved VTC statistics
* [ ] Savegame-related tools

---

## 🏗️ Architecture

RoadWatch uses a modern frontend and backend architecture.

```text
RoadWatch
│
├── Frontend
│   ├── React
│   ├── TypeScript
│   ├── Vite
│   ├── Components
│   └── Services
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

## 🌐 Data Sources

RoadWatch uses external services to retrieve player and VTC information.

### TruckersMP API

Used for:

* VTC information
* VTC member data
* Player-related information

### ETS2Map

Used for:

* Live player data
* Player positions
* Online player detection
* Server information

RoadWatch does not own or control these external services.

---

## 🛠️ Development Setup

### Requirements

You will need:

* Node.js
* npm
* Rust
* Cargo
* Tauri development environment

### Install Dependencies

```bash
npm install
```

### Start Development Mode

```bash
npm run tauri dev
```

### Build the Application

```bash
npm run tauri build
```

---

## ⚠️ Disclaimer

RoadWatch is an independent community project.

It is not affiliated with, endorsed by, or officially connected to:

* TruckersMP
* SCS Software
* ETS2Map

All trademarks and game names belong to their respective owners.

---

## 🚧 Development

RoadWatch is currently under active development.

New features, improvements, and changes are continuously being added.

The project focuses on creating a fast, modern, and user-friendly desktop application for monitoring TruckersMP VTC activity.

---

## 🚛 RoadWatch

**Track your VTC. See who's on the road.**
