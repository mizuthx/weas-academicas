# 📊 GUÍA PARA PRESENTACIÓN - PROYECTO MATEMÁTICAS

## Sistema de Optimización de Inventarios

---

## 📋 INFORMACIÓN DEL PROYECTO

- **Nombre**: Sistema de optimización de inventarios
- **Tema**: Uso de matrices en Python
- **Integrantes**: Tomas Serey, Jorge Adonis, Mariano Provoste
- **Evaluación**: Sumativa 04 (40%)
- **Duración video**: 8-15 minutos

---

## 🎯 ESTRUCTURA DE LA PRESENTACIÓN (11 DIAPOSITIVAS)

### **DIAPOSITIVA 1: PORTADA**
**Elementos obligatorios (7):**
1. Título: "Sistema de Optimización de Inventarios"
2. Subtítulo: "Uso de matrices en Python"
3. Integrantes: Tomas Serey, Jorge Adonis, Mariano Provoste
4. Profesor: [Nombre del profesor]
5. Asignatura: [Nombre de la asignatura]
6. Sección: [Tu sección]
7. Carrera: Ingeniería en Ciberseguridad

**Diseño sugerido:**
- Fondo profesional
- Tipografía legible (Arial, Calibri)
- Logo institucional (si aplica)

---

### **DIAPOSITIVA 2: CONTEXTO Y PROBLEMÁTICA**
**Debe incluir 4 elementos:**

1. **Contexto**:
   - Describir la situación actual de gestión de inventarios en tiendas multi-sucursal
   - Ejemplo: "Las empresas con múltiples sucursales enfrentan desafíos al..."

2. **Declaración del problema**:
   - "La falta de control automatizado genera desbalances entre sucursales"

3. **Consecuencias**:
   - Sobrestock en algunas sucursales
   - Desabastecimiento en otras
   - Pérdidas económicas

4. **Solución propuesta**:
   - "Sistema automatizado que utiliza matrices para optimizar inventarios"

**Formato**: Párrafos cortos o viñetas (máximo 4-5 líneas por elemento)

---

### **DIAPOSITIVA 3: OBJETIVO Y PREGUNTAS DE INVESTIGACIÓN**

**Objetivo General:**
```
Crear un sistema capaz de optimizar los inventarios de distintas 
tiendas aplicando matrices para mejorar el control de productos.
```

**4 Preguntas de Investigación:**
1. ¿Cuál es el valor total del inventario por sucursal?
2. ¿Cómo redistribuir inventario para equilibrar las sucursales?
3. ¿De qué sucursal despacho para minimizar desbalance?
4. ¿Cuánto debo pedir para cada sucursal si quiero 2 semanas de stock ideal?

---

### **DIAPOSITIVA 4: METODOLOGÍA - CARTA GANTT**

**Crear tabla con:**

| Tarea | Responsable | Fecha Inicio | Fecha Término | Estado |
|-------|-------------|--------------|---------------|--------|
| Investigación teórica | Equipo | [fecha] | [fecha] | ✅ |
| Diseño del sistema | Tomas | [fecha] | [fecha] | ✅ |
| Desarrollo en Python | Jorge | [fecha] | [fecha] | ✅ |
| Pruebas y validación | Mariano | [fecha] | [fecha] | ✅ |
| Documentación | Equipo | [fecha] | [fecha] | ✅ |
| Presentación final | Equipo | [fecha] | [fecha] | 🔄 |

**Alternativa**: Diagrama Gantt visual con barras de tiempo

---

### **DIAPOSITIVAS 5-6: JUSTIFICACIÓN DEL PRODUCTO**

**IMPORTANTE**: Cada respuesta debe tener **mínimo 1 fuente citada**

#### **Diapositiva 5 (Preguntas 1 y 2):**

