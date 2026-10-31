# Android port (experimental)

This branch is the first Android bring-up layer for Horizon. It is **not yet a complete mobile release**.

## Current changes

- Adds an Android-aware Winit event-loop builder using the AndroidApp passed by android-activity.
- Adds a cdylib target and enables Winit's android-native-activity backend.
- Exposes the existing client game loop to the library and adds an android_main entry point.
- Sets the app-specific data directory as the working directory.
- Adds a GitHub Actions workflow that attempts an ARM64 APK build, signs the result with a one-run CI key, verifies the signature, and uploads the artifact.

## Current touch implementation

The Android-only input path already handles independent Winit touch IDs and provides a first playable HUD: a virtual joystick, a right-side camera-look area, jump, sneak, sprint, attack, use, inventory, perspective and hotbar taps. It keeps the desktop keyboard/mouse path intact. The `EDIT` overlay can drag the joystick and buttons, change the overall scale, and persist normalized button centers in `android-controls.json`.

This is a prototype, not the requested full Bedrock-style controls/settings experience. It does not yet provide a D-pad option, a Mobile page inside the game's Options menu, per-button enable/disable or sizing, an opacity control, camera sensitivity/inversion settings, reset/cancel layout actions, or dedicated drop/pause buttons. The persisted opacity value currently has no UI control. These gaps must be closed and tested before calling the mobile controls complete.

## Important limitations

- **Build status is not yet verified.** The first Android CI run compiled most of the Rust dependency graph but stopped at native linking (`ld.lld: unable to find library -laaudio`). The workflow has been changed to omit the optional desktop audio feature for this initial Android bring-up; it still needs a fresh run and a successful APK packaging step.
- **No Bedrock PNG files are bundled.** The supplied archive says its images came from `Mojang/bedrock-samples`; that repository's `LICENSE.md` says copyright belongs to Mojang AB and the files are subject to the Minecraft EULA. No separate redistribution grant was supplied, so the port uses its own drawn controls for now rather than copying those textures into the APK. Revisit only with rights that allow redistribution.
- **Minecraft Java resources are not included.** Horizon requires prepared game files described by horizon-resources.json. Supply files you are legally entitled to use; this repository must not redistribute Mojang's copyrighted assets.
- **Minecraft Java resources are not included.** Horizon requires prepared game files described by horizon-resources.json. Supply files you are legally entitled to use; this repository must not redistribute Mojang's copyrighted assets.
- **The CI signing key is ephemeral.** An APK signed by this workflow is for testing only and cannot be updated in place by a later run, because each run uses a new key. A stable release needs a persistent signing key stored as a protected GitHub Actions secret.
- **No device testing has been performed yet.** Vulkan/GLES selection, pause/resume, memory use, touch input, and performance on the Redmi Pad SE 8.7 remain to be validated.

## Build artifact

Open the repository's Actions tab and select Android ARM64 APK (experimental). The workflow is configured to upload the APK and SHA-256 checksum when the build succeeds. A failed run is useful diagnostic output, not a downloadable release.

## Next milestones

1. Get the first ARM64 compile/package run green and fix compiler/linker/manifest errors.
2. Confirm the APK installs, opens a Winit surface, resumes after backgrounding, and logs failures clearly.
3. Finish the Mobile settings page and editor: D-pad/joystick choice, per-control visibility and size, opacity, sensitivity/inversion, undo/cancel and reset.
4. Add the remaining actions (drop and pause/menu), verify press/release/cancel/focus-loss behavior, and add automated tests for the Android touch state machine.
5. Add a resource import flow and a stable signing setup.
6. Test on the target tablet, profile frame time and memory, then create a versioned release only after those checks pass.
