# GenX Delay — Distribution & Installation Guide

This guide explains what the GenX Delay plugin is, how to build it, how to share it with others, and how end users install and use it. No prior knowledge of C++, VST, or audio programming is assumed.

---

## What Is This Plugin?

GenX Delay is an **audio effect plugin** — a piece of software that runs *inside* a music production program (called a **DAW**, short for Digital Audio Workstation). Think of it like an Instagram filter, but for sound instead of photos. It adds echo/delay effects to audio.

### Plugin Formats

The plugin is built in three industry-standard formats:

| Format | File Extension | Who Uses It |
|--------|---------------|-------------|
| **VST3** | `.vst3` | Almost every DAW (Ableton Live, FL Studio, REAPER, Cubase, Studio One, Logic Pro, etc.) |
| **AU** | `.component` | macOS DAWs (Logic Pro, GarageBand, Ableton Live, REAPER, etc.) |
| **Standalone** | `.app` (macOS) | Run without a DAW |

You don't need to understand the technical difference. Just know that **VST3 has the widest cross-platform compatibility**, and **AU is the native macOS format** preferred by Logic Pro and GarageBand. When distributing, include all formats so users can pick the one their DAW supports.

---

## How the Plugin Gets Built

The plugin is written in **C++** using the **JUCE framework**. The source code lives in this repository under `plugins/genx_delay`. To turn the source code into the plugin files that users can install, someone with the development environment set up runs:

```bash
cd plugins/genx_delay
./build.sh Release
```

This produces plugin files under `build/GenXDelay_artefacts/Release/`:
- `GenX Delay.vst3`
- `GenX Delay.component` (AU, macOS only)
- `GenX Delay.app` (Standalone, macOS only)

These are the **only files needed for distribution**. End users do NOT need C++, CMake, or any programming knowledge. They just need these files.

### Building for Different Operating Systems

One important caveat: the build command produces plugin files **only for the computer you build on**:
- Building on a **Mac** produces Mac plugins
- Building on a **Windows PC** produces Windows plugins
- Building on **Linux** produces Linux plugins

If you want to distribute to all platforms, you need to build on each platform (or set up cross-compilation / CI, which is an advanced topic).

---

## What to Distribute

When sharing the plugin, users need:

| Item | Required? | What It Is |
|------|-----------|------------|
| `GenX Delay.vst3` | Yes (for VST3 users) | The VST3 plugin bundle |
| `GenX Delay.component` | Yes (for AU users on macOS) | The AU plugin bundle |
| `GenX Delay.app` | Optional | Standalone application (no DAW needed) |
| A README or install instructions | Recommended | So users know where to put the files |
| A license file | Recommended | This project uses GPL-3.0 |

You do NOT need to include any source code, font files, image files, or anything else. Everything is embedded inside the plugin files.

---

## Where to Share / Sell the Plugin

### Free Distribution

| Platform | URL | Notes |
|----------|-----|-------|
| **GitHub Releases** | Your repo's Releases page | Best for open-source. Upload `.vst3` and `.clap` as release assets. Free. |
| **KVR Audio** | https://www.kvraudio.com | The largest directory of audio plugins. Free to list free plugins. Huge audience of musicians and producers. |
| **itch.io** | https://itch.io | Originally for indie games, but widely used for creative software. Supports "pay what you want" pricing. |
| **Your own website** | — | Full control. Just host the files for download. |

### Paid Distribution

| Platform | URL | Notes |
|----------|-----|-------|
| **Gumroad** | https://gumroad.com | Simple storefront for digital products. Handles payments, download delivery, and license keys. Takes a small percentage per sale. |
| **itch.io** | https://itch.io | Supports paid downloads and "pay what you want" with a minimum price. |
| **Plugin Boutique** | https://www.pluginboutique.com | Dedicated audio plugin marketplace. Large audience but requires approval and they take a revenue share. |
| **Your own website + Stripe/PayPal** | — | Maximum control and revenue, but you handle everything yourself. |

### Tips for Listing

- Include **screenshots** of the plugin GUI running inside a DAW
- Mention supported formats (VST3, CLAP) and platforms (macOS, Windows, etc.)
- List which DAWs you've tested in (REAPER, Ableton Live, Bitwig, etc.)
- Provide a short audio demo so people can hear what it sounds like
- State the license clearly (GPL-3.0 for this project)

---

## How End Users Install the Plugin

### macOS Installation

Users download the `.vst3` and/or `.clap` file, then copy it to the correct system folder:

**VST3:**
```
Copy "GenX Delay.vst3" to:
/Library/Audio/Plug-Ins/VST3/
```
Or for the current user only:
```
~/Library/Audio/Plug-Ins/VST3/
```

