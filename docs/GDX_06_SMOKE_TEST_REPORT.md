# GDX-06 Host Smoke Test Report

**Plugin:** GenX Delay v0.1.0
**Build:** `cd plugins/genx_delay && ./build.sh Release`
**Bundles:** `build/GenXDelay_artefacts/Release/` (VST3, AU, Standalone)
**Date:** ___________
**Tester:** ___________

---

## Installation

Copy bundles to host-visible plugin directories before testing:

```bash
# VST3
cp -r build/GenXDelay_artefacts/Release/VST3/"GenX Delay.vst3" ~/Library/Audio/Plug-Ins/VST3/

# AU (automatically copied if COPY_PLUGIN_AFTER_BUILD is enabled)
cp -r build/GenXDelay_artefacts/Release/AU/"GenX Delay.component" ~/Library/Audio/Plug-Ins/Components/
```

Rescan plugins in each host after copying.

---

## Host Matrix

Per `docs/VST_GUI_DEVELOPMENT_STANDARDS.md` Section 6, minimum 4 hosts required.

### Host 1: REAPER (macOS)

| # | Check | Format | Result | Notes |
|---|-------|--------|--------|-------|
| 1 | Insert plugin and open GUI | VST3 / AU | [ ] Pass [ ] Fail | |
| 2 | Automate 3+ params during playback | VST3 / AU | [ ] Pass [ ] Fail | |
| 3 | Save project, close, reopen, verify state | VST3 / AU | [ ] Pass [ ] Fail | |
| 4 | Toggle GUI open/close repeatedly during playback | VST3 / AU | [ ] Pass [ ] Fail | |
| 5 | Resize window + HiDPI scaling | VST3 / AU | [ ] Pass [ ] Fail | |

**Params automated:** ___________
**Version:** REAPER ___________
**Overall:** [ ] PASS [ ] FAIL

---

### Host 2: Ableton Live (macOS)

| # | Check | Format | Result | Notes |
|---|-------|--------|--------|-------|
| 1 | Insert plugin and open GUI | VST3 | [ ] Pass [ ] Fail | |
| 2 | Automate 3+ params during playback | VST3 | [ ] Pass [ ] Fail | |
| 3 | Save project, close, reopen, verify state | VST3 | [ ] Pass [ ] Fail | |
| 4 | Toggle GUI open/close repeatedly during playback | VST3 | [ ] Pass [ ] Fail | |
| 5 | Resize window + HiDPI scaling | VST3 | [ ] Pass [ ] Fail | |

**Params automated:** ___________
**Version:** Ableton Live ___________
**Overall:** [ ] PASS [ ] FAIL

---

### Host 3: Bitwig Studio (macOS)

| # | Check | Format | Result | Notes |
|---|-------|--------|--------|-------|
| 1 | Insert plugin and open GUI | VST3 / AU | [ ] Pass [ ] Fail | |
| 2 | Automate 3+ params during playback | VST3 / AU | [ ] Pass [ ] Fail | |
| 3 | Save project, close, reopen, verify state | VST3 / AU | [ ] Pass [ ] Fail | |
| 4 | Toggle GUI open/close repeatedly during playback | VST3 / AU | [ ] Pass [ ] Fail | |
| 5 | Resize window + HiDPI scaling | VST3 / AU | [ ] Pass [ ] Fail | |

**Params automated:** ___________
**Version:** Bitwig Studio ___________
**Overall:** [ ] PASS [ ] FAIL

---

### Host 4: ____________ (macOS)

| # | Check | Format | Result | Notes |
|---|-------|--------|--------|-------|
| 1 | Insert plugin and open GUI | VST3 / AU | [ ] Pass [ ] Fail | |
| 2 | Automate 3+ params during playback | VST3 / AU | [ ] Pass [ ] Fail | |
| 3 | Save project, close, reopen, verify state | VST3 / AU | [ ] Pass [ ] Fail | |
| 4 | Toggle GUI open/close repeatedly during playback | VST3 / AU | [ ] Pass [ ] Fail | |
| 5 | Resize window + HiDPI scaling | VST3 / AU | [ ] Pass [ ] Fail | |

**Params automated:** ___________
**Version:** ___________
**Overall:** [ ] PASS [ ] FAIL

---

## Suggested Params to Automate

Pick at least 3 per host. Good candidates covering different control types:

| Param | Type | Why |
|-------|------|-----|
| **Delay Time** | Float (continuous) | Core param, tests smooth interpolation |
| **Mix** | Float (continuous) | Audible feedback on automation accuracy |
| **Feedback** | Float (continuous) | Tests smoothing, verifies no runaway |
| **Mode** | Enum (Digital/Analog) | Tests discrete automation + modulation gating |
| **Tempo Sync** | Bool | Tests checkbox automation + note-division enable |
| **Mod Rate** | Float (Analog only) | Tests mode-gated param automation |

---

## Known Issues

| Host | Issue | Severity | Workaround |
|------|-------|----------|------------|
| | | | |

---

## Sign-Off

- [ ] All 4 hosts tested with all 5 checks passing
- [ ] At least 3 params automated per host
- [ ] No crashes or hangs observed
- [ ] State restore is accurate across save/reload
- [ ] GUI resize/HiDPI behaves correctly at 100%, 150%, 200%
- [ ] GDX-06 test gate verified
- [ ] Kanban board updated to Done

**GDX-06 Status:** [ ] PASS — ready for release [ ] FAIL — see Known Issues
