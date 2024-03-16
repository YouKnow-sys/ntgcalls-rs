# libNTgCalls-sys
build command:
```cmd
bindgen libntgcall-sys\lib\include\ntgcalls.h -o libntgcall-sys\src\bindings.rs --allowlist-var '^NTG_.*' --allowlist-function '^ntg_.*'
```