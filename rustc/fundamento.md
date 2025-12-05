# Rust - Fundamentos con Arquitectura en Capas
## Proyectos Graduales de Aprendizaje

---

## 🎯 Objetivo General

Aprender Rust mediante proyectos prácticos que implementan arquitectura en 3 capas:
- **Capa de Presentación** (UI/CLI)
- **Capa de Lógica/Negocio** (Business Logic)
- **Capa de Datos** (Persistencia)

Cada proyecto introduce conceptos nuevos manteniendo la arquitectura consistente.

---

## 📁 Estructura de Carpetas (Estándar para todos)
```
proyecto/
├── src/
│   ├── main.rs              # Punto de entrada
│   ├── presentacion/
│   │   └── mod.rs           # CLI/UI
│   ├── logica/
│   │   └── mod.rs           # Reglas de negocio
│   ├── datos/
│   │   └── mod.rs           # Persistencia
│   └── lib.rs               # Exporta módulos
├── Cargo.toml
└── datos/                   # Archivos de datos
```

---

## 🟢 NIVEL 1: Fundamentos Básicos

### Proyecto 1.1: Calculadora de Presupuesto Personal
**Duración estimada**: 2-3 días

#### 📖 Teoría a Estudiar ANTES de Programar

**1. Ownership (Sistema de Propiedad)**
- **Concepto**: Cada valor tiene un único dueño
- **Reglas**:
  - Un valor solo puede tener un dueño a la vez
  - Cuando el dueño sale del scope, se libera la memoria
  - Mover vs Copiar: tipos simples (i32, bool) se copian, tipos complejos (String, Vec) se mueven
- **Lecturas**: Rust Book Capítulo 4.1
- **Tiempo estimado**: 1-2 horas

**2. References y Borrowing (Préstamos)**
- **Concepto**: Usar datos sin tomar ownership
- **Tipos**:
  - `&T`: referencia inmutable (puedes leer)
  - `&mut T`: referencia mutable (puedes modificar)
- **Regla de oro**: O muchas inmutables O una mutable (nunca ambas)
- **Lecturas**: Rust Book Capítulo 4.2
- **Tiempo estimado**: 1-2 horas

**3. Structs y Métodos**
- **Concepto**: Tipos de datos personalizados
- **Sintaxis**:
```rust
  struct Persona {
      nombre: String,
      edad: u32,
  }
  
  impl Persona {
      fn saludar(&self) {
          println!("Hola, soy {}", self.nombre);
      }
  }
```
- **Lecturas**: Rust Book Capítulo 5
- **Tiempo estimado**: 2 horas

**4. String vs &str**
- **String**: tipo owned, heap, mutable, crece dinámicamente
- **&str**: slice, referencia, inmutable, tamaño fijo
- **Cuándo usar cada uno**:
  - `String`: cuando necesitas modificar o ser dueño
  - `&str`: para parámetros de funciones que solo leen
- **Lecturas**: Rust Book Capítulo 4.3 (Slices)
- **Tiempo estimado**: 1 hora

**5. Vectores (Vec<T>)**
- **Concepto**: Array dinámico en heap
- **Operaciones básicas**: `push()`, `pop()`, `len()`, indexación
- **Ownership**: mover vs clonar elementos
- **Lecturas**: Rust Book Capítulo 8.1
- **Tiempo estimado**: 1 hora

**6. Result<T, E> - Manejo de Errores**
- **Concepto**: representa operaciones que pueden fallar
- **Variantes**: `Ok(T)` o `Err(E)`
- **Operadores**: `?` para propagación, `unwrap()` solo en desarrollo
- **Lecturas**: Rust Book Capítulo 9
- **Tiempo estimado**: 2 horas

**7. Módulos y Visibilidad**
- **Concepto**: organizar código en archivos
- **Keywords**: `mod`, `pub`, `use`
- **Estructura**: `mod.rs` como punto de entrada del módulo
- **Lecturas**: Rust Book Capítulo 7
- **Tiempo estimado**: 1-2 horas

**Total teoría Proyecto 1.1**: ~10-12 horas

#### Conceptos a aplicar en código:
- `struct` y métodos (`impl`)
- Ownership básico
- `String` vs `&str`
- `Vec<T>` y operaciones básicas
- `Result<T, E>` para manejo de errores
- Lectura/escritura de archivos

#### Arquitectura:

**Presentación** (`presentacion/mod.rs`):
```rust
// Menú CLI
// - Agregar gasto
// - Listar gastos
// - Ver total
// - Salir
```

**Lógica** (`logica/mod.rs`):
```rust
pub struct Gasto {
    descripcion: String,
    monto: f64,
    categoria: String,
}

pub struct Presupuesto {
    gastos: Vec<Gasto>,
}

impl Presupuesto {
    pub fn agregar_gasto(&mut self, gasto: Gasto)
    pub fn total(&self) -> f64
    pub fn listar_gastos(&self) -> &Vec<Gasto>
}
```

