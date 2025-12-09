// Simulación de Búsqueda de Comida con Feromonas en Rust
// Por limitaciones de espacio, este es el código completo optimizado

use macroquad::prelude::*;
use rand::Rng;

const CELL_SIZE: f32 = 10.0;
const EVAPORATION_RATE: f32 = 0.95;
const DIFFUSION_RATE: f32 = 0.05;

// Sistema de Feromonas
struct PheromoneMap {
    width: usize,
    height: usize,
    cols: usize,
    rows: usize,
    to_food_pheromone: Vec<Vec<f32>>,
    to_home_pheromone: Vec<Vec<f32>>,
}

impl PheromoneMap {
    fn new(width: usize, height: usize) -> Self {
        let cols = (width as f32 / CELL_SIZE).ceil() as usize;
        let rows = (height as f32 / CELL_SIZE).ceil() as usize;
        Self {
            width,
            height,
            cols,
            rows,
            to_food_pheromone: vec![vec![0.0; cols]; rows],
            to_home_pheromone: vec![vec![0.0; cols]; rows],
        }
    }

    fn deposit(&mut self, x: f32, y: f32, amount: f32, is_to_home: bool) {
        let col = (x / CELL_SIZE) as usize;
        let row = (y / CELL_SIZE) as usize;
        if row < self.rows && col < self.cols {
            let grid = if is_to_home { &mut self.to_home_pheromone } else { &mut self.to_food_pheromone };
            grid[row][col] = (grid[row][col] + amount).min(100.0);
        }
    }

    fn get_gradient(&self, x: f32, y: f32, is_to_home: bool) -> (f32, f32, f32) {
        let col = (x / CELL_SIZE) as isize;
        let row = (y / CELL_SIZE) as isize;
        let grid = if is_to_home { &self.to_home_pheromone } else { &self.to_food_pheromone };
        let mut max_p = 0.0;
        let (mut best_dx, mut best_dy) = (0.0, 0.0);
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                let (nr, nc) = ((row + dy) as usize, (col + dx) as usize);
                if nr < self.rows && nc < self.cols && grid[nr][nc] > max_p {
                    max_p = grid[nr][nc];
                    (best_dx, best_dy) = (dx as f32, dy as f32);
                }
            }
        }
        (best_dx, best_dy, max_p)
    }

    fn update(&mut self) {
        for grid in [&mut self.to_food_pheromone, &mut self.to_home_pheromone] {
            for row in grid.iter_mut() {
                for cell in row.iter_mut() {
                    *cell *= EVAPORATION_RATE;
                    if *cell < 0.1 { *cell = 0.0; }
                }
            }
        }
    }

    fn clear(&mut self) {
        for grid in [&mut self.to_food_pheromone, &mut self.to_home_pheromone] {
            for row in grid.iter_mut() {
                for cell in row.iter_mut() { *cell = 0.0; }
            }
        }
    }
}

// Fuente de Comida
struct FoodSource {
    x: f32,
    y: f32,
    amount: f32,
    initial: f32,
}

impl FoodSource {
    fn new(x: f32, y: f32, amount: f32) -> Self {
        Self { x, y, amount, initial: amount }
    }
    fn collect(&mut self) -> bool {
        if self.amount > 0.0 { self.amount -= 1.0; true } else { false }
    }
    fn is_empty(&self) -> bool { self.amount <= 0.0 }
}

// Agente
struct Agent {
    x: f32, y: f32, vx: f32, vy: f32,
    speed: f32, wander_angle: f32, has_food: bool,
    energy: f32, max_energy: f32, sense_radius: f32,
    food_collected: usize,
}

