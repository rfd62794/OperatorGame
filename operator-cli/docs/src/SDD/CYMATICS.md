# SDD-003: Cymatic Rendering & Harmonic Audio Synthesis

## 1. Overview
The "Cymatics" system translates the 9-Point Chromatic Spectrum into physical geometry (vertices) and resonant audio (oscillators). This ensures the game is entirely "asset-less," generating all visuals and sounds from pure math.

## 2. The Frequency-Vertex Map & Geometric Archetypes
Slime shapes are rendered as parametrically generated polygons, fractals, or rings.

| Culture | Frequency (Hz) | Geometric Form | Sonic Archetype |
|---------|---------------|----------------|-----------------|
| Ember (Red) | 256.0 | Tetrahedron (3/4 pts)| Deep Cello |
| Gale (Yellow) | 288.0 | Octahedron (8 pts) | Wind Chimes |
| Orange | 293.66 | Hexagon (6 pts) | Wooden Xylophone |
| Tide (Blue) | 320.0 | Icosahedron (20 pts)| Tibetan Bowl |
| Marsh (Green) | 384.0 | Torus (Ring) | Soft Rainfall |
| Crystal (Purple)| 426.0 | Dodecahedron (12 pts)| Crystal Singing Bowl |
| Void (White) | 432.0 | Perfect Sphere | Universal Resonance |
| Amber | 480.0 | Cube/Square (4 pts) | Low Throat Singing |
| Teal | 512.0 | Star Fractal | Aeolian Harp |
| Tundra (Frost) | 540.0 | Spiked Star | Cracking Ice |

## 3. Parametric Geometry (The "Wobble")
The radius $r$ of any vertex at angle $\theta$ is calculated using:
$$r(\theta, t) = R_{base} + \sin(f \cdot t + \text{offset}) \cdot A$$

Where:
*   $R_{base}$ is the base radius from `SlimeGenome.body_size`.
*   $f$ is the Culture frequency.
*   $A$ is the energy amplitude (higher energy = more violent vibration calculated by stats).

## 4. Audio Synthesis (The Solfeggio Loop)
Each Slime acts as a monophonic oscillator, combining WaveTypes (Sine, Triangle, Complex).
*   **The Major Third Rule:** When two Slimes are in proximity (Garden) or being Spliced (Incubator), their frequencies must resolve to a Harmonic Interval.
*   **Dissonance Detection:** If frequencies create a Minor Second, the UI adds "Digital Artifacting" (Visual Glitch) to represent planetary stress.
*   **The Master Resonance:** The Elder Slime emanates a perfect 432 Hz drone. Player actions ("Hooting and hollering") introduce 528 Hz transformation pulses, resulting in harmonic stabilization cycles.

## 5. Implementation Strategy
*   **Graphics:** Use `egui::Painter` to draw custom `Shape::Path` or `Mesh`. Vertices are updated continuously based on tick deltas.
*   **Audio:** Use the `cpal` or `rodio` crate for real-time synthesis (WASM compatible via `web-sys`).
