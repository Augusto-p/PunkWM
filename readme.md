| Módulo               | Función                                                       |
| -------------------- | ------------------------------------------------------------- |
| **WM Core**          | Manejo de ventanas: **abrir**, **cerrar**, maximizar, mover, tiling   |
| **Layout Manager**   | **Definir layouts**: **tile**, **max**, **spiral**, **dwindle**, **magnifier**, **fair** |
| **Widget Manager**   | Controlar widgets HTML/CSS y su comunicación con Core         |
| **Input Handler**    | Mapear teclas, atajos (ej. Alt+Q, Super+... )                 |
| **Persistence**      | Guardar layouts y posición de widgets en `persistence.bin`    |
| **IPC / Messaging**  | Comunicación Core ↔ Widgets (WebSocket / JSON / IPC)          |
| **Rendering Engine** | WebView o motor HTML/CSS para render de widgets               |
| **Event Loop**       | Captura eventos X11/Wayland y los distribuye a Core y Widgets |



## 🔧 Requisitos por GPU

### 🟩 NVIDIA

* GPU NVIDIA
* Drivers propietarios instalados
* Herramienta incluida con el driver:

  * `nvidia-smi`

Verificación:

```bash
command -v nvidia-smi
```

---

### 🟦 Intel (iGPU)

* GPU Intel integrada
* Kernel con soporte i915
* Paquete:

  ```bash
  intel-gpu-tools
  ```
* Dependencia adicional:

  ```bash
  jq
  ```

Instalación:

```bash
sudo apt install intel-gpu-tools jq
```

Verificación:

```bash
command -v intel_gpu_top
```

---

### 🟥 AMD

* GPU AMD (amdgpu)
* Paquete:

  ```bash
  radeontop
  ```

Instalación:

```bash
sudo apt install radeontop
```

Verificación:

```bash
command -v radeontop
```

---

## 🧪 Salida

* Devuelve **un número**
* Sin `%`
* Ejemplo:

```text
42
```

---

## ⚠️ Notas

* No funciona en sistemas sin GPU
* Requiere permisos normales de usuario
* Compatible con **X11 y Wayland**
* Ideal para scripts, Rust, paneles y dashboards

---

## 💡 Tip

Para asegurar siempre una salida numérica:

```bash
( comando ) || echo 0
```

---

Si querés, te armo también:

* un README completo para **CPU + RAM + DISCO + GPU**
* o una versión **sin jq** para Intel

Decime 😎
¡Perfecto! Te voy a dar un **tutorial paso a paso para obtener `credentials.json`** listo para poner en tu README. Lo voy a escribir de forma clara, como para que cualquiera lo siga.

---

# Cómo obtener `credentials.json` para Google Calendar API

Para usar la **Google Calendar API** en tu proyecto Rust (o cualquier lenguaje), necesitas un archivo `credentials.json` que contenga tus credenciales de OAuth 2.0. Este archivo permite que tu aplicación acceda a la API en nombre de un usuario.

---

## 1️⃣ Crear un proyecto en Google Cloud

1. Ve a [Google Cloud Console](https://console.cloud.google.com/).
2. Haz clic en **Seleccionar proyecto → Nuevo proyecto**.
3. Escribe un nombre para tu proyecto y haz clic en **Crear**.
4. Espera unos segundos hasta que el proyecto se haya creado.

---

## 2️⃣ Activar la API de Google Calendar

1. Con tu proyecto seleccionado, ve a **APIs y servicios → Biblioteca**.
2. Busca **Google Calendar API**.
3. Haz clic en **Habilitar**.

---

## 3️⃣ Crear credenciales OAuth 2.0

1. Ve a **APIs y servicios → Credenciales**.
2. Haz clic en **+ Crear credenciales → ID de cliente OAuth**.
3. Si es la primera vez, te pedirá configurar **Pantalla de consentimiento**:

   * Nombre de la aplicación: `Mi App Rust` (o lo que quieras)
   * Correo de soporte: tu correo
   * Guarda los cambios
4. Tipo de aplicación: **Aplicación de escritorio**
5. Nombre: `Mi App Rust Desktop`
6. Haz clic en **Crear**.
7. Descarga el archivo JSON haciendo clic en **Descargar JSON**.

> Este archivo es tu `credentials.json`.

---