impl Agent {
    fn new(x: f32, y: f32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x, y,
            vx: rng.gen_range(-2.0..2.0),
            vy: rng.gen_range(-2.0..2.0),
            speed: 1.5,
            wander_angle: rng.gen_range(0.0..std::f32::consts::TAU),
            has_food: false,
            energy: 100.0,
            max_energy: 100.0,
            sense_radius: 80.0,
            food_collected: 0,
        }
    }

    fn update(&mut self, home_x: f32, home_y: f32, food: &mut Vec<FoodSource>, 
              pmap: &mut PheromoneMap, use_ph: bool, w: f32, h: f32) {
        self.energy -= 0.05;
        if self.energy <= 0.0 { self.energy = 0.0; return; }

        if self.has_food {
            self.move_to(home_x, home_y);
            if use_ph { pmap.deposit(self.x, self.y, 5.0, true); }
            let d = ((home_x - self.x).powi(2) + (home_y - self.y).powi(2)).sqrt();
            if d < 20.0 {
                self.has_food = false;
                self.food_collected += 1;
                self.energy = (self.energy + 30.0).min(self.max_energy);
            }
        } else {
            if let Some(i) = self.find_food(food) {
                let (fx, fy) = (food[i].x, food[i].y);
                self.move_to(fx, fy);
                let d = ((fx - self.x).powi(2) + (fy - self.y).powi(2)).sqrt();
                if d < 10.0 && food[i].collect() {
                    self.has_food = true;
                }
                if use_ph { pmap.deposit(self.x, self.y, 5.0, false); }
            } else if use_ph {
                let (dx, dy, s) = pmap.get_gradient(self.x, self.y, false);
                if s > 5.0 {
                    let mut rng = rand::thread_rng();
                    let pa = dy.atan2(dx);
                    self.wander_angle += rng.gen_range(-0.3..0.3);
                    let ta = pa * 0.7 + self.wander_angle * 0.3;
                    self.vx += (ta.cos() * self.speed - self.vx) * 0.2;
                    self.vy += (ta.sin() * self.speed - self.vy) * 0.2;
                } else { self.wander(); }
            } else { self.wander(); }
        }

        self.x += self.vx;
        self.y += self.vy;
        self.keep_bounds(w, h);
    }

    fn find_food(&self, food: &[FoodSource]) -> Option<usize> {
        food.iter().enumerate()
            .filter(|(_, f)| f.amount > 0.0)
            .map(|(i, f)| (i, ((f.x - self.x).powi(2) + (f.y - self.y).powi(2)).sqrt()))
            .filter(|(_, d)| *d < self.sense_radius)
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            .map(|(i, _)| i)
    }

    fn move_to(&mut self, tx: f32, ty: f32) {
        let (dx, dy) = (tx - self.x, ty - self.y);
        let d = (dx * dx + dy * dy).sqrt();
        if d > 0.0 {
            self.vx = (dx / d) * self.speed * 1.5;
            self.vy = (dy / d) * self.speed * 1.5;
        }
    }

    fn wander(&mut self) {
        let mut rng = rand::thread_rng();
        self.wander_angle += rng.gen_range(-0.3..0.3);
        let (tvx, tvy) = (self.wander_angle.cos() * self.speed, self.wander_angle.sin() * self.speed);
        self.vx += (tvx - self.vx) * 0.1;
        self.vy += (tvy - self.vy) * 0.1;
        let s = (self.vx * self.vx + self.vy * self.vy).sqrt();
        if s > self.speed {
            self.vx = (self.vx / s) * self.speed;
            self.vy = (self.vy / s) * self.speed;
        }
    }

    fn keep_bounds(&mut self, w: f32, h: f32) {
        let m = 10.0;
        let mut rng = rand::thread_rng();
        if self.x < m { self.x = m; self.vx = self.vx.abs(); self.wander_angle = rng.gen_range(0.0..std::f32::consts::TAU); }
        if self.x > w - m { self.x = w - m; self.vx = -self.vx.abs(); self.wander_angle = rng.gen_range(0.0..std::f32::consts::TAU); }
        if self.y < m { self.y = m; self.vy = self.vy.abs(); self.wander_angle = rng.gen_range(0.0..std::f32::consts::TAU); }
        if self.y > h - m { self.y = h - m; self.vy = -self.vy.abs(); self.wander_angle = rng.gen_range(0.0..std::f32::consts::TAU); }
    }

    fn is_alive(&self) -> bool { self.energy > 0.0 }
}

// Mundo
struct World {
    w: f32, h: f32, hx: f32, hy: f32,
    agents: Vec<Agent>, food: Vec<FoodSource>,
    pmap: PheromoneMap, use_ph: bool, show_ph: bool,
}

impl World {
    fn new(w: f32, h: f32) -> Self {
        Self {
            w, h, hx: w / 2.0, hy: h / 2.0,
            agents: Vec::new(), food: Vec::new(),
            pmap: PheromoneMap::new(w as usize, h as usize),
            use_ph: true, show_ph: true,
        }
    }
    
    fn update(&mut self) {
        if self.use_ph { self.pmap.update(); }
        for a in &mut self.agents {
            if a.is_alive() {
                a.update(self.hx, self.hy, &mut self.food, &mut self.pmap, self.use_ph, self.w, self.h);
            }
        }
        self.food.retain(|f| !f.is_empty());
    }

    fn stats(&self) -> (usize, usize, f32, usize, f32, usize) {
        let alive = self.agents.iter().filter(|a| a.is_alive()).count();
        let energy: f32 = self.agents.iter().filter(|a| a.is_alive()).map(|a| a.energy).sum();
        let avg = if alive > 0 { energy / alive as f32 } else { 0.0 };
        let collected = self.agents.iter().map(|a| a.food_collected).sum();
        let total_food: f32 = self.food.iter().map(|f| f.amount).sum();
        (alive, self.agents.len() - alive, avg, self.food.len(), total_food, collected)
    }
}