**Datos** (`datos/mod.rs`):
```rust
// Guardar/cargar desde archivo TXT
pub fn guardar(presupuesto: &Presupuesto) -> Result<(), std::io::Error>
pub fn cargar() -> Result<Presupuesto, std::io::Error>
```

#### Errores esperados a resolver:
- ❌ `value moved here` - aprender sobre borrowing
- ❌ `cannot borrow as mutable` - entender `&` vs `&mut`
- ❌ `expected struct Presupuesto, found ()` - retornos de funciones

#### Dependencias (Cargo.toml):
```toml
[dependencies]
# Ninguna - solo std library
```

---

### Proyecto 1.2: Lista de Tareas con Prioridades
**Duración estimada**: 3-4 días

#### 📖 Teoría a Estudiar ANTES de Programar

**1. Enums (Enumeraciones)**
- **Concepto**: tipo que puede ser una de varias variantes
- **Sintaxis básica**:
```rust
  enum Estado {
      Activo,
      Inactivo,
      Pausado,
  }
```
- **Enums con datos**:
```rust
  enum Resultado {
      Exito(String),
      Error(i32),
  }
```
- **Lecturas**: Rust Book Capítulo 6.1
- **Tiempo estimado**: 2 horas

**2. Pattern Matching**
- **Concepto**: control de flujo exhaustivo
- **Sintaxis**:
```rust
  match valor {
      Patron1 => expresion1,
      Patron2 => expresion2,
      _ => default,  // catch-all
  }
```
- **if let**: forma compacta para un solo patrón
- **Lecturas**: Rust Book Capítulo 6.2
- **Tiempo estimado**: 2 horas

**3. Option<T>**
- **Concepto**: representa valor que puede estar ausente
- **Variantes**: `Some(T)` o `None`
- **Diferencia con Result**: Option para ausencia, Result para errores
- **Métodos útiles**: `unwrap_or()`, `map()`, `and_then()`
- **Lecturas**: Rust Book Capítulo 6.1 (parte de enums)
- **Tiempo estimado**: 1-2 horas

**4. Traits - Derivación Automática**
- **Concepto**: interfaces que definen comportamiento
- **Derive macros**: `#[derive(Debug, Clone, PartialEq)]`
- **Traits comunes**:
  - `Debug`: para imprimir con `{:?}`
  - `Clone`: duplicar valores
  - `PartialEq`: comparar con `==`
- **Lecturas**: Rust Book Capítulo 10.2 (parte básica)
- **Tiempo estimado**: 1 hora

**5. Iteradores Básicos**
- **Concepto**: procesamiento de colecciones
- **Métodos fundamentales**:
  - `iter()`: iterador inmutable
  - `iter_mut()`: iterador mutable
  - `into_iter()`: consume la colección
- **Adaptadores**: `filter()`, `map()`, `collect()`
- **Lecturas**: Rust Book Capítulo 13.2
- **Tiempo estimado**: 2 horas

**6. Closures (Funciones Anónimas)**
- **Concepto**: funciones inline que capturan contexto
- **Sintaxis**: `|param| expresion`
- **Uso típico**: con iteradores
```rust
  vec.iter().filter(|x| x > &5).collect()
```
- **Lecturas**: Rust Book Capítulo 13.1
- **Tiempo estimado**: 2 horas

**7. Serialización con Serde**
- **Concepto**: convertir structs a JSON y viceversa
- **Derive**: `#[derive(Serialize, Deserialize)]`
- **Uso básico**:
```rust
  let json = serde_json::to_string(&objeto)?;
  let objeto: Tipo = serde_json::from_str(&json)?;
```
- **Lecturas**: Documentación de serde
- **Tiempo estimado**: 1 hora

**Total teoría Proyecto 1.2**: ~11-13 horas

#### Conceptos nuevos:
- `enum` y pattern matching
- `Option<T>` para valores opcionales
- Traits básicos (`Display`, `Debug`)
- Iteradores y closures
- Serialización JSON con `serde`

#### Arquitectura:

**Presentación**:
```rust
// CLI con comandos:
// add "tarea" --prioridad alta
// list --filtro completadas
// complete <id>
// delete <id>
```

**Lógica**:
```rust
#[derive(Debug)]
pub enum Prioridad {
    Alta,
    Media,
    Baja,
}

pub struct Tarea {
    id: u32,
    descripcion: String,
    completada: bool,
    prioridad: Prioridad,
}

pub struct GestorTareas {
    tareas: Vec<Tarea>,
    siguiente_id: u32,
}

impl GestorTareas {
    pub fn agregar(&mut self, desc: String, prio: Prioridad) -> u32
    pub fn completar(&mut self, id: u32) -> Result<(), String>
    pub fn buscar(&self, id: u32) -> Option<&Tarea>
    pub fn filtrar_por_prioridad(&self, prio: Prioridad) -> Vec<&Tarea>
}
```