**AU (Audio Unit):**
```
Copy "GenX Delay.component" to:
/Library/Audio/Plug-Ins/Components/
```
Or for the current user only:
```
~/Library/Audio/Plug-Ins/Components/
```

Note: If you build with `COPY_PLUGIN_AFTER_BUILD` enabled (the default), plugins are automatically copied to these system folders after a successful build.

The `~/Library` folder is hidden by default on Mac. Users can access it by:
1. Opening Finder
2. Clicking the **Go** menu in the menu bar
3. Holding the **Option** key — "Library" will appear in the dropdown
4. Navigating to `Audio > Plug-Ins > VST3` (or `CLAP`)

Alternatively, press **Cmd+Shift+G** in Finder and paste the path.

### Windows Installation

**VST3:**
```
Copy "GenX Delay.vst3" to:
C:\Program Files\Common Files\VST3\
```

### Linux Installation

**VST3:**
```
Copy "GenX Delay.vst3" to:
~/.vst3/
```

### After Copying

After placing the file in the correct folder, open your DAW and **rescan plugins**:

| DAW | How to Rescan |
|-----|---------------|
| **Ableton Live** | Preferences > Plug-ins > Rescan (or restart Ableton) |
| **REAPER** | Options > Preferences > VST > Re-scan |
| **Bitwig Studio** | Settings > Plug-ins > Rescan (or restart Bitwig) |
| **FL Studio** | Options > Manage Plugins > Start Scan |
| **Logic Pro** | Logic Pro > Settings > Plug-in Manager > Reset & Rescan |
| **Cubase** | Studio > VST Plug-in Manager > Rescan All |
| **Studio One** | Studio One > Options > Locations > VST Plug-ins > Rescan |

The plugin should then appear in your DAW's effect/plugin list, typically under the vendor name **"trwolf"** or in the **Delay** category.

---

## How End Users Use the Plugin

1. **Insert the plugin** on an audio or bus track in your DAW (each DAW does this differently, but it's usually right-clicking the track's effect chain and browsing for "GenX Delay")
2. The plugin GUI will open showing a crimson red window with delay controls
3. Adjust the knobs and sliders:
   - **Delay Time** — how long the echo takes to repeat
   - **Feedback** — how many times the echo repeats
   - **Mix** — blend between the original sound and the echo
   - **Mode** — Digital (clean) or Analog (warm, with modulation and drive)
   - Plus tone, stereo, modulation, and ducking controls
4. Automate parameters by recording parameter changes during playback in your DAW

---

## Frequently Asked Questions

**Q: Do users need to install C++ or any programming tools?**
A: No. The `.vst3`, `.component`, and `.app` files are fully self-contained. Users just copy them to the right folder.

**Q: Why does macOS say the plugin is from an "unidentified developer"?**
A: Apple requires developers to pay $99/year for a Developer ID certificate and **notarize** their software. Without this, macOS Gatekeeper will block the plugin. Users can work around this by right-clicking the file, choosing "Open", and confirming — but for a professional release, you should sign and notarize the plugin. See Apple's [developer documentation](https://developer.apple.com/developer-id/) for details.

**Q: Can I distribute a Windows version if I only have a Mac?**
A: Not directly from your Mac. You would need either a Windows machine, a Windows virtual machine, or a CI/CD service like GitHub Actions that can build on Windows. A GitHub Actions workflow can automatically build for Mac, Windows, and Linux on every release.

**Q: What does GPL-3.0 mean for distribution?**
A: The GPL-3.0 license means:
- Anyone can use, modify, and redistribute the plugin
- If someone distributes a modified version, they must also release their source code under GPL-3.0
- You CAN sell GPL software — the license allows commercial distribution
- You MUST make the source code available to anyone who receives the binary (e.g., link to this GitHub repo)

**Q: What if I want to sell the plugin without sharing source code?**
A: You would need to change the license to a proprietary or more permissive one. Since you are the copyright holder, you can re-license your own code. However, check the licenses of all dependencies (JUCE has its own licensing terms) — consult a lawyer for commercial licensing questions.

**Q: Can the plugin run as a standalone application (without a DAW)?**
A: Yes. The JUCE build produces a Standalone format in addition to VST3 and AU. The standalone `.app` is included in the build output.

---

## Quick-Start Checklist for Sharing

1. Build the plugin: `cd plugins/genx_delay && ./build.sh Release`
2. Locate the output files in `build/GenXDelay_artefacts/Release/`
3. Create a zip file containing the `.vst3`, `.component`, and `.app` bundles
4. Write a short description and take a screenshot of the GUI
5. Upload to your chosen platform (GitHub Releases, KVR, Gumroad, etc.)
6. Include install instructions (point users to the "How End Users Install" section above, or write a short version)
7. Done — people can download, install, and start using the plugin
