use cortex_m::peripheral::SYST;

use crate::device::radio::Power;

use super::{App, Mode, DUAL_STANDBY_HOLD_TICKS, TICKS_PER_SECOND, VOX_HOLD_AFTER_PTT_TICKS};

fn resolve_ani_target(app: &mut App) -> Option<[u8; 3]> {
    if let Some(id) = app.ani_target_override {
        return Some(id);
    }
    let idx = app.sides[app.master].ani_target?;
    let contact = app.storage.read_contact(idx);
    (!contact.is_empty()).then(|| contact.id())
}

fn reload_pa_calibration(app: &mut App, tx_freq_hz: u32, power: Power) {
    app.radio
        .apply_pa_calibration(&mut app.storage, tx_freq_hz, power);
}

pub(super) fn set_ptt(app: &mut App, syst: &mut SYST, pressed: bool) {
    app.note_power_save_activity(syst);
    app.note_backlight_activity();
    if pressed && !app.transmitting {
        if app.mode != Mode::Standby {
            return;
        }
        if app.settings.tx_forbid {
            return;
        }
        if app.settings.busy_lock && app.radio.rssi_open() {
            return;
        }

        app.watching = app.master;
        let s = &app.sides[app.master];
        let tx_freq_hz = s.cfg.tx_freq_hz;
        let power = s.cfg.power;
        app.radio.set_frequency(s.cfg.freq_hz);
        app.radio.set_tx_frequency(tx_freq_hz);
        app.radio.set_power(power);
        app.radio.set_subaudio_tx(s.cfg.subaudio_tx);
        app.radio.set_subaudio_rx(s.cfg.subaudio_rx);
        app.radio.set_modulation(s.cfg.modulation);
        reload_pa_calibration(app, tx_freq_hz, power);
        if app.radio.enter_tx(syst) {
            app.transmitting = true;
            app.tot_ticks = 0;
            app.tx_prohibited = false;
            if let Some(dial) = app.dtmf_dial.take() {
                let digits = &dial.digits[..dial.len];
                if dial.len == 3 {
                    let target = [digits[0], digits[1], digits[2]];
                    app.ani_target_override = Some(target);
                    app.radio.send_ani(syst, target);
                } else if dial.len > 0 {
                    app.radio.send_dtmf_digits(syst, digits);
                }
            } else if app.settings.ani_tx {
                if let Some(target) = resolve_ani_target(app) {
                    app.radio.send_ani(syst, target);
                }
            }
        } else {
            app.tx_prohibited = true;
        }
    } else if pressed {
        app.vox_active = false;
        app.vox_work_dly = 0;
    } else {
        // PTT released.
        app.tx_prohibited = false;
        if app.transmitting {
            app.transmitting = false;
            if app.rtone_sounding {
                app.rtone_sounding = false;
            }
            app.radio.end_tx(syst);
            app.dual_hold_ticks = DUAL_STANDBY_HOLD_TICKS;
            app.vox_det_dly = VOX_HOLD_AFTER_PTT_TICKS;
            app.vox_work_dly = 0;
            app.vox_active = false;
        }
    }
}

pub(super) fn poll_tot(app: &mut App, syst: &mut SYST, ticks: u16) {
    if !app.is_transmitting() || app.settings.tot_level == 0 {
        return;
    }
    app.tot_ticks = app.tot_ticks.saturating_add(ticks);
    let limit = app.settings.tot_seconds() * TICKS_PER_SECOND;
    if app.tot_ticks >= limit {
        set_ptt(app, syst, false);
    }
}
