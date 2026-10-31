# Android port (experimental)

This branch currently packages the Horizon **game client** as a NativeActivity APK. It is **not yet the Android port of the desktop Horizon launcher** and is not a complete mobile release.

## Current changes

- Adds an Android-aware Winit event-loop builder using the AndroidApp passed by android-activity.
- Adds a cdylib target and enables Winit's android-native-activity backend.
- Exposes the existing client game loop to the library and adds an android_main entry point.
- Uses `AndroidApp::internal_data_path()` as the working directory and `HORIZON_HOME`, keeping saves, options, logs and the touch layout in app-private storage.
- Packages the ARM64 game-client library into an APK. The Android manifest sets the Horizon app label, launcher icon, network permission, SDK levels and landscape activity.
- Shows an Android toast when the client exits before opening a window because the prepared Minecraft 26.3 files are missing.

## Current touch implementation

The Android-only input path handles independent Winit touch IDs and currently draws a prototype HUD with a virtual joystick or D-pad, a right-side camera-look area, jump, sneak, sprint, attack, use, inventory, perspective, drop, pause and hotbar taps. It keeps the desktop keyboard/mouse path intact. The `EDIT` overlay can drag the joystick and several action buttons, adjust global size and opacity, switch between joystick and D-pad, reset the layout, and persist normalized button centers in `android-controls.json`.

This is not yet the requested full Bedrock-style controls/settings experience. There is no Mobile page inside the game's Options menu, per-button visibility or individual sizing, independent camera swipe sensitivity/inversion controls, or undo/cancel action for layout editing. Size and opacity are global editor settings. The controls are drawn with basic shapes and text rather than separate Bedrock PNG assets. These gaps and press/release/cancel/focus-loss behavior still need tests before calling the mobile controls complete.

## Important limitations

- **An APK build is not proof that the app works on a device.** CI has compiled, packaged, signed and verified an ARM64 APK, but no real Android device test has been performed. The current APK starts the game client directly. Since the APK does not include Mojang's copyrighted game files and has no Android resource-import screen yet, a fresh install without prepared files exits before the game window appears.
- **No Bedrock PNG files are bundled.** The supplied archive says its images came from `Mojang/bedrock-samples`; that repository's `LICENSE.md` says copyright belongs to Mojang AB and the files are subject to the Minecraft EULA. No separate redistribution grant was supplied, so the port uses its own drawn controls for now rather than copying those textures into the APK. Revisit only with rights that allow redistribution.
- **Minecraft Java resources are not included.** Horizon requires prepared game files described by `horizon-resources.json`. Supply files you are legally entitled to use; this repository must not redistribute Mojang's copyrighted assets.
- **The desktop launcher UI is not in this APK.** The original launcher lives in the separate `launcher/` workspace and uses GPUI desktop window APIs. The present Android target builds `horizon-client`, so it cannot be described as having the desktop launcher's one-to-one UI. A real Android launcher front end and import flow are still required.
- **The CI signing key is ephemeral.** An APK signed by this workflow is for testing only and cannot be updated in place by a later run, because each run uses a new key. A stable release needs a persistent signing key stored as a protected GitHub Actions secret.
- **No device testing has been performed yet.** Vulkan/GLES selection, pause/resume, memory use, touch input, and performance on the Redmi Pad SE 8.7 remain to be validated.

## Build artifact

Open the repository's Actions tab and select the `Bootstrap private Horizon workspace` workflow. A successful run uploads the `Horizon-Android-arm64-private-test` APK and its SHA-256 checksum. A failed run is diagnostic output, not a downloadable release.

## Next milestones

1. Port the original desktop launcher's pages and navigation to an Android-capable frontend, keeping the same branding, layout and behavior where Android allows.
2. Add an Android file-import flow for the player's prepared Minecraft resources, so first launch does not terminate on a missing-resource check.
3. Confirm the APK installs, opens a Winit surface, resumes after backgrounding, and logs failures clearly.
4. Finish the Mobile settings page and editor: D-pad/joystick choice, per-control visibility and size, opacity, sensitivity/inversion, undo/cancel and reset.
5. Add the remaining actions (drop and pause/menu), verify press/release/cancel/focus-loss behavior, and add automated tests for the Android touch state machine.
6. Add a stable signing setup.
7. Test on the target tablet, profile frame time and memory, then create a versioned release only after those checks pass.