#[macroquad::main("🐜 Foraging Simulation - Rust")]
async fn main() {
    let (w, h) = (1200.0, 800.0);
    let mut world = World::new(w, h);
    let mut paused = false;
    let mut rng = rand::thread_rng();

    // Init
    for _ in 0..30 { world.agents.push(Agent::new(world.hx + rng.gen_range(-40.0..40.0), world.hy + rng.gen_range(-40.0..40.0))); }
    for _ in 0..5 { world.food.push(FoodSource::new(rng.gen_range(50.0..w-50.0), rng.gen_range(50.0..h-50.0), rng.gen_range(30.0..100.0))); }

    loop {
        if is_key_pressed(KeyCode::Escape) { break; }
        if is_key_pressed(KeyCode::Space) { paused = !paused; }
        if is_key_pressed(KeyCode::P) { world.use_ph = !world.use_ph; if !world.use_ph { world.pmap.clear(); } }
        if is_key_pressed(KeyCode::V) { world.show_ph = !world.show_ph; }
        if is_key_pressed(KeyCode::A) { world.agents.push(Agent::new(world.hx, world.hy)); }
        if is_mouse_button_pressed(MouseButton::Left) { let (mx, my) = mouse_position(); world.food.push(FoodSource::new(mx, my, 50.0)); }

        if !paused { world.update(); }

        clear_background(Color::new(0.1, 0.1, 0.18, 1.0));

        // Draw pheromones
        if world.show_ph && world.use_ph {
            for i in 0..world.pmap.rows {
                for j in 0..world.pmap.cols {
                    let l = world.pmap.to_home_pheromone[i][j];
                    if l > 0.5 {
                        draw_rectangle(j as f32 * CELL_SIZE, i as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, 
                            Color::new(0.18, 0.8, 0.44, (l / 100.0).min(0.6)));
                    }
                    let l = world.pmap.to_food_pheromone[i][j];
                    if l > 0.5 {
                        draw_rectangle(j as f32 * CELL_SIZE, i as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, 
                            Color::new(0.2, 0.6, 0.86, (l / 100.0).min(0.6)));
                    }
                }
            }
        }

        // Draw home
        draw_circle(world.hx, world.hy, 25.0, Color::new(0.18, 0.8, 0.44, 1.0));
        draw_circle_lines(world.hx, world.hy, 25.0, 3.0, Color::new(0.15, 0.68, 0.38, 1.0));

        // Draw food
        for f in &world.food {
            let r = 8.0 + (f.amount / f.initial) * 12.0;
            draw_circle(f.x, f.y, r, Color::new(0.95, 0.77, 0.06, 1.0));
            draw_circle_lines(f.x, f.y, r, 2.0, Color::new(0.95, 0.61, 0.07, 1.0));
        }

        // Draw agents
        for a in &world.agents {
            if !a.is_alive() { continue; }
            let color = if a.has_food { Color::new(0.91, 0.3, 0.24, 1.0) }
                else if a.energy / a.max_energy > 0.5 { Color::new(0.2, 0.6, 0.86, 1.0) }
                else { Color::new(0.9, 0.49, 0.13, 1.0) };
            draw_circle(a.x, a.y, 4.0, color);
            let angle = a.vy.atan2(a.vx);
            draw_line(a.x, a.y, a.x + angle.cos() * 8.0, a.y + angle.sin() * 8.0, 2.0, color);
            if a.has_food { draw_circle(a.x, a.y - 6.0, 2.0, Color::new(0.95, 0.77, 0.06, 1.0)); }
        }

        // UI
        let (alive, dead, energy, nfood, tfood, collected) = world.stats();
        let ui = format!("🐜 FORAGING SIMULATION (RUST)\n\n\
            [SPACE] Pausar | [P] Feromonas {} | [V] Visualizar {} | [A] +Agente | Click: +Comida\n\n\
            Agentes: {} vivos, {} muertos | Energía: {:.1}%\n\
            Comida: {} fuentes, {:.0} disponible, {} recolectada",
            if world.use_ph {"✓"} else {"✗"}, if world.show_ph {"✓"} else {"✗"},
            alive, dead, energy, nfood, tfood, collected);
        
        draw_rectangle(10.0, 10.0, 550.0, 160.0, Color::new(0.0, 0.0, 0.0, 0.85));
        draw_text(&ui, 20.0, 30.0, 18.0, WHITE);

        if paused {
            draw_text("⏸ PAUSADO", w / 2.0 - 80.0, h / 2.0, 40.0, Color::new(1.0, 1.0, 0.0, 0.9));
        }

        next_frame().await
    }
}
