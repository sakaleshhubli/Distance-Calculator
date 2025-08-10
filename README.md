# Distance Calculator 🌍

A Rust-based command line interface (CLI) tool that calculates the shortest distance between cities using their international airport IATA codes and the Haversine formula.


- Uses the Haversine formula for accurate great-circle distance calculations
- Input cities using their 3-letter airport codes (e.g., DEL for Delhi, BLR for Bengaluru)
- Easy to use CLI with intuitive commands
- Works on Windows, macOS, and Linux

### Sample Output
```
Distance between Delhi (DEL) and Bengaluru (BLR): 1,697.23 km

# With verbose flag:
From: Indira Gandhi International Airport (DEL) - New Delhi, India
  Coordinates: 28.5562°N, 77.1000°E
To: Kempegowda International Airport (BLR) - Bengaluru, India  
  Coordinates: 13.1979°N, 77.7064°E
Great circle distance: 1,697.23 km
```

### How It Works

The Distance Calculator uses the **Haversine formula** to calculate the great-circle distance between two points on Earth's surface. This formula accounts for the Earth's curvature and provides accurate distances for aviation and geographical purposes.

#### Haversine Formula

```
a = sin²(Δφ/2) + cos φ1 ⋅ cos φ2 ⋅ sin²(Δλ/2)
c = 2 ⋅ atan2(√a, √(1−a))
d = R ⋅ c
```

Where:
- φ is latitude
- λ is longitude  
- R is Earth's radius (mean radius = 6,371 km)
- d is the distance between the two points


The tool supports airports worldwide using standard IATA 3-letter codes. Some popular examples:


### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Development Setup

```bash
# Clone your fork
git clone https://github.com/yourusername/Distance-Calculator.git
cd Distance-Calculator

# Run tests
cargo test

# Run with debug output
cargo run -- DEL BLR --verbose

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

---

⭐ Star this repository if you find it helpful!
