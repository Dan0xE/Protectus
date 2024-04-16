# Protecuts

Call VmProtect functions directly from your Tauri WebView!

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

```json
"permissions": [
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

## Prerequisite

- VmProtect Installation
- Tauri Version > 2.0.0-beta.0

When using a Debug Build of your Application, please copy the following files from VmProtect into your target/debug folder:

- libVMProtectSDK.dylib
- libVMProtectSDK32.so
- libVMProtectSDK64.so
- VMProtectSDK32.dll
- VMProtectSDK32.lib
- VMProtectSDK64.dll
- VMProtectSDK64.lib

## Limitations of the Demo

Some functions do not return a valid result until the Application is protected with VmProtect


## What does x function do?

For more information on what certain functions do, please consult these sites:

- [VmProtect Docs](https://vmpsoft.com/vmprotect/user-manual/)
- [VmProtect Rust](https://github.com/Dan0xE/vmprotect/tree/master)

---

BIG BIG BIG THANKS to [CertainLach](https://github.com/CertainLach/vmprotect/commits?author=CertainLach) for creating VmProtect Rust!