**Datos**:
```rust
// JSON con serde
use serde::{Serialize, Deserialize};

pub fn guardar_json(gestor: &GestorTareas) -> Result<(), Box<dyn Error>>
pub fn cargar_json() -> Result<GestorTareas, Box<dyn Error>>
```

#### Errores esperados:
- ❌ Pattern matching no exhaustivo en `match`
- ❌ Confusión entre `Some(T)` y `Ok(T)`
- ❌ Lifetimes en funciones que retornan referencias

#### Dependencias:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 🟡 NIVEL 2: Conceptos Intermedios

### Proyecto 2.1: Sistema de Inventario con Búsqueda
**Duración estimada**: 4-5 días

#### 📖 Teoría a Estudiar ANTES de Programar

**1. Traits Personalizados**
- **Concepto**: definir comportamiento compartido
- **Sintaxis**:
```rust
  trait MiTrait {
      fn metodo(&self) -> String;
      fn metodo_default(&self) { /* implementación */ }
  }
  
  impl MiTrait for MiStruct {
      fn metodo(&self) -> String { /* ... */ }
  }
```
- **Lecturas**: Rust Book Capítulo 10.2 (completo)
- **Tiempo estimado**: 3 horas

**2. Genéricos (Generics)**
- **Concepto**: código que funciona con múltiples tipos
- **En funciones**:
```rust
  fn procesar<T>(item: T) -> T { item }
```
- **En structs**:
```rust
  struct Contenedor<T> {
      valor: T,
  }
```
- **Lecturas**: Rust Book Capítulo 10.1
- **Tiempo estimado**: 2-3 horas

**3. Trait Bounds**
- **Concepto**: restricciones en tipos genéricos
- **Sintaxis**:
```rust
  fn funcion<T: Display + Clone>(item: T) { /* ... */ }
  
  // Equivalente con where
  fn funcion<T>(item: T) 
  where
      T: Display + Clone
  { /* ... */ }
```
- **Lecturas**: Rust Book Capítulo 10.2 (traits como parámetros)
- **Tiempo estimado**: 2 horas

**4. HashMap<K, V>**
- **Concepto**: diccionario clave-valor
- **Requisitos**: K debe implementar `Eq + Hash`
- **Operaciones**: `insert()`, `get()`, `remove()`, `entry()`
- **Ownership**: claves y valores se mueven al HashMap
- **Lecturas**: Rust Book Capítulo 8.3
- **Tiempo estimado**: 2 horas

**5. Módulos Anidados**
- **Concepto**: organización jerárquica
- **Estructura**:
```
  logica/
  ├── mod.rs
  ├── inventario.rs
  └── productos/
      ├── mod.rs
      └── tipos.rs
```
- **Paths**: absolutos (`crate::`) vs relativos (`super::`)
- **Lecturas**: Rust Book Capítulo 7 (profundizar)
- **Tiempo estimado**: 1-2 horas

**6. Tests Unitarios**
- **Concepto**: verificar funcionalidad de forma automática
- **Sintaxis**:
```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      
      #[test]
      fn test_nombre() {
          assert_eq!(2 + 2, 4);
      }
  }
```
- **Macros de assertion**: `assert!()`, `assert_eq!()`, `assert_ne!()`
- **Lecturas**: Rust Book Capítulo 11
- **Tiempo estimado**: 2-3 horas

**7. Box<dyn Error>**
- **Concepto**: tipo de error genérico (trait object)
- **Uso**: cuando función puede retornar múltiples tipos de error
- **Sintaxis**: `Result<T, Box<dyn Error>>`
- **Lecturas**: Rust Book Capítulo 17.2 (trait objects)
- **Tiempo estimado**: 2 horas

**Total teoría Proyecto 2.1**: ~14-17 horas

#### Conceptos nuevos:
- Traits personalizados
- Trait bounds y genéricos `<T>`
- `HashMap<K, V>` para búsquedas rápidas
- Módulos anidados
- Tests unitarios con `#[cfg(test)]`

#### Arquitectura:

**Presentación**:
```rust
// Menú interactivo con validación de entrada
// Búsqueda por nombre, categoría, rango de precio
```

**Lógica**:
```rust
pub trait Producto {
    fn nombre(&self) -> &str;
    fn precio(&self) -> f64;
    fn categoria(&self) -> &str;
    fn valorar(&self) -> f64; // Precio * cantidad
}

pub struct Item {
    id: String,
    nombre: String,
    precio: f64,
    cantidad: u32,
    categoria: String,
}

impl Producto for Item { /* ... */ }

pub struct Inventario {
    items: HashMap<String, Item>,
}

impl Inventario {
    pub fn agregar<T: Producto>(&mut self, item: T)
    pub fn buscar_por_categoria(&self, cat: &str) -> Vec<&Item>
    pub fn buscar_por_rango_precio(&self, min: f64, max: f64) -> Vec<&Item>
    pub fn valor_total(&self) -> f64
}
```

