# Android port (experimental)

This branch is the first Android bring-up layer for Horizon. It is **not yet a complete mobile release**.

## Current changes

- Adds an Android-aware Winit event-loop builder using the AndroidApp passed by android-activity.
- Adds a cdylib target and enables Winit's android-native-activity backend.
- Exposes the existing client game loop to the library and adds an android_main entry point.
- Sets the app-specific data directory as the working directory.
- Adds a GitHub Actions workflow that attempts an ARM64 APK build, signs the result with a one-run CI key, verifies the signature, and uploads the artifact.

## Important limitations

- **Build status is not yet verified.** The CI job is the first compile/package test. A successful workflow and an install/launch test on a real device are required before treating this as usable.
- **Touch controls are not implemented by this bootstrap.** Bedrock-style joystick, separate action buttons, editable layout, saved control profiles, and touch camera look still need to be integrated into the gameplay input path.
- **Minecraft Java resources are not included.** Horizon requires prepared game files described by horizon-resources.json. Supply files you are legally entitled to use; this repository must not redistribute Mojang's copyrighted assets.
- **The CI signing key is ephemeral.** An APK signed by this workflow is for testing only and cannot be updated in place by a later run, because each run uses a new key. A stable release needs a persistent signing key stored as a protected GitHub Actions secret.
- **No device testing has been performed yet.** Vulkan/GLES selection, pause/resume, memory use, touch input, and performance on the Redmi Pad SE 8.7 remain to be validated.

## Build artifact

Open the repository's Actions tab and select Android ARM64 APK (experimental). The workflow is configured to upload the APK and SHA-256 checksum when the build succeeds. A failed run is useful diagnostic output, not a downloadable release.

## Next milestones

1. Get the first ARM64 compile/package run green and fix compiler/linker/manifest errors.
2. Confirm the APK installs, opens a Winit surface, resumes after backgrounding, and logs failures clearly.
3. Add a touch-input layer that translates multi-touch contacts into Horizon's existing movement/look/action input without disturbing mouse/keyboard controls.
4. Build a touch HUD and in-game layout editor with drag/resize/opacity controls and persistent JSON settings.
5. Add a resource import flow and a stable signing setup.
6. Test on the target tablet, profile frame time and memory, then create a versioned release only after those checks pass.
