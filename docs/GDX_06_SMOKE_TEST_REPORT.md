# GDX-06 Host Smoke Test Report

**Plugin:** GenX Delay v0.1.0
**Build:** `cargo xtask bundle genx_delay --release`
**Bundles:** `target/bundled/genx_delay.vst3`, `target/bundled/genx_delay.clap`
**Date:** ___________
**Tester:** ___________

---

## Installation

Copy bundles to host-visible plugin directories before testing:

```bash
# VST3
cp -r target/bundled/genx_delay.vst3 ~/Library/Audio/Plug-Ins/VST3/

# CLAP
cp -r target/bundled/genx_delay.clap ~/Library/Audio/Plug-Ins/CLAP/
```

Rescan plugins in each host after copying.

---

## Host Matrix

Per `docs/VST_GUI_DEVELOPMENT_STANDARDS.md` Section 6, minimum 4 hosts required.

### Host 1: REAPER (macOS)

| # | Check | Format | Result | Notes |
|---|-------|--------|--------|-------|
| 1 | Insert plugin and open GUI | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 2 | Automate 3+ params during playback | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 3 | Save project, close, reopen, verify state | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 4 | Toggle GUI open/close repeatedly during playback | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 5 | Resize window + HiDPI scaling | VST3 / CLAP | [ ] Pass [ ] Fail | |

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
| 1 | Insert plugin and open GUI | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 2 | Automate 3+ params during playback | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 3 | Save project, close, reopen, verify state | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 4 | Toggle GUI open/close repeatedly during playback | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 5 | Resize window + HiDPI scaling | VST3 / CLAP | [ ] Pass [ ] Fail | |

**Params automated:** ___________
**Version:** Bitwig Studio ___________
**Overall:** [ ] PASS [ ] FAIL

---

### Host 4: ____________ (macOS)

| # | Check | Format | Result | Notes |
|---|-------|--------|--------|-------|
| 1 | Insert plugin and open GUI | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 2 | Automate 3+ params during playback | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 3 | Save project, close, reopen, verify state | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 4 | Toggle GUI open/close repeatedly during playback | VST3 / CLAP | [ ] Pass [ ] Fail | |
| 5 | Resize window + HiDPI scaling | VST3 / CLAP | [ ] Pass [ ] Fail | |

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
- [ ] GDX-06 test gate updated in `lib.rs`
- [ ] Kanban board updated to Done

**GDX-06 Status:** [ ] PASS — ready for release [ ] FAIL — see Known Issues
