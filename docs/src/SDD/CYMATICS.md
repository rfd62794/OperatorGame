# SDD-003: Cymatic Rendering & Harmonic Audio Synthesis

## 1. Overview
The "Cymatics" system translates the 9-Point Chromatic Spectrum into physical geometry (vertices) and resonant audio (oscillators). This ensures the game is entirely "asset-less," generating all visuals and sounds from pure math.

## 2. The Frequency-Vertex Map
Slime shapes are rendered as regular or warped polygons. The number of vertices ($V$) is derived from the base frequency.

| Culture | Frequency (f) | Shape Base (V) | Geometry Logic |
|---------|---------------|----------------|----------------|
| Ember (Red) | 256 Hz | 3 (Triangle) | Jagged edges, thermal flicker. |
| Gale (Yellow) | 288 Hz | 8 (Octagon) | High rotation, blurred vertices. |
| Tide (Blue) | 320 Hz | 20 (Icosahedron)| High-point approximation of a sphere. |
| Orange | 341 Hz | 6 (Hexagon) | Perfect tiling, structural rigidity. |
| Marsh (Green) | 384 Hz | Torus (Ring) | Overlapping sine-circles. |
| Crystal (Purple)| 426 Hz | 12 (Dodecahedron)| Sharp, refractive edge-logic. |
| Amber | 480 Hz | 4 (Cube/Square) | Heavy, low-vibration density. |
| Teal | 512 Hz | 5-pt Star | Multi-layered alpha-blending. |
| Frost (Tundra) | 540 Hz | Spiked Star | Static vertices with brittle shivering. |

## 3. Parametric Geometry (The "Wobble")
The radius $r$ of any vertex at angle $\theta$ is calculated using:
$$r(\theta, t) = R_{base} + \sin(f \cdot t + \text{offset}) \cdot A$$
Where:
*   $R_{base}$ is the base radius from SlimeGenome.body_size.
*   $f$ is the Culture frequency.
*   $A$ is the energy amplitude (higher energy = more violent vibration).

## 4. Audio Synthesis (The Solfeggio Loop)
Each Slime acts as a monophonic oscillator.
*   **Waveform:** Organic Sine (80%) + Warm Triangle (20%) to mimic "Cello" or "Tibetan Bowl."
*   **The Major Third Rule:** When two Slimes are in proximity (Garden) or being Spliced (Incubator), their frequencies must resolve to a Harmonic Interval.
*   **Dissonance Detection:** If frequencies create a Minor Second, the UI adds "Digital Artifacting" (Visual Glitch) to represent planetary stress.

## 5. Implementation Strategy
*   **Graphics:** Use `egui::Painter` to draw custom `Shape::Path` or `Mesh`.
*   **Audio:** Use the `cpal` or `rodio` crate for real-time synthesis (WASM compatible via `web-sys`).