**Pregunta 1: ¿Cuál es el valor total del inventario por sucursal?**
- Respuesta: "Mediante suma de matrices podemos calcular el valor total multiplicando cantidad × precio unitario..."
- **Fuente**: [Autor. (Año). Título. https://ejemplo.com]

**Pregunta 2: ¿Cómo redistribuir inventario?**
- Respuesta: "Utilizando resta de matrices identificamos diferencias entre sucursales..."
- **Fuente**: [Autor. (Año). Título. https://ejemplo.com]

#### **Diapositiva 6 (Preguntas 3 y 4):**

**Pregunta 3: ¿De qué sucursal despachar?**
- Respuesta: "Comparando matrices de inventario actual vs ideal..."
- **Fuente**: [Autor. (Año). Título. https://ejemplo.com]

**Pregunta 4: ¿Cuánto pedir para 2 semanas?**
- Respuesta: "Multiplicación de matrices: demanda diaria × 14 días..."
- **Fuente**: [Autor. (Año). Título. https://ejemplo.com]

---

### **DIAPOSITIVA 7: EVIDENCIA DEL PRODUCTO**

**Debe incluir:**

1. **Capturas de pantalla del código**
   - Mostrar funciones principales
   - Ejemplo de matrices utilizadas

2. **Datos de ejemplo**
   ```
   Sucursal A: [100, 50, 30]
   Sucursal B: [80, 60, 40]
   Sucursal C: [90, 55, 35]
   ```

3. **Resultados de operaciones**
   - Inventario total
   - Redistribuciones sugeridas
   - Stock a pedir

4. **Simulación en vivo** (durante el video)
   - Ejecutar el programa
   - Mostrar inputs y outputs

---

### **DIAPOSITIVA 8: NOCIONES MATEMÁTICAS**

**Crear esquema visual:**

```
OPERACIONES CON MATRICES APLICADAS

├─ SUMA DE MATRICES
│  └─ Uso: Calcular inventario total por producto
│  └─ Ejemplo: Matriz_A + Matriz_B = Total
│
├─ RESTA DE MATRICES  
│  └─ Uso: Identificar diferencias entre sucursales
│  └─ Ejemplo: Stock_Ideal - Stock_Actual = Faltante
│
├─ MULTIPLICACIÓN ESCALAR
│  └─ Uso: Calcular stock para N semanas
│  └─ Ejemplo: Demanda_Diaria × 14 = Stock_2_Semanas
│
└─ TRANSPUESTA
   └─ Uso: Reorganizar datos (productos × sucursales)
   └─ Ejemplo: Matriz^T
```

**Incluir 1 ejemplo numérico:**
```
Ejemplo: Suma de inventarios
Sucursal A: [10, 20, 30]
Sucursal B: [15, 25, 35]
─────────────────────────
Total:      [25, 45, 65]
```

---

### **DIAPOSITIVA 9: CONCLUSIONES Y REFLEXIONES**

**Estructura en 3 párrafos:**

1. **Cumplimiento del objetivo**:
   - "El proyecto logró desarrollar un sistema que optimiza inventarios mediante matrices..."
   - "Se cumplió el objetivo de crear una herramienta automatizada..."

2. **Resolución de la problemática**:
   - "La implementación resuelve el problema de desbalance entre sucursales..."
   - "Reduce tiempo de gestión manual de inventarios..."

3. **Proyección futura**:
   - "El sistema puede expandirse para incluir predicción de demanda..."
   - "Aplicable a otros rubros como retail, farmacia, etc."

---

### **DIAPOSITIVA 10: MAPA DE IMPACTO TFL**

**Elementos requeridos:**

1. **Malla curricular** (imagen o diagrama simplificado)

2. **Flechas desde "Matemáticas/Funciones" hacia:**
   - **Programación**: "Aplicación de Python y estructuras de datos"
   - **Base de Datos**: "Gestión y consulta de información estructurada"
   - **Gestión de Proyectos**: "Resolución de problemas empresariales"
   - **Algoritmos**: "Optimización y lógica computacional"

3. **Competencias desarrolladas:**
   - ✅ Pensamiento lógico-matemático
   - ✅ Programación en Python
   - ✅ Análisis de problemas reales
   - ✅ Trabajo colaborativo
   - ✅ Diseño de soluciones tecnológicas

4. **Elementos conceptuales aplicables:**
   - Matrices y operaciones
   - Algoritmos de optimización
   - Modelamiento de problemas

---

### **DIAPOSITIVA 11: BIBLIOGRAFÍA**

**Formato APA (o según indique el profesor):**

```
[1] Apellido, N. (Año). Título del artículo. Nombre de la revista.
    https://doi.org/xxxxx

[2] Apellido, N. (Año). Título del libro. Editorial.

[3] Organización. (Año). Título del documento. Recuperado de
    https://www.ejemplo.com

[4] Apellido, N. (Año). Título del paper. Conferencia/Congreso.
```

**Mínimo 4 fuentes** (1 por pregunta de investigación)

**Tipos de fuentes aceptadas:**
- ✅ Papers académicos
- ✅ Libros especializados
- ✅ Sitios web institucionales (.edu, .gov, .org)
- ✅ Documentación técnica oficial
- ❌ Wikipedia (como única fuente)
- ❌ Blogs personales sin respaldo

---

## 🎥 REQUISITOS DEL VIDEO

### **Duración:**
- **Mínimo**: 8 minutos
- **Máximo**: 15 minutos
- ⚠️ Fuera de este rango = penalización

### **Formato de grabación:**
**Opción 1 - Videollamada** (Recomendado):
- Plataformas: Zoom, Google Meet, Microsoft Teams
- Grabar reunión con pantalla compartida
- Todos los integrantes visibles en cámara

**Opción 2 - PowerPoint**:
- Grabar presentación desde PPT (función "Grabar presentación de diapositivas")
- Insertar video de webcam

**Opción 3 - Presencial**:
- Grabar con cámara/celular en sala
- Asegurar buena calidad de audio e imagen
- Proyectar diapositivas en pantalla

### **Participación:**
- ✅ **Todos deben aparecer en cámara** (no solo voz de fondo)
- ✅ Tiempo equitativo por integrante (3-5 minutos c/u)
- ✅ Presentar 3-4 diapositivas por persona

**Distribución sugerida:**
- **Tomas**: Diapositivas 1-3 (Intro, contexto, objetivo)
- **Jorge**: Diapositivas 4-6 (Metodología, justificación)
- **Mariano**: Diapositivas 7-9 (Producto, matemáticas, conclusiones)
- **Todos**: Diapositivas 10-11 (Impacto, bibliografía)

### **Lenguaje y presentación:**
- ✅ Lenguaje **formal** (evitar muletillas: "ehh", "este", "o sea")
- ✅ **No leer** las diapositivas completas (solo citas o definiciones)
- ✅ Elocuencia: fluidez, persuasión, elegancia
- ✅ Contacto visual con la cámara
- ✅ Vestimenta apropiada (formal o business casual)

### **Edición del video:**
- ✅ Cortar silencios prolongados (>5 seg)
- ✅ Eliminar conversaciones paralelas
- ✅ Transiciones suaves entre diapositivas
- ✅ Audio limpio y claro
- ❌ No incluir pantallas de espera o carga

**Software recomendado:**
- Edición simple: Windows Video Editor, iMovie
- Edición avanzada: DaVinci Resolve (gratis), Camtasia

---

## ✅ CHECKLIST ANTES DE ENTREGAR

### **Contenido:**
- [ ] 11 diapositivas mínimo (estructura completa)
- [ ] 7 elementos en portada
- [ ] 4 elementos en contexto/problemática
- [ ] 4 preguntas respondidas con fuentes
- [ ] Mínimo 4 fuentes bibliográficas citadas
- [ ] Carta Gantt o bitácora presente
- [ ] Simulación del producto mostrada
- [ ] Esquema de nociones matemáticas completo
- [ ] Mapa de impacto TFL con malla curricular
- [ ] Conclusiones relacionan todo el proyecto

### **Formato:**
- [ ] Orden correcto de diapositivas
- [ ] Cero errores ortográficos
- [ ] Diseño coherente y profesional
- [ ] Fuentes legibles (mínimo 18pt)
- [ ] Imágenes de buena calidad

### **Video:**
- [ ] Duración: 8-15 minutos
- [ ] Todos los integrantes aparecen en cámara
- [ ] Participación equitativa
- [ ] Lenguaje formal
- [ ] Audio claro
- [ ] Video editado (sin errores)
- [ ] Formato de entrega según indicaciones del docente

---

## 📊 CÁLCULO DE NOTA

### **Tu situación actual:**

| Indicador | Ponderación | Puntaje Max | Tu puntaje esperado |
|-----------|-------------|-------------|---------------------|
| 1. Entrega Etapa 1 | 5% | 7 | 7 ✅ |
| 2. Entrega Etapa 2 | 5% | 7 | **1 ❌** |
| 3. Presentación | 5% | 7 | 7 |
| 4. Contexto | 10% | 7 | 7 |
| 5. Objetivo | 5% | 7 | 7 |
| 6. Preguntas | 10% | 7 | 7 |
| 7. Metodología | 5% | 7 | 7 |
| 8. Justificación | 10% | 7 | 7 |
| 9. Fuentes | 5% | 7 | 7 |
| 10. Producto | 10% | 7 | 7 |
| 11. Matemáticas | 10% | 7 | 7 |
| 12. Conclusiones | 5% | 7 | 7 |
| 13. Impacto TFL | 5% | 7 | 7 |
| 14. Audiovisual | 5% | 7 | 7 |
| 15. Formato | 5% | 7 | 7 |
| **TOTAL** | **100%** | **105** | **99** |

### **Cálculo:**
- Puntaje: 99/105 = **94.3%**
- Con 60% de exigencia: **Nota 6.1**

**Fórmula**: `Nota = 1 + 6 × (% - 0.6) / 0.4`

### **Escala de referencia:**
| Puntaje | % | Nota |
|---------|---|------|
| 105 pts | 100% | 7.0 |
| 99 pts | 94% | **6.1** (tu nota esperada) |
| 84 pts | 80% | 5.0 |
| 63 pts | 60% | 4.0 |

---

## ⚠️ ERRORES COMUNES A EVITAR

### **En contenido:**
- ❌ Olvidar algún elemento de la portada (7 obligatorios)
- ❌ No citar fuentes en las respuestas
- ❌ Bibliografía incompleta (<4 fuentes)
- ❌ No mostrar simulación del producto
- ❌ Errores ortográficos

### **En el video:**
- ❌ Duración menor a 8 o mayor a 15 minutos
- ❌ Integrantes solo con voz (deben aparecer)
- ❌ Leer todas las diapositivas
- ❌ Conversaciones paralelas sin editar
- ❌ Audio de mala calidad

### **En formato:**
- ❌ Desorden en las diapositivas
- ❌ Tipografía muy pequeña
- ❌ Diseño poco profesional
- ❌ Entregar en formato incorrecto

---

## 🚀 RECOMENDACIONES FINALES

### **Para maximizar tu nota:**

1. **Compensa la falta de Etapa 2**:
   - Asegúrate que TODOS los otros 14 indicadores estén perfectos
   - Especial énfasis en fuentes y justificación

2. **Practica la presentación**:
   - Ensayar al menos 2 veces completas
   - Cronometrar para no exceder 15 minutos
   - Verificar transiciones entre presentadores

3. **Revisa múltiples veces**:
   - Ortografía (usar corrector)
   - Coherencia entre diapositivas
   - Funcionamiento del código/simulación

4. **Calidad del video**:
   - Grabar en lugar silencioso
   - Buena iluminación
   - Conexión estable si es por videollamada

5. **Respaldo**:
   - Guardar múltiples versiones del PPT
   - Tener el video en 2 lugares (nube + local)
   - Probar apertura antes de entregar

---

## 📞 CONSULTAS

Si tienes dudas:
1. Consultar con el profesor **antes** de la fecha límite
2. Revisar esta guía completamente
3. Comparar con la rúbrica oficial

---

## 📅 RECORDATORIO DE FECHAS

**Fecha de entrega**: [Completar según tu profesor]

⏰ **Recomendación**: Terminar 2-3 días antes para tener margen de corrección

---

**¡Éxito en tu presentación! 🎓**

---

*Última actualización: Diciembre 2024*
