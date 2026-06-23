# COLDCASE

<img width="1672" height="941" alt="cc" src="https://github.com/user-attachments/assets/6cf81c01-a80c-4ec1-8bb2-af2d641e14b7" />

**COLDCASE** es una mesa de triage forense local construida en Rust para organizar casos, capturar snapshots, generar líneas temporales, calcular hashes, detectar artefactos sospechosos y producir reportes defensivos.

Creado por **xtr4ng3**.

---

## Propósito

Cuando una máquina empieza a comportarse de forma extraña, el primer paso no debería ser borrar, limpiar o formatear sin entender qué ocurrió.

El primer paso es observar.

**COLDCASE** fue diseñado para reunir evidencia local, ordenar señales importantes y generar reportes claros. Su objetivo es ayudar a usuarios técnicos, administradores y analistas a responder preguntas básicas:

- qué archivos aparecieron,
- qué archivos cambiaron,
- qué rutas merecen revisión,
- qué hashes se generaron,
- qué scripts o ejecutables existen en zonas sensibles,
- qué eventos pueden formar parte de una línea temporal,
- qué elementos conviene documentar antes de tomar decisiones.

COLDCASE no elimina archivos.  
COLDCASE no modifica el sistema.  
COLDCASE no sube evidencia a internet.  
COLDCASE organiza información local para revisión defensiva.

---

## Características principales

- CLI en Rust.
- Base de datos SQLite por caso.
- Espacios de trabajo independientes.
- Captura de snapshot del sistema.
- Escaneo de carpetas.
- Cálculo de SHA-256.
- Estimación de entropía.
- Reglas defensivas de triage.
- Detección de scripts.
- Detección de ejecutables.
- Detección de archivos comprimidos.
- Detección de rutas sensibles.
- Detección de nombres de alto riesgo.
- Generación de línea temporal.
- Reporte HTML.
- Reporte JSON.
- Reporte SARIF.
- Dashboard estático local.
- Workflow de GitHub Actions.
- Script de build para Windows.

---

## Arquitectura

```text
case workspace
  -> SQLite database
  -> snapshot collector
  -> file scanner
  -> rule evaluator
  -> timeline builder
  -> report generator
  -> static dashboard
```

Componentes principales:

```text
src/
├─ main.rs        -> entrada CLI
├─ case.rs        -> creación y estado de casos
├─ collectors.rs  -> snapshots del sistema
├─ scanner.rs     -> escaneo de carpetas
├─ hashing.rs     -> SHA-256 y entropía
├─ rules.rs       -> reglas de triage
├─ db.rs          -> almacenamiento SQLite
├─ report.rs      -> HTML / JSON / SARIF
├─ models.rs      -> modelos de datos
└─ util.rs        -> utilidades internas
```

---

## Comandos

Crear un caso:

```bash
coldcase new cases/my-case --title "Suspicious download review"
```

Capturar snapshot local:

```bash
coldcase snapshot cases/my-case
```

Escanear una carpeta:

```bash
coldcase scan cases/my-case C:\Users\User\Downloads
```

Generar línea temporal:

```bash
coldcase timeline cases/my-case
```

Generar reportes:

```bash
coldcase report cases/my-case
```

Ver estado del caso:

```bash
coldcase status cases/my-case
```

Crear archivo de reglas por defecto:

```bash
coldcase rules coldcase.rules
```

---

## Flujo recomendado

```bash
coldcase new cases/test-case --title "PC triage"
coldcase snapshot cases/test-case
coldcase scan cases/test-case C:\Users\User\Downloads
coldcase scan cases/test-case C:\Users\User\AppData\Roaming
coldcase timeline cases/test-case
coldcase report cases/test-case
```

Después de generar el reporte, abrir el HTML dentro de:

```text
cases/test-case/reports/
```

---

## Estructura de un caso

```text
my-case/
├─ CASE.md
├─ coldcase.db
├─ evidence/
├─ exports/
└─ reports/
   ├─ coldcase_<timestamp>.html
   ├─ coldcase_<timestamp>.json
   └─ coldcase_<timestamp>.sarif
```

Cada caso tiene su propia base SQLite y sus propios reportes.

---

## Qué detecta COLDCASE

COLDCASE etiqueta y documenta artefactos como:

- scripts,
- ejecutables,
- archivos comprimidos,
- archivos en rutas sensibles,
- artefactos de alta entropía,
- nombres de archivo con palabras de riesgo,
- entradas de inicio sospechosas detectadas en snapshots,
- elementos útiles para una línea temporal.

Ejemplos de rutas sensibles:

```text
AppData
Temp
Downloads
Descargas
Public
```

Ejemplos de extensiones revisadas:

```text
ps1, vbs, js, jse, wsf, hta, bat, cmd, scr, pif, lnk
exe, dll, sys, msi
zip, rar, 7z, iso, img
```

---

## Reportes

COLDCASE genera tres formatos:

```text
HTML   -> lectura humana
JSON   -> integración / dashboard / archivo estructurado
SARIF  -> revisión compatible con flujos de seguridad
```

El reporte HTML incluye:

- veredicto,
- score,
- hallazgos,
- artefactos,
- hashes,
- línea temporal,
- snapshots,
- recomendaciones.

---

## Dashboard

El dashboard es local y estático:

```text
dashboard/index.html
```

Puede cargar un reporte JSON generado por COLDCASE para revisar:

- resumen,
- hallazgos,
- artefactos,
- timeline.

No requiere servidor externo.

---

## Reglas

El archivo base de reglas está en:

```text
rules/coldcase.rules
```

También puede generarse con:

```bash
coldcase rules coldcase.rules
```

Las reglas iniciales son simples y defensivas. Están pensadas para triage local, no para emitir condenas automáticas.

---

## Compilación

Requiere Rust.

```bash
cargo build --release
```

En Windows se puede usar:

```bat
build_windows\BUILD_RELEASE.bat
```

El binario queda en:

```text
target/release/coldcase
```

En Windows:

```text
target\release\coldcase.exe
```

---

## Instalación rápida en Windows

1. Instalar Rust desde `rustup`.
2. Abrir una terminal en la carpeta del proyecto.
3. Ejecutar:

```bat
build_windows\BUILD_RELEASE.bat
```

4. Usar el binario generado dentro de `CLIENTE_PORTABLE`.

---

## Seguridad

COLDCASE no realiza acciones destructivas.

No hace:

- borrado de archivos,
- cuarentena automática,
- cierre de procesos,
- desactivación de entradas de inicio,
- cambios de registro,
- subida de evidencia,
- explotación,
- persistencia,
- modificación silenciosa del sistema.

COLDCASE recolecta, organiza y reporta.

---

## Uso responsable

Los hallazgos de COLDCASE son puntos de revisión.  
Un hallazgo alto no significa por sí solo que un archivo sea malicioso. Significa que merece análisis manual.

El contexto importa:

- origen del archivo,
- fecha de creación,
- ubicación,
- firma,
- hash,
- comportamiento observado,
- relación con otros eventos.

---

## Estado del proyecto

COLDCASE v1.0 es una base profesional para triage local.

Áreas futuras posibles:

- más collectors,
- parsing de eventos más avanzado,
- exportación CSV,
- reglas configurables más completas,
- análisis de accesos directos `.lnk`,
- comparación entre snapshots,
- empaquetado portable más avanzado,
- integración opcional con dashboards externos.

---

## Licencia

<img width="384" height="384" alt="giphy (4)" src="https://github.com/user-attachments/assets/18d64b9e-fba8-493e-8462-f6722e0e64b7" />

**xtr4ng3**

MIT.