**Datos**:
```rust
// CSV con serde_csv
pub fn exportar_csv(inv: &Inventario) -> Result<(), Box<dyn Error>>
pub fn importar_csv(path: &str) -> Result<Inventario, Box<dyn Error>>
```

#### Errores esperados:
- ❌ `trait bound not satisfied` - implementar traits correctamente
- ❌ Confusión entre `&str` y `String` en HashMap keys
- ❌ Tests que fallan por ownership

#### Dependencias:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv = "1.3"

[dev-dependencies]
# Para tests más avanzados si quieres
```

---

### Proyecto 2.2: Registro de Contactos con SQLite
**Duración estimada**: 5-6 días

#### 📖 Teoría a Estudiar ANTES de Programar

**1. Lifetimes (Tiempos de Vida)**
- **Concepto**: anotar cuánto viven las referencias
- **Problema que resuelven**: evitar referencias colgantes (dangling)
- **Sintaxis**:
```rust
  fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() { x } else { y }
  }
```
- **Reglas de elisión**: cuándo Rust infiere lifetimes
- **Lecturas**: Rust Book Capítulo 10.3
- **Tiempo estimado**: 3-4 horas (concepto difícil)

**2. Lifetimes en Structs**
- **Concepto**: structs que contienen referencias
- **Sintaxis**:
```rust
  struct Referenciador<'a> {
      dato: &'a str,
  }
```
- **Regla**: el struct no puede vivir más que la referencia
- **Lecturas**: Rust Book Capítulo 10.3 (parte de structs)
- **Tiempo estimado**: 2 horas

**3. Trait Objects (dyn Trait)**
- **Concepto**: polimorfismo dinámico (runtime)
- **vs Genéricos**: trait objects = runtime, genéricos = compile time
- **Sintaxis**: `Box<dyn Trait>`, `&dyn Trait`
- **Limitaciones**: no puede tener métodos genéricos
- **Lecturas**: Rust Book Capítulo 17.2
- **Tiempo estimado**: 2-3 horas

**4. SQL y Bases de Datos**
- **Conceptos básicos SQL**:
  - CREATE TABLE
  - INSERT, SELECT, UPDATE, DELETE
  - WHERE, ORDER BY, LIMIT
- **Prepared Statements**: prevenir SQL injection
- **Lecturas**: Tutorial SQL básico (externo)
- **Tiempo estimado**: 3-4 horas (si no sabes SQL)

**5. rusqlite - Librería SQLite**
- **Conexión**: `Connection::open()`
- **Ejecutar queries**: `execute()` vs `query()`
- **Mapear resultados**: `query_map()`, `query_row()`
- **Transacciones**: `transaction()`
- **Lecturas**: Documentación de rusqlite
- **Tiempo estimado**: 2 horas

**6. Patrón Repository**
- **Concepto**: abstraer acceso a datos
- **Beneficio**: cambiar DB sin afectar lógica
- **Estructura**:
```rust
  trait Repository {
      fn guardar(&mut self, item: Item) -> Result<()>;
      fn obtener(&self, id: Id) -> Result<Item>;
  }
```
- **Lecturas**: Diseño de software (externo)
- **Tiempo estimado**: 1-2 horas

**7. impl Trait en Retornos**
- **Concepto**: retornar tipo que implementa trait sin especificar cuál
- **Sintaxis**: `fn obtener() -> impl Iterator<Item = String>`
- **Limitación**: solo un tipo concreto puede retornarse
- **vs dyn**: impl = compile time, dyn = runtime
- **Lecturas**: Rust Book Capítulo 10.2 (trait como retorno)
- **Tiempo estimado**: 1 hora

**Total teoría Proyecto 2.2**: ~14-18 horas

#### Conceptos nuevos:
- Manejo de bases de datos (SQLite)
- `Box<dyn Error>` para error handling genérico
- Lifetimes explícitos en structs
- `impl Trait` en retornos
- Refactoring de código repetitivo

#### Arquitectura:

**Presentación**:
```rust
// CLI con comandos tipo SQL
// add --nombre "Juan" --email "juan@mail.com" --telefono "123"
// search --nombre "Juan"
// update <id> --email "nuevo@mail.com"
// delete <id>
```

**Lógica**:
```rust
pub struct Contacto {
    id: Option<i32>, // None cuando es nuevo
    nombre: String,
    email: String,
    telefono: Option<String>,
}

pub struct GestorContactos<'a> {
    repositorio: &'a mut dyn RepositorioContactos,
}

pub trait RepositorioContactos {
    fn crear(&mut self, contacto: Contacto) -> Result<i32, Box<dyn Error>>;
    fn leer(&self, id: i32) -> Result<Contacto, Box<dyn Error>>;
    fn actualizar(&mut self, contacto: Contacto) -> Result<(), Box<dyn Error>>;
    fn eliminar(&mut self, id: i32) -> Result<(), Box<dyn Error>>;
    fn buscar(&self, nombre: &str) -> Result<Vec<Contacto>, Box<dyn Error>>;
}
```

**Datos**:
```rust
use rusqlite::{Connection, Result};

