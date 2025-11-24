# 🐉 Proyecto 1: Knight Maze
 
> Explora un laberinto, esquiva a los goblins, y rescata a la princesa antes de que sea demasiado tarde.

![Knight's Quest Banner](assets/start.png)

## 🎮 Jugabilidad

- 🧑‍🚀 **Control del caballero**:  
  - `↑` / `↓` — Avanzar / Retroceder  
  - `←` / `→` — Rotar vista (mirar a los lados)  
- 👹 **Enemigos (Goblins)**: Patrullan el laberinto y persiguen al jugador si lo ven.  
- 👑 **Objetivo**: Encontrar y rescatar a la princesa escondida en el laberinto.  
- 🎵 **Audio**: Ambiente inmersivo con efectos de goblins, victoria y sonido de (usando `rodio`).

---

## 📦 Instalación y Ejecución
```bash
# Instalar rodio
sudo apt update
sudo apt install -y libasound2-dev pkg-config


# Clonar y ejecutar
git clone https://github.com/Isabella334/Graficas_Proyecto1.git
cd Graficas_Proyecto1
cargo run --release
```