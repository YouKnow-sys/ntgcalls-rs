# NTgCall-rs
A simple safe binding for NTgCalls C api.

## Todo
- [ ] Add support for linking NTgCalls lib statically when NTgCalls added it.
- [x] Add support for downloading precompiled NTgCalls lib if a feature flag is enabled instead of putting the lib directly in the project. (kinda done, but im just hardcoding a version at this point and also including a lib file)
- [ ] Clean up the code.
- [x] Copy the shared lib to output folder after build for each platform. (can get buggy if build for multiplatform)