pub struct SqliteRepositorio {
    conn: Connection,
}

impl RepositorioContactos for SqliteRepositorio {
    // Implementar CRUD con SQL
}

impl SqliteRepositorio {
    pub fn nuevo(path: &str) -> Result<Self>
    fn inicializar_schema(&self) -> Result<()>
}
```

#### Errores esperados:
- ❌ Lifetimes conflictivos en structs con referencias
- ❌ SQL injection (aprender prepared statements)
- ❌ `cannot return value referencing local variable`

#### Dependencias:
```toml
[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
```

---

## 🔴 NIVEL 3: Conceptos Avanzados

### Proyecto 3.1: API REST de Notas (Async)
**Duración estimada**: 6-8 días

#### 📖 Teoría a Estudiar ANTES de Programar

**1. Programación Asíncrona - Conceptos**
- **Problema**: operaciones bloqueantes (I/O, red)
- **Solución**: concurrencia sin threads
- **Event loop**: ejecutor que maneja múltiples tareas
- **Comparación con Python**: similar a `asyncio`
- **Lecturas**: Rust Book Capítulo 20 (intro)
- **Tiempo estimado**: 2 horas

**2. Futures y async/await**
- **Future**: valor que estará disponible en el futuro
- **async fn**: función que retorna Future
- **await**: esperar resultado de Future
- **Sintaxis**:
```rust
  async fn obtener_datos() -> Result<String> {
      let respuesta = cliente.get(url).await?;
      Ok(respuesta.text().await?)
  }
```
- **Lecturas**: Async Book (oficial de Rust)
- **Tiempo estimado**: 3-4 horas

**3. Tokio Runtime**
- **Concepto**: ejecutor de Futures (como asyncio.run())
- **Macros**: `#[tokio::main]`
- **Tasks**: `tokio::spawn()`
- **Diferencia con threads**: más liviano, mismo thread
- **Lecturas**: Documentación de Tokio
- **Tiempo estimado**: 2 horas

**4. Arc<T> - Atomic Reference Counting**
- **Concepto**: ownership compartido thread-safe
- **vs Rc**: Arc es atómico (puede cruzar threads)
- **Clonación**: `Arc::clone()` solo incrementa contador
- **Uso**: compartir datos inmutables entre threads/tasks
- **Lecturas**: Rust Book Capítulo 16.3
- **Tiempo estimado**: 2 horas

**5. Mutex<T> - Mutual Exclusion**
- **Concepto**: acceso exclusivo a datos compartidos
- **Uso con Arc**: `Arc<Mutex<T>>`
- **lock()**: obtener guard, auto-unlock al salir del scope
- **Deadlocks**: cómo evitarlos (orden de locks)
- **Lecturas**: Rust Book Capítulo 16.3
- **Tiempo estimado**: 2-3 horas

**6. Send y Sync Traits**
- **Send**: tipo puede transferirse entre threads
- **Sync**: tipo puede compartirse entre threads (&T es Send)
- **Auto-implementación**: mayoría de tipos
- **Importante para**: `Arc<Mutex<T>>` requiere T: Send
- **Lecturas**: Rust Book Capítulo 16.4
- **Tiempo estimado**: 1-2 horas

**7. Actix-web Framework**
- **Concepto**: framework web asíncrono
- **Estructura**:
  - `App`: aplicación
  - `web::scope()`: agrupación de rutas
  - `web::Json`: extractor de JSON
  - Handlers: funciones async
- **Lecturas**: Documentación de Actix-web
- **Tiempo estimado**: 3-4 horas

**8. Estado Compartido en Web**
- **Problema**: múltiples requests accediendo a datos
- **Solución**: `web::Data<Arc<Mutex<T>>>`
- **Extractor**: inyección automática en handlers
- **Patrón**:
```rust
  async fn handler(data: web::Data<Arc<Mutex<DB>>>) {
      let db = data.lock().unwrap();
      // usar db
  }
```
- **Lecturas**: Guía de Actix-web (State)
- **Tiempo estimado**: 2 horas

**Total teoría Proyecto 3.1**: ~17-23 horas

#### Conceptos nuevos:
- `async`/`await` y Futures
- `Arc<Mutex<T>>` para estado compartido
- Manejo de JSON en HTTP
- Estructuración de proyecto grande
- Testing de integración

#### Arquitectura:

**Presentación** (`presentacion/servidor.rs`):
```rust
// Endpoints HTTP
// GET /notas
// GET /notas/:id
// POST /notas
// PUT /notas/:id
// DELETE /notas/:id

use actix_web::{web, App, HttpServer};
```

**Lógica** (`logica/servicio.rs`):
```rust
use std::sync::{Arc, Mutex};

pub struct Nota {
    id: u32,
    titulo: String,
    contenido: String,
    fecha_creacion: String,
}

pub struct ServicioNotas {
    repositorio: Arc<Mutex<dyn RepositorioNotas + Send>>,
}

impl ServicioNotas {
    pub async fn crear_nota(&self, nota: Nota) -> Result<u32, String>
    pub async fn obtener_nota(&self, id: u32) -> Result<Nota, String>
    pub async fn listar_todas(&self) -> Result<Vec<Nota>, String>
}
```

**Datos** (`datos/repositorio_memoria.rs`):
```rust
use std::collections::HashMap;

pub struct RepositorioMemoria {
    notas: HashMap<u32, Nota>,
    siguiente_id: u32,
}

// Implementar trait con Mutex interno
```

#### Errores esperados:
- ❌ `future cannot be sent between threads safely`
- ❌ Deadlocks con múltiples locks de Mutex
- ❌ Olvidar `.await` en funciones async

#### Dependencias:
```toml
[dependencies]
actix-web = "4"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

### Proyecto 3.2: CLI de Gestión de Archivos (Concurrente)
**Duración estimada**: 7-10 días

#### 📖 Teoría a Estudiar ANTES de Programar

**1. Threads (Hilos)**
- **Concepto**: ejecución paralela real (múltiples CPU cores)
- **Sintaxis**: `thread::spawn(|| { /* código */ })`
- **Join**: esperar que thread termine
- **vs async**: threads = paralelismo, async = concurrencia
- **Lecturas**: Rust Book Capítulo 16.1
- **Tiempo estimado**: 2-3 horas

**2. Message Passing (mpsc)**
- **Concepto**: comunicación entre threads vía canales
- **mpsc**: multiple producer, single consumer
- **Sintaxis**:
```rust
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || tx.send(42).unwrap());
  let valor = rx.recv().unwrap();
