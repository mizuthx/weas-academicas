# Guía Completa: Fundamentos de Rust y Proyectos Progresivos

## 📚 Tabla de Contenidos
1. [Fundamentos de Rust](#fundamentos)
2. [Proyectos Básicos](#proyectos-básicos)
3. [Proyectos Intermedios](#proyectos-intermedios)
4. [Proyectos Avanzados](#proyectos-avanzados)
5. [Proyecto Final: Bot de Discord](#proyecto-final-bot-de-discord)

---

## 🎯 Fundamentos

### 1. Sistema de Ownership (Propiedad)
**Concepto central de Rust: cada valor tiene UN dueño**
```rust
// REGLA 1: Un valor, un dueño
let s1 = String::from("hola");
let s2 = s1; // s1 ya NO es válido (moved)

// REGLA 2: El dueño se destruye, el valor se libera
{
    let x = String::from("temporal");
} // x se destruye aquí automáticamente

// REGLA 3: Puedes PRESTAR (borrow) sin transferir propiedad
let s3 = String::from("mundo");
let len = calcular_longitud(&s3); // prestamos &s3
println!("{}", s3); // s3 sigue siendo válido
```

### 2. Referencias y Borrowing
```rust
// Referencia inmutable (&T)
fn leer(texto: &String) {
    println!("{}", texto);
}

// Referencia mutable (&mut T) - SOLO UNA a la vez
fn modificar(texto: &mut String) {
    texto.push_str(" modificado");
}

fn main() {
    let mut s = String::from("Hola");
    leer(&s);           // ✅ Múltiples lecturas OK
    modificar(&mut s);  // ✅ Una escritura OK
}
```

**Regla de oro:** O múltiples lecturas (&) O una escritura (&mut), nunca ambas simultáneamente.

### 3. Tipos de Datos Fundamentales
```rust
// Escalares
let entero: i32 = 42;
let flotante: f64 = 3.14;
let booleano: bool = true;
let caracter: char = '🦀';

// Compuestos
let tupla: (i32, f64, char) = (500, 6.4, 'x');
let arreglo: [i32; 5] = [1, 2, 3, 4, 5];

// Strings
let literal: &str = "inmutable en memoria";
let dinamico: String = String::from("crece dinámicamente");
```

### 4. Control de Flujo
```rust
// if/else
if numero % 2 == 0 {
    println!("Par");
} else {
    println!("Impar");
}

// loop (infinito)
loop {
    break; // salir
}

// while
while contador < 10 {
    contador += 1;
}

// for (iteradores)
for elemento in &coleccion {
    println!("{}", elemento);
}
```

### 5. Pattern Matching
```rust
match valor {
    1 => println!("Uno"),
    2 | 3 => println!("Dos o tres"),
    4..=10 => println!("Entre 4 y 10"),
    _ => println!("Otro"), // catch-all obligatorio
}

// Option<T> - Manejo de valores opcionales
let numero: Option<i32> = Some(5);
match numero {
    Some(n) => println!("Valor: {}", n),
    None => println!("Sin valor"),
}
```

### 6. Structs y Enums
```rust
// Struct: agrupar datos relacionados
struct Usuario {
    nombre: String,
    email: String,
    activo: bool,
}

impl Usuario {
    fn nuevo(nombre: String, email: String) -> Self {
        Usuario {
            nombre,
            email,
            activo: true,
        }
    }
    
    fn saludar(&self) {
        println!("Hola, soy {}", self.nombre);
    }
}

// Enum: uno de varios posibles valores
enum Resultado {
    Exito(String),
    Error(u32),
}
```

### 7. Manejo de Errores
```rust
// Result<T, E> - Operaciones que pueden fallar
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("División por cero"))
    } else {
        Ok(a / b)
    }
}

// Uso con ? operator (propaga errores)
fn leer_archivo() -> Result<String, std::io::Error> {
    let contenido = std::fs::read_to_string("archivo.txt")?;
    Ok(contenido)
}
```

### 8. Traits (Interfaces)
```rust
trait Describible {
    fn describir(&self) -> String;
}

struct Perro {
    nombre: String,
}

impl Describible for Perro {
    fn describir(&self) -> String {
        format!("Soy un perro llamado {}", self.nombre)
    }
}
```

---

## 🔰 Proyectos Básicos

### Proyecto 1: Calculadora CLI
**Conceptos:** Variables, operadores, input/output, match
```rust
use std::io;

fn main() {
    println!("=== Calculadora Simple ===");
    println!("Ingresa primer número:");
    
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Error al leer");
    let num1: f64 = num1.trim().parse().expect("No es un número");
    
    println!("Ingresa operación (+, -, *, /):");
    let mut operacion = String::new();
    io::stdin().read_line(&mut operacion).expect("Error al leer");
    let operacion = operacion.trim();
    
    println!("Ingresa segundo número:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Error al leer");
    let num2: f64 = num2.trim().parse().expect("No es un número");
    
    let resultado = match operacion {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error: División por cero");
                return;
            }
        }
        _ => {
            println!("Operación inválida");
            return;
        }
    };
    
    println!("Resultado: {}", resultado);
}
```

### Proyecto 2: Adivina el Número
**Conceptos:** loops, random, comparaciones, error handling
```rust
// Cargo.toml
// [dependencies]
// rand = "0.8"

use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("🎲 Adivina el número (1-100)");
    
    let numero_secreto = rand::thread_rng().gen_range(1..=100);
    let mut intentos = 0;
    
    loop {
        println!("\nIngresa tu número:");
        
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Error");
        
        let entrada: u32 = match entrada.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Ingresa un número válido");
                continue;
            }
        };
        
        intentos += 1;
        
        match entrada.cmp(&numero_secreto) {
            Ordering::Less => println!("📉 Muy bajo"),
            Ordering::Greater => println!("📈 Muy alto"),
            Ordering::Equal => {
                println!("🎉 ¡Correcto en {} intentos!", intentos);
                break;
            }
        }
    }
}
```

### Proyecto 3: Gestor de Tareas Simple
**Conceptos:** Vec, structs, métodos, ownership
```rust
struct Tarea {
    descripcion: String,
    completada: bool,
}

impl Tarea {
    fn nueva(descripcion: String) -> Self {
        Tarea {
            descripcion,
            completada: false,
        }
    }
    
    fn completar(&mut self) {
        self.completada = true;
    }
}

struct ListaTareas {
    tareas: Vec<Tarea>,
}

impl ListaTareas {
    fn nueva() -> Self {
        ListaTareas { tareas: Vec::new() }
    }
    
    fn agregar(&mut self, descripcion: String) {
        self.tareas.push(Tarea::nueva(descripcion));
    }
    
    fn mostrar(&self) {
        for (i, tarea) in self.tareas.iter().enumerate() {
            let estado = if tarea.completada { "✅" } else { "⬜" };
            println!("{} {}. {}", estado, i + 1, tarea.descripcion);
        }
    }
    
    fn completar_tarea(&mut self, indice: usize) {
        if let Some(tarea) = self.tareas.get_mut(indice) {
            tarea.completar();
        }
    }
}
```

---

## 🚀 Proyectos Intermedios

### Proyecto 4: Analizador de Logs
**Conceptos:** File I/O, HashMap, iteradores, Result
```rust
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct EstadisticasLog {
    total_lineas: usize,
    niveles: HashMap<String, usize>,
}

impl EstadisticasLog {
    fn nueva() -> Self {
        EstadisticasLog {
            total_lineas: 0,
            niveles: HashMap::new(),
        }
    }
    
    fn analizar_linea(&mut self, linea: &str) {
        self.total_lineas += 1;
        
        if let Some(nivel) = extraer_nivel(linea) {
            *self.niveles.entry(nivel).or_insert(0) += 1;
        }
    }
    
    fn mostrar_resumen(&self) {
        println!("\n📊 Estadísticas del Log:");
        println!("Total líneas: {}", self.total_lineas);
        println!("\nPor nivel:");
        
        for (nivel, cantidad) in &self.niveles {
            println!("  {}: {}", nivel, cantidad);
        }
    }
}

fn extraer_nivel(linea: &str) -> Option<String> {
    let niveles = ["ERROR", "WARN", "INFO", "DEBUG"];
    
    for nivel in &niveles {
        if linea.contains(nivel) {
            return Some(nivel.to_string());
        }
    }
    None
}

fn analizar_log(ruta: &str) -> Result<EstadisticasLog, std::io::Error> {
    let archivo = File::open(ruta)?;
    let lector = BufReader::new(archivo);
    let mut stats = EstadisticasLog::nueva();
    
    for linea in lector.lines() {
        if let Ok(linea) = linea {
            stats.analizar_linea(&linea);
        }
    }
    
    Ok(stats)
}
```

### Proyecto 5: API REST Client
**Conceptos:** async/await, JSON, HTTP, error handling
```rust
// Cargo.toml
// [dependencies]
// tokio = { version = "1", features = ["full"] }
// reqwest = { version = "0.11", features = ["json"] }
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Usuario {
    id: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET request
    let url = "https://jsonplaceholder.typicode.com/users/1";
    let usuario: Usuario = reqwest::get(url)
        .await?
        .json()
        .await?;
    
    println!("Usuario: {:?}", usuario);
    
    // POST request
    let nuevo_usuario = Usuario {
        id: 0,
        name: String::from("Test User"),
        email: String::from("test@example.com"),
    };
    
    let cliente = reqwest::Client::new();
    let respuesta = cliente
        .post("https://jsonplaceholder.typicode.com/users")
        .json(&nuevo_usuario)
        .send()
        .await?;
    
    println!("Status: {}", respuesta.status());
    
    Ok(())
}
```

### Proyecto 6: Mini Base de Datos en Memoria
**Conceptos:** Generics, Traits, HashMap, serialización
```rust
use std::collections::HashMap;
use std::hash::Hash;

trait BaseDatos<K, V> {
    fn insertar(&mut self, clave: K, valor: V);
    fn obtener(&self, clave: &K) -> Option<&V>;
    fn eliminar(&mut self, clave: &K) -> Option<V>;
    fn listar(&self) -> Vec<(&K, &V)>;
}

struct DB<K, V> 
where
    K: Eq + Hash,
{
    datos: HashMap<K, V>,
}

impl<K, V> DB<K, V> 
where
    K: Eq + Hash,
{
    fn nueva() -> Self {
        DB {
            datos: HashMap::new(),
        }
    }
}

impl<K, V> BaseDatos<K, V> for DB<K, V>
where
    K: Eq + Hash,
{
    fn insertar(&mut self, clave: K, valor: V) {
        self.datos.insert(clave, valor);
    }
    
    fn obtener(&self, clave: &K) -> Option<&V> {
        self.datos.get(clave)
    }
    
    fn eliminar(&mut self, clave: &K) -> Option<V> {
        self.datos.remove(clave)
    }
    
    fn listar(&self) -> Vec<(&K, &V)> {
        self.datos.iter().collect()
    }
}

// Uso
fn main() {
    let mut db: DB<String, i32> = DB::nueva();
    db.insertar(String::from("edad"), 25);
    
    if let Some(valor) = db.obtener(&String::from("edad")) {
        println!("Edad: {}", valor);
    }
}
```

---

## 🔥 Proyectos Avanzados

### Proyecto 7: Web Scraper Concurrente
**Conceptos:** async/await, concurrency, HTML parsing
```rust
// Cargo.toml
// [dependencies]
// tokio = { version = "1", features = ["full"] }
// reqwest = "0.11"
// scraper = "0.17"
// futures = "0.3"

use scraper::{Html, Selector};
use futures::future::join_all;

#[derive(Debug)]
struct Articulo {
    titulo: String,
    url: String,
}

async fn extraer_articulos(url: &str) -> Result<Vec<Articulo>, Box<dyn std::error::Error>> {
    let respuesta = reqwest::get(url).await?.text().await?;
    let documento = Html::parse_document(&respuesta);
    
    let selector_titulo = Selector::parse("h2.titulo").unwrap();
    let selector_link = Selector::parse("a").unwrap();
    
    let mut articulos = Vec::new();
    
    for elemento in documento.select(&selector_titulo) {
        if let Some(link) = elemento.select(&selector_link).next() {
            articulos.push(Articulo {
                titulo: elemento.text().collect::<String>(),
                url: link.value().attr("href").unwrap_or("").to_string(),
            });
        }
    }
    
    Ok(articulos)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://example.com/page1",
        "https://example.com/page2",
        "https://example.com/page3",
    ];
    
    // Scraping concurrente
    let tareas: Vec<_> = urls
        .iter()
        .map(|url| extraer_articulos(url))
        .collect();
    
    let resultados = join_all(tareas).await;
    
    for (i, resultado) in resultados.iter().enumerate() {
        match resultado {
            Ok(articulos) => {
                println!("\n📄 Página {}:", i + 1);
                for articulo in articulos {
                    println!("  - {}", articulo.titulo);
                }
            }
            Err(e) => println!("❌ Error en página {}: {}", i + 1, e),
        }
    }
    
    Ok(())
}
```

### Proyecto 8: Servidor HTTP Simple
**Conceptos:** Networking, multithreading, HTTP protocol
```rust
// Cargo.toml
// [dependencies]
// tokio = { version = "1", features = ["full"] }
// warp = "0.3"
// serde = { version = "1.0", features = ["derive"] }

use warp::{Filter, reply, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Mensaje {
    id: u64,
    contenido: String,
}

type DB = Arc<Mutex<Vec<Mensaje>>>;

#[tokio::main]
async fn main() {
    let db: DB = Arc::new(Mutex::new(Vec::new()));
    
    // GET /mensajes
    let listar = warp::path("mensajes")
        .and(warp::get())
        .and(with_db(db.clone()))
        .map(|db: DB| {
            let mensajes = db.lock().unwrap();
            reply::json(&*mensajes)
        });
    
    // POST /mensajes
    let crear = warp::path("mensajes")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .map(|nuevo: Mensaje, db: DB| {
            let mut mensajes = db.lock().unwrap();
            mensajes.push(nuevo);
            reply::with_status("Creado", StatusCode::CREATED)
        });
    
    let rutas = listar.or(crear);
    
    println!("🚀 Servidor corriendo en http://localhost:3030");
    warp::serve(rutas).run(([127, 0, 0, 1], 3030)).await;
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
```

---

## 🤖 Proyecto Final: Bot de Discord

### Estructura del Proyecto
```
discord-bot/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── ping.rs
│   │   └── info.rs
│   └── handlers/
│       ├── mod.rs
│       └── events.rs
```

### Cargo.toml
```toml
[package]
name = "discord-bot"
version = "0.1.0"
edition = "2021"

[dependencies]
serenity = { version = "0.12", default-features = false, features = ["client", "gateway", "rustls_backend", "model"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
dotenv = "0.15"
```

### main.rs
```rust
mod commands;
mod handlers;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{command, group},
};

#[group]
#[commands(ping, info, encuesta)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Responder a menciones
        if msg.mentions_me(&ctx).await.unwrap_or(false) {
            if let Err(e) = msg.reply(&ctx, "¡Hola! Usa !help para ver comandos").await {
                println!("Error al responder: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("✅ {} está conectado!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Error al cargar .env");
    
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Se requiere DISCORD_TOKEN en .env");
    
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error al crear cliente");
    
    if let Err(why) = client.start().await {
        println!("Error del cliente: {:?}", why);
    }
}

// ==================== COMANDOS ====================

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "🏓 Pong!").await?;
    Ok(())
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let info = format!(
        "**Bot Info**\n\
        📊 Servidor: {}\n\
        👤 Usuarios: {}\n\
        💬 Canales: {}",
        msg.guild_id.unwrap_or_default(),
        msg.guild(&ctx).map(|g| g.member_count).unwrap_or(0),
        msg.guild(&ctx).map(|g| g.channels.len()).unwrap_or(0)
    );
    
    msg.channel_id.say(&ctx, info).await?;
    Ok(())
}

#[command]
#[description = "Crear una encuesta rápida"]
#[usage = "!encuesta \"Pregunta\" \"Opción1\" \"Opción2\""]
async fn encuesta(ctx: &Context, msg: &Message) -> CommandResult {
    let contenido = &msg.content;
    let partes: Vec<&str> = contenido
        .split('"')
        .filter(|s| !s.trim().is_empty() && s.trim() != "!encuesta")
        .collect();
    
    if partes.len() < 3 {
        msg.reply(ctx, "❌ Uso: !encuesta \"Pregunta\" \"Op1\" \"Op2\"").await?;
        return Ok(());
    }
    
    let pregunta = partes[0];
    let opciones = &partes[1..];
    
    let emojis = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣"];
    
    let mut texto_encuesta = format!("📊 **{}**\n\n", pregunta);
    for (i, opcion) in opciones.iter().enumerate() {
        if i < emojis.len() {
            texto_encuesta.push_str(&format!("{} {}\n", emojis[i], opcion));
        }
    }
    
    let encuesta_msg = msg.channel_id.say(&ctx, texto_encuesta).await?;
    
    for (i, _) in opciones.iter().enumerate() {
        if i < emojis.len() {
            encuesta_msg.react(&ctx, emojis[i].parse().unwrap()).await?;
        }
    }
    
    Ok(())
}
```

### Funcionalidades Adicionales

#### Sistema de Niveles (commands/niveles.rs)
```rust
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::command};
use std::collections::HashMap;
use std::sync::Arc;

pub struct NivelesData;

impl TypeMapKey for NivelesData {
    type Value = Arc<Mutex<HashMap<u64, u32>>>;
}

#[command]
#[description = "Ver tu nivel actual"]
async fn nivel(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let niveles = data.get::<NivelesData>().unwrap().lock().await;
    
    let xp = niveles.get(&msg.author.id.0).unwrap_or(&0);
    let nivel = (*xp as f32 / 100.0).sqrt() as u32;
    
    msg.reply(
        ctx,
        format!("⭐ Nivel: {} | XP: {}", nivel, xp)
    ).await?;
    
    Ok(())
}

pub async fn agregar_xp(ctx: &Context, user_id: u64, cantidad: u32) {
    let data = ctx.data.read().await;
    let mut niveles = data.get::<NivelesData>().unwrap().lock().await;
    
    let xp_actual = niveles.entry(user_id).or_insert(0);
    *xp_actual += cantidad;
}
```

#### Sistema de Moderación
```rust
#[command]
#[required_permissions(KICK_MEMBERS)]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.is_empty() {
        msg.reply(ctx, "❌ Menciona a un usuario para expulsar").await?;
        return Ok(());
    }
    
    let usuario = &msg.mentions[0];
    let razon = msg.content
        .split_whitespace()
        .skip(2)
        .collect::<Vec<_>>()
        .join(" ");
    
    if let Some(guild_id) = msg.guild_id {
        if let Ok(member) = guild_id.member(&ctx, usuario.id).await {
            member.kick_with_reason(&ctx, &razon).await?;
            msg.reply(
                ctx,
                format!("👢 {} ha sido expulsado. Razón: {}", usuario.name, razon)
            ).await?;
        }
    }
    
    Ok(())
}

#[command]
#[required_permissions(BAN_MEMBERS)]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    // Similar a kick pero con ban
    Ok(())
}

#[command]
#[required_permissions(MANAGE_MESSAGES)]
async fn limpiar(ctx: &Context, msg: &Message) -> CommandResult {
    let cantidad: u64 = msg.content
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    
    let mensajes = msg.channel_id
        .messages(&ctx, |m| m.limit(cantidad))
        .await?;
    
    msg.channel_id.delete_messages(&ctx, mensajes).await?;
    
    let confirmacion = msg.channel_id
        .say(&ctx, format!("🗑️ {} mensajes eliminados", cantidad))
        .await?;
    
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    confirmacion.delete(&ctx).await?;
    
    Ok(())
}
```

### Archivo .env
```
DISCORD_TOKEN=tu_token_aqui
```

### Comandos del Bot
```
!ping           - Verificar latencia
!info           - Información del servidor
!nivel          - Ver tu nivel y XP
!encuesta       - Crear encuesta rápida
!kick @user     - Expulsar miembro (requiere permisos)
!ban @user      - Banear miembro (requiere permisos)
!limpiar [N]    - Borrar N mensajes (requiere permisos)
```

### Cómo Ejecutar

1. **Crear aplicación en Discord:**
   - Ve a https://discord.com/developers/applications
   - Crea nueva aplicación
   - En "Bot", crea un bot y copia el token
   - En "OAuth2" > "URL Generator", selecciona `bot` y los permisos necesarios
   - Usa la URL generada para invitar el bot

2. **Configurar proyecto:**
```bash
   cargo new discord-bot
   cd discord-bot
   # Edita Cargo.toml y src/main.rs
   # Crea archivo .env con tu token
```

3. **Ejecutar:**
```bash
   cargo run
```

### Próximos Pasos

- Agregar base de datos (SQLite/PostgreSQL) con `sqlx`
- Implementar comandos slash (Discord Interactions)
- Sistema de economía virtual
- Integración con APIs externas
- Dashboard web con `axum`
- Sistema de música con `songbird`

---

## 📖 Recursos Adicionales

- [The Rust Book (ES)](https://book.rustlang-es.org/)
- [Comprehensive Rust (ES)](https://google.github.io/comprehensive-rust/es/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Serenity Documentation](https://docs.rs/serenity/)

## 🎯 Ruta de Aprendizaje Sugerida

1. **Semana 1-2:** Fundamentos + Proyectos 1-3
2. **Semana 3-4:** Proyectos 4-6 + Conceptos async
3. **Semana 5-6:** Proyectos 7-8 + Web frameworks
4. **Semana 7+:** Bot Discord + Personalizaciones

¡Éxito en tu viaje con Rust! 🦀
