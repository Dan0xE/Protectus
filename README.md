# Protectus


## Supported functions

- get_hwid,
- set_serial_number,
- get_serial_number_state,
- get_serial_number_data,
- activate_license,
- deactivate_license,
- is_protected,
- is_virtual_machine,
- is_debugger_present

This will be expanded in the near future!

## Tauri Plugin Permissions

> [!IMPORTANT]
> Protectus performs feature checks at runtime before executing any function.
> This is done to verify whether a feature that the called function depends on (for example, 'licensing')
> is enabled in the Rust backend, in order to prevent unwanted behavior.
> Without adding the following permission, Protectus will not function properly!

```json
"permissions": [
    "protectus:allow-feature-check-command"
]
```

Protectus Permission List:


```json
"permissions": [
    "protectus:allow-feature-check-command"
    "protectus:allow-is-debugger-present-command",
    "protectus:allow-is-protected-command",
    "protectus:allow-is-virtual-machine-command",
    "protectus:allow-get-hwid-command",
    "protectus:allow-get-serial-number-data-command",
    "protectus:allow-get-serial-number-state-command",
    "protectus:allow-activate-license-command",
    "protectus:allow-deactivate-license-command",
    "protectus:allow-set-serial-number-command"
  ]
```

## Prerequisites

- VmProtect Installation
- Tauri Version >= 2.0.0-beta

When using a Debug Build of your application, copy either all files or only the files necessary for your operating system and architecture from VmProtect into your `target/debug` folder:

- For macOS: `libVMProtectSDK.dylib`
- For 32-bit Linux: `libVMProtectSDK32.so`
- For 64-bit Linux: `libVMProtectSDK64.so`
- For 32-bit Windows: `VMProtectSDK32.dll`, `VMProtectSDK32.lib`
- For 64-bit Windows: `VMProtectSDK64.dll`, `VMProtectSDK64.lib`

## Limitations when using Debug Builds

Some functions do not return a valid result until the Application is protected with VmProtect


## What does x function do?

For more information on what certain functions do, please consult these sites:

- [VmProtect Docs](https://vmpsoft.com/vmprotect/user-manual/)
- [VmProtect Rust](https://github.com/Dan0xE/vmprotect/tree/master)

---

BIG BIG BIG THANKS to [CertainLach](https://github.com/CertainLach/vmprotect/commits?author=CertainLach) for creating VmProtect Rust!