```
- **Ownership**: valores se mueven al canal
- **Lecturas**: Rust Book Capítulo 16.2
- **Tiempo estimado**: 2-3 horas

**3. Arc vs Rc**
- **Rc**: reference counting (single-threaded)
- **Arc**: atomic reference counting (multi-threaded)
- **Cuándo usar cada uno**:
  - Rc: dentro de un thread
  - Arc: compartir entre threads
- **Overhead**: Arc es más costoso (operaciones atómicas)
- **Lecturas**: Rust Book Capítulo 15.4 y 16.3
- **Tiempo estimado**: 1-2 horas

**4. Filesystem Operations**
- **std::fs**: operaciones de archivos
  - `read_to_string()`, `write()`
  - `create_dir_all()`, `remove_dir_all()`
  - `metadata()`: info de archivo
- **std::path::PathBuf**: rutas multiplataforma
- **Lecturas**: Std docs (fs module)
- **Tiempo estimado**: 2 horas

**5. Recursión y Directorios**
- **Concepto**: recorrer árbol de directorios
- **Librería walkdir**: simplifica recorrido
- **Patrón**:
```rust
  for entry in WalkDir::new(path) {
      let entry = entry?;
      // procesar archivo/directorio
  }
```
- **Lecturas**: Docs de walkdir
- **Tiempo estimado**: 1 hora

**6. Clap - Argument Parsing**
- **Concepto**: parsear argumentos CLI robustamente
- **Derive API**: usar atributos
```rust
  #[derive(Parser)]
  struct Args {
      #[arg(short, long)]
      archivo: PathBuf,
  }
```
- **Subcomandos**: comandos anidados
- **Lecturas**: Documentación de Clap
- **Tiempo estimado**: 2-3 horas

**7. Hashing y Checksums**
- **Concepto**: identificar archivos duplicados
- **SHA-256**: algoritmo hash criptográfico
- **Uso**:
```rust
  use sha2::{Sha256, Digest};
  let mut hasher = Sha256::new();
  hasher.update(contenido);
  let hash = hasher.finalize();
```
- **Lecturas**: Docs de sha2
- **Tiempo estimado**: 1 hora

**8. Error Handling Avanzado**
- **Propagación con ?**: simplificar código
- **From trait**: conversión automática de errores
- **Custom errors**: crear tipos de error propios
- **thiserror**: macro para definir errores
- **Lecturas**: Rust Book Capítulo 9 (profundizar)
- **Tiempo estimado**: 2-3 horas

**Total teoría Proyecto 3.2**: ~13-18 horas

#### Conceptos nuevos:
- Threads y channels (`mpsc`)
- `Arc` vs `Rc`
- Manejo de filesystem
- Parsing de argumentos con `clap`
- Error handling robusto

#### Arquitectura:

**Presentación**:
```rust
// CLI tipo "grep" o "find"
// cargo run -- buscar "texto" --directorio ./src --extension rs
// cargo run -- duplicados --directorio ./
// cargo run -- comprimir --origen ./docs --destino ./backup.zip
```

**Lógica**:
```rust
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

