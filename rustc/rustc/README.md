# 🦀 Simulación de Foraging con Feromonas en Rust

Implementación en Rust del algoritmo de búsqueda de comida con sistema de feromonas usando macroquad.

## 🚀 Ejecutar

```bash
cd rustc
cargo run --release
```

## 🎮 Controles

- **ESPACIO**: Pausar/Reanudar
- **P**: Toggle feromonas
- **V**: Toggle visualización
- **A**: Agregar agente
- **Click**: Agregar comida
- **ESC**: Salir

## 🎨 Colores

- 🟢 Verde: Nido y feromonas hacia nido
- 🔵 Azul: Agentes y feromonas hacia comida
- 🟠 Naranja: Agentes con poca energía
- 🔴 Rojo: Agentes con comida
- 🟡 Amarillo: Fuentes de comida

## ⚡ Características

- Sistema de feromonas con evaporación y difusión
- Agentes autónomos con energía y comportamiento emergente
- Visualización en tiempo real optimizada
- Rendimiento nativo (60+ FPS con cientos de agentes)

## 📦 Estructura

```
rustc/
├── Cargo.toml    # Dependencias
├── src/
│   └── main.rs   # Código completo
└── README.md     # Documentación
```

## 🔧 Parámetros

Modifica en `main.rs`:

```rust
CELL_SIZE: 10.0          // Tamaño de celda
EVAPORATION_RATE: 0.95   // Evaporación
DIFFUSION_RATE: 0.05     // Difusión
```

## 🆚 vs JavaScript

| Aspecto | JS | Rust |
|---------|---|------|
| Velocidad | ~30 FPS | ~60 FPS |
| Memoria | ~50 MB | ~10 MB |
| Plataforma | Web | Nativo |

## 🐛 Troubleshooting

```bash
# Actualizar Rust
rustup update

# Limpiar y recompilar
cargo clean
cargo build --release
```

**Linux**: Puede necesitar `sudo apt install libgl1-mesa-dev`

---

**¡Feromonas a velocidad nativa! 🐜🦀✨**
