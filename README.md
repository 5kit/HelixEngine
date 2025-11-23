# Helix Engine

![Helix Engine](https://img.shields.io/badge/Helix-Engine-blue) ![Python](https://img.shields.io/badge/Python-3.10%2B-blue) ![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)

**Helix Engine** is a modular, Python-accessible 3D engine system designed for building 3D applications and games. It consists of separate modules for rendering, physics, and potentially other systems, all designed for high-performance computation in Rust with Python bindings.
To Be Developed
---

## Modules

### 1. HelixRender
The 3D rendering module of Helix Engine.
- GPU-powered rendering with **full pixel-level control**.  
- Supports **basic meshes** (cube, sphere, OBJ files).  
- Uses **external GLSL shaders**.  
- Scene management: cameras, objects, and lights.  
- `render()` produces a **screen buffer** (RGB array), optionally updated.  
- `show()` displays the current scene buffer in a window.  

### 2. HelixDynamics (Planned)
- Physics engine for 3D simulations.  
- Modular design to allow selective importing of only the components you need.  
- Python API for seamless integration with HelixRender or other systems.

---

## v0.1 Focus – HelixRender
- Working **3D scene** with basic mesh rendering.  
- Single camera, optional basic light.  
- External GLSL shader support (per-pixel output).  
- Screen buffer accessible in Python (`Scene.screen`).  
- `render()` → updates screen and returns RGB array.  
- `show()` → displays the last rendered frame in a window.  

**Excluded Features for v0.1:**  
- Advanced materials (metallic, PBR, roughness).  
- Multiple lights, shadows, global illumination.  
- Compute shader optimizations, post-processing, serialization, or extended Python-Rust interop.

---

## Installation

```bash
git clone https://github.com/yourusername/helix-engine.git
cd helix-engine
```

## Requirements:

- Rust 1.70+
- Python 3.10+
- Dependencies will be listed in pyproject.toml or requirements.txt as development progresses.

## Basic Usage:
```bash
from Helix.render import Scene, Cube, Camera

# Initialize scene
scene = Scene()
cube = Cube(1, 1, 1)
scene.add_object(cube)

camera = Camera()
scene.add_camera(camera)
scene.set_active_camera(0)

# Open window
scene.show("shaders/basic.glsl")

# Main loop
running = True
while running:
    # Optional scene updates
    frame = scene.render("shaders/basic.glsl")
    scene.show()
```