pub struct Buscador {
    directorio: PathBuf,
    extension: Option<String>,
}

impl Buscador {
    pub fn buscar_paralelo(&self, texto: &str) -> Vec<PathBuf> {
        // Spawn threads, usar channels para resultados
    }
}

pub struct AnalizadorDuplicados {
    // Usa hashing de contenidos
}
```

**Datos**:
```rust
use std::fs;
use std::io::{self, Read};

pub fn leer_archivo(path: &PathBuf) -> io::Result<String>
pub fn hash_archivo(path: &PathBuf) -> io::Result<String>
pub fn listar_recursivo(dir: &PathBuf) -> io::Result<Vec<PathBuf>>
```

#### Errores esperados:
- ❌ Data races en variables compartidas
- ❌ Channels cerrados prematuramente
- ❌ `Send` trait not implemented

#### Dependencias:
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
walkdir = "2"
sha2 = "0.10"
```

---

## 🏆 PROYECTO FINAL: Sistema Completo MVC

### "Gestor de Proyectos Técnicos"
**Duración estimada**: 2-3 semanas

#### 📖 Teoría a Consolidar (Repaso y Nuevos Conceptos)

**1. Arquitectura de Software**
- **Capas vs MVC**: similitudes y diferencias
- **Separación de responsabilidades**
- **Dependency injection**: pasar dependencias
- **Lecturas**: Clean Architecture (libro/artículos)
- **Tiempo estimado**: 3-4 horas

**2. Testing Comprehensivo**
- **Tipos de tests**:
  - Unitarios: función individual
  - Integración: múltiples módulos
  - End-to-end: sistema completo
- **Mocks y Stubs**: simular dependencias
- **Coverage**: medir cobertura con `tarpaulin`
- **Lecturas**: Rust Book Capítulo 11 (completo)
- **Tiempo estimado**: 4-5 horas

**3. Autenticación Básica**
- **JWT (JSON Web Tokens)**: concepto
- **Hashing de contraseñas**: bcrypt/argon2
- **Middleware**: validación en cada request
- **Lecturas**: Docs de jsonwebtoken
- **Tiempo estimado**: 3-4 horas

**4. Relaciones en Base de Datos**
- **Foreign keys**: relacionar tablas
- **JOINs**: consultas relacionadas
- **Normalización**: evitar duplicación
- **Migrations**: evolucionar schema
- **Lecturas**: Tutorial SQL avanzado
- **Tiempo estimado**: 3-4 horas

**5. Cliente HTTP (reqwest)**
- **Concepto**: hacer requests desde CLI
- **async client**: no bloquear
- **Deserialización**: JSON a structs
- **Lecturas**: Docs de reqwest
- **Tiempo estimado**: 2 horas

**6. Logging y Debugging**
- **log crate**: diferentes niveles (error, warn, info, debug)
- **env_logger**: configurar desde variables
- **Debug efectivo**: usar logs vs println
- **Lecturas**: Docs de log
- **Tiempo estimado**: 1-2 horas

**7. Configuración con archivos**
- **TOML/YAML**: formatos de config
- **Variables de entorno**: config sensible
- **config crate**: cargar configuración
- **Lecturas**: Docs de config
- **Tiempo estimado**: 1-2 horas

**Total teoría Proyecto Final**: ~17-25 horas

#### Integra TODOS los conceptos:

**Características**:
- API REST (async)
- Base de datos SQLite
- Autenticación básica
- Múltiples entidades relacionadas (Proyectos, Tareas, Usuarios)
- Tests completos
- CLI cliente que consume la API

**Arquitectura**:
```
proyecto_final/
├── src/
│   ├── main.rs
│   ├── presentacion/
│   │   ├── mod.rs
│   │   ├── servidor.rs      # Actix-web
│   │   └── cli.rs           # Cliente CLI
│   ├── logica/
│   │   ├── mod.rs
│   │   ├── proyectos.rs
│   │   ├── tareas.rs
│   │   └── usuarios.rs
│   ├── datos/
│   │   ├── mod.rs
│   │   ├── repositorio.rs   # Trait
│   │   └── sqlite.rs        # Implementación
│   └── lib.rs
├── tests/
│   └── integracion.rs
└── Cargo.toml
```

---

