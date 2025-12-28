# HTTP Toolkit Patcher

A specialized tool for patching **HTTP Toolkit** to unlock Pro features through `app.asar` modification.

<<<<<<< HEAD
> This tool is more stable and performant than other alternatives as it doesn't require a mitm proxy server to continuously be running to patch. Instead the patch is built directly into http toolkit. This however has it's downsides as it means that if http toolkit updates, the patch may be overwritten and need to be reapplied.

> [!WARNING]
> **FOR EDUCATIONAL PURPOSES ONLY.**
> This project was created to explore patching, and reverse engineering concepts. It is not intended for piracy or commercial use. If you find HTTP Toolkit useful, please support the original developers by purchasing a legitimate Pro license.
=======
> [!WARNING]  
> **FOR EDUCATIONAL PURPOSES ONLY.** > This project explores patching and reverse engineering concepts. It is not intended for piracy. If you find HTTP Toolkit useful, please support the developers by purchasing a legitimate Pro license.
>>>>>>> 1.24.2-integrity-check

---

## Features

- **Automatic Detection:** Automatically locates your HTTP Toolkit installation.
- **Safety First:** Creates an automatic backup of your original `app.asar` and binary.
- **Rust-Powered:** Fast, memory-safe execution.

## Installation & Usage

Currently, **Windows** is the only supported platform.

1.  **Download:** Grab the latest executable from the [Releases](../../releases) tab.
2.  **Terminal:** Open an Administrator Command Prompt or PowerShell.
3.  **Navigate:** Use `cd` to enter the directory where you downloaded the tool.
4.  **Run:** Execute the patcher:
    ```powershell
    .\http-toolkit-patcher.exe
    ```
5.  **Manual Path (Optional):** If the tool doesn't find your install, pass the path manually:
    - **User Install:** `%LocalAppData%\Programs\httptoolkit\resources\app.asar`
    - **System Install:** `C:\Program Files\HTTP Toolkit\resources\app.asar`

### Status Indicators

- ✅ **SUCCESS:** Patch completed! You are ready to go.
- ❌ **FAILURE:** Patch failed. Check the log for the specific error reason.

---

## How It Works

The patcher modifies the `app.asar` archive to enable the **Chrome DevTools Protocol**. This allows for the interception and alteration of `main.js` at runtime, injecting a modified user object that grants premium plan access.

_Note: Since this modifies core files, you may need to re-run the patcher after an HTTP Toolkit update._

---

## Recovery & Troubleshooting

If the application fails to launch or behaves unexpectedly, restore your backup:

1.  Navigate to the `resources` folder in your install directory.
2.  Delete the modified `app.asar`.
3.  Rename `app.asar.backup` to `app.asar`.
4.  Restart HTTP Toolkit.

---

## 🏗️ Building from Source

Requires the **Rust/Cargo** toolchain.

```bash
git clone [https://github.com/7i7u5/http-toolkit-patch](https://github.com/7i7u5/http-toolkit-patch)
cd http-toolkit-patch
cargo build --release
```

## Disclaimer

The author and contributors are not responsible for misuse, data loss, or account bans. Use this software at your own risk.