## 📊 Cronograma Sugerido con Teoría
```
Semana 1:
  - Lunes-Martes: Teoría Proyecto 1.1 (10-12h)
  - Miércoles-Viernes: Programar Proyecto 1.1 (2-3 días)
  
Semana 2:
  - Lunes-Miércoles: Teoría Proyecto 1.2 (11-13h)
  - Jueves-Domingo: Programar Proyecto 1.2 (3-4 días)

Semana 3:
  - Lunes-Miércoles: Teoría Proyecto 2.1 (14-17h)
  - Jueves-Domingo: Programar Proyecto 2.1 (3-4 días)

Semana 4-5:
  - Lunes-Miércoles: Teoría Proyecto 2.2 (14-18h)
  - Jueves-Martes: Programar Proyecto 2.2 (5-6 días)

Semana 6-7:
  - Lunes-Jueves: Teoría Proyecto 3.1 (17-23h)
  - Viernes-Miércoles: Programar Proyecto 3.1 (6-8 días)

Semana 8-9:
  - Lunes-Jueves: Teoría Proyecto 3.2 (13-18h)
  - Viernes-Lunes: Programar Proyecto 3.2 (7-10 días)

Semana 10-12:
  - Semana 10: Teoría Proyecto Final (17-25h)
  - Semanas 11-12: Programar Proyecto Final (2-3 semanas)
```

**Total teoría**: ~96-134 horas  
**Total programación**: ~10 semanas

---

## 💡 Estrategia de Estudio Teoría

### Por Cada Concepto:

1. **Lee la teoría** (tiempo estimado)
2. **Escribe notas propias** con tus palabras
3. **Busca ejemplos adicionales** (YouTube, blogs)
4. **Prueba en Rust Playground** (play.rust-lang.org)
5. **Aplica en el proyecto** inmediatamente

### Recursos por Nivel:

**Nivel 1**: The Rust Book es suficiente  
**Nivel 2**: Rust Book + Rust By Example  
**Nivel 3**: Async Book + documentación de crates

---

## 🛠️ Herramientas Esenciales
```bash
# Compilar
cargo build

# Ejecutar
cargo run

# Tests
cargo test

# Linter (muy útil para aprender)
cargo clippy

# Formatear código
cargo fmt

# Ver errores detallados
RUST_BACKTRACE=1 cargo run

# Documentación
cargo doc --open

# Benchmark (Nivel 3)
cargo bench
```

---

## 📚 Recursos Ordenados por Prioridad

### Esenciales (Leer SIEMPRE):
1. **The Rust Book**: https://doc.rust-lang.org/book/
2. **Rust By Example**: https://doc.rust-lang.org/rust-by-example/

### Intermedios (Nivel 2+):
3. **Rust Async Book**: https://rust-lang.github.io/async-book/
4. **Rustlings** (ejercicios): https://github.com/rust-lang/rustlings

### Avanzados (Nivel 3):
5. **Tokio Tutorial**: https://tokio.rs/tokio/tutorial
6. **Actix-web Guide**: https://actix.rs/docs/

### Referencia Rápida:
7. **Cheat Sheet**: https://cheats.rs/
8. **Docs.rs**: https://docs.rs/ (docs de cualquier crate)

---

## ✅ Checklist de Dominio con Teoría

**Nivel 1**:
- [ ] Leí capítulos 1-9 del Rust Book
- [ ] Entiendo ownership y borrowing sin ayuda
- [ ] Puedo usar `Result` y `Option` correctamente
- [ ] Sé cuándo usar `String` vs `&str`
- [ ] Completé Proyecto 1.1 y 1.2

**Nivel 2**:
- [ ] Leí capítulos 10-11 del Rust Book
- [ ] Creo traits y los implemento sin errores
- [ ] Uso genéricos con trait bounds
- [ ] Escribo tests unitarios útiles
- [ ] Entiendo lifetimes básicos
- [ ] Completé Proyecto 2.1 y 2.2

**Nivel 3**:
- [ ] Leí Async Book completo
- [ ] Manejo concurrencia sin data races
- [ ] Entiendo cuándo usar `Arc`, `Rc`, `Box`
- [ ] Diseño APIs async eficientes
- [ ] Escribo tests de integración
- [ ] Completé Proyecto 3.1 y 3.2

**Proyecto Final**:
- [ ] Integré todos los conceptos
- [ ] Sistema funcional end-to-end
- [ ] Coverage >70% en tests
- [ ] Documentación completa
- [ ] Listo para proyecto real

---

## 🎓 Metodología de Aprendizaje Sugerida

### Ciclo por Proyecto:
```
1. TEORÍA (40% del tiempo)
   ↓
2. EXPERIMENTACIÓN (20% del tiempo)
   - Rust Playground
   - Mini-ejemplos
   ↓
3. IMPLEMENTACIÓN (30% del tiempo)
   - Proyecto completo
   ↓
4. REVISIÓN (10% del tiempo)
   - Refactoring
   - Tests
   - Documentación
```

### Regla de Oro:
**"No código sin teoría, no teoría sin código"**

Si te trabas >2 horas: vuelve a la teoría.  
Si la teoría no tiene sentido: escribe código.

---

**Creado**: Diciembre 2025  
**Enfoque**: Teoría sólida + Práctica con arquitectura en capas  
**Filosofía**: "Entender primero, programar después"  
**Meta**: Convertirte en Rustacean competente en 10-12 semanas
