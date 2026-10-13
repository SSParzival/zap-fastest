//! Per-call voice cleanup. Only the microphone is changed; render audio is a
//! reference for acoustic echo cancellation, never saved or sent elsewhere.

use sonora::config::{EchoCanceller, HighPassFilter, NoiseSuppression, NoiseSuppressionLevel};
use sonora::{AudioProcessing, Config, StreamConfig};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

const BLOCK: usize = 160; // 10 ms at the existing 16 kHz mono call clock.

struct State {
    apm: AudioProcessing,
    render_pending: Vec<i16>,
    render_output: [i16; BLOCK],
    capture_blocks: u64,
    render_blocks: u64,
}

impl State {
    fn new() -> Self {
        Self {
            apm: AudioProcessing::builder()
                .capture_config(StreamConfig::new(16000, 1))
                .render_config(StreamConfig::new(16000, 1))
                .config(Config {
                    echo_canceller: Some(EchoCanceller::default()),
                    high_pass_filter: Some(HighPassFilter::default()),
                    noise_suppression: Some(NoiseSuppression {
                        level: NoiseSuppressionLevel::Low,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build(),
            render_pending: Vec::with_capacity(BLOCK),
            render_output: [0; BLOCK],
            capture_blocks: 0,
            render_blocks: 0,
        }
    }
}

pub(crate) struct CallProcessing {
    state: Mutex<State>,
    failed: AtomicBool,
}

impl CallProcessing {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(State::new()),
            failed: AtomicBool::new(false),
        })
    }

    pub(crate) fn failed(&self) -> bool {
        self.failed.load(Ordering::Relaxed)
    }

    pub(crate) fn counters(&self) -> (u64, u64) {
        match self.state.lock() {
            Ok(state) => (state.capture_blocks, state.render_blocks),
            Err(_) => {
                self.fail();
                (0, 0)
            }
        }
    }

    fn fail(&self) {
        if !self.failed.swap(true, Ordering::Relaxed) {
            log::error!(
                "call microphone cleanup failed; audio is being sent without echo cancellation"
            );
        }
    }

    /// A device change invalidates the acoustic path. Do not carry adaptation
    /// from a different speaker/headset into its replacement.
    pub(crate) fn reset(&self) {
        let mut state = self.state.lock().unwrap_or_else(|error| error.into_inner());
        *state = State::new();
        self.state.clear_poison();
        self.failed.store(false, Ordering::Relaxed);
    }

    /// Called by the speaker thread immediately before queueing actual playout,
    /// after device selection/backpressure, rather than upon network receipt.
    pub(crate) fn render(&self, frame: &[i16]) {
        if self.failed() {
            return;
        }
        let Ok(mut state) = self.state.lock() else {
            self.fail();
            return;
        };
        for sample in frame {
            state.render_pending.push(*sample);
            if state.render_pending.len() == BLOCK {
                let State {
                    apm,
                    render_pending,
                    render_output,
                    ..
                } = &mut *state;
                if apm
                    .process_render_i16(render_pending, render_output)
                    .is_err()
                {
                    self.fail();
                    return;
                }
                render_pending.clear();
                state.render_blocks += 1;
            }
        }
    }

    /// Called on the microphone thread. The 60 ms transport frame remains the
    /// same length; cleanup processes its six 10 ms blocks without a new queue.
    pub(crate) fn capture(&self, frame: &mut [i16]) {
        if self.failed() {
            return;
        }
        if !frame.len().is_multiple_of(BLOCK) {
            self.fail();
            return;
        }
        let Ok(mut state) = self.state.lock() else {
            self.fail();
            return;
        };
        for block in frame.as_chunks_mut::<BLOCK>().0 {
            let mut output = [0; BLOCK];
            if state.apm.process_capture_i16(block, &mut output).is_err() {
                self.fail();
                return;
            }
            block.copy_from_slice(&output);
            state.capture_blocks += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(samples: usize) -> Vec<i16> {
        let mut seed = 123_u32;
        (0..samples)
            .map(|_| {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                ((seed >> 16) as i16) / 3
            })
            .collect()
    }

    #[test]
    fn cancels_delayed_synthetic_speaker_echo_with_real_transport_frame_sizes() {
        let processor = CallProcessing::new();
        let noise_only = CallProcessing::new();
        {
            let mut state = noise_only.state.lock().unwrap();
            let mut config = state.apm.config().clone();
            config.echo_canceller = None;
            state.apm.apply_config(config);
        }
        let reference = reference(16000 * 20);
        let mut raw = 0_f64;
        let mut clean = 0_f64;
        let mut noise_only_energy = 0_f64;
        for n in 0..reference.len() / 960 {
            let first = n * 960;
            processor.render(&reference[first..first + 960]);
            noise_only.render(&reference[first..first + 960]);
            let mut capture: Vec<i16> = (first..first + 960)
                .map(|i| if i >= 640 { reference[i - 640] / 2 } else { 0 })
                .collect();
            let mut control = capture.clone();
            noise_only.capture(&mut control);
            if first > 16000 * 10 {
                raw += capture.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>();
            }
            processor.capture(&mut capture);
            assert_eq!(capture.len(), 960);
            if first > 16000 * 10 {
                clean += capture.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>();
                noise_only_energy += control.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>();
            }
        }
        assert!(!processor.failed());
        assert!(!noise_only.failed());
        let reduction = 10.0 * (raw / clean.max(1.0)).log10();
        assert!(
            reduction > 20.0,
            "measured synthetic echo reduction {reduction:.1} dB"
        );
        assert!(
            clean < noise_only_energy * 0.01,
            "echo cancellation must outperform noise suppression alone"
        );
        eprintln!("Synthetic echo reduction: {reduction:.1} dB");
    }

    #[test]
    fn silence_stays_silent_and_bad_capture_framing_is_reported_until_device_reset() {
        let processor = CallProcessing::new();
        let mut silence = [0; 960];
        for _ in 0..100 {
            processor.capture(&mut silence);
        }
        assert!(silence.iter().all(|sample| *sample == 0));
        processor.capture(&mut [100; 159]);
        assert!(processor.failed());
        processor.reset();
        processor.capture(&mut silence);
        assert!(!processor.failed());
    }

    #[test]
    fn independent_near_end_voice_survives_while_speaker_echo_is_removed() {
        let processor = CallProcessing::new();
        let reference = reference(16000 * 20);
        let mut sin = 0_f64;
        let mut cos = 0_f64;
        let mut samples = 0;
        for n in 0..reference.len() / 960 {
            let first = n * 960;
            processor.render(&reference[first..first + 960]);
            let mut capture: Vec<i16> = (first..first + 960)
                .map(|i| {
                    let echo = if i >= 640 {
                        f64::from(reference[i - 640]) / 2.0
                    } else {
                        0.0
                    };
                    let voice = if i >= 16000 * 10 {
                        (i as f64 * std::f64::consts::TAU * 200.0 / 16000.0).sin() * 4000.0
                    } else {
                        0.0
                    };
                    (echo + voice) as i16
                })
                .collect();
            processor.capture(&mut capture);
            if first > 16000 * 15 {
                for (offset, sample) in capture.iter().enumerate() {
                    let phase = (first + offset) as f64 * std::f64::consts::TAU * 200.0 / 16000.0;
                    sin += f64::from(*sample) * phase.sin();
                    cos += f64::from(*sample) * phase.cos();
                    samples += 1;
                }
            }
        }
        let voice_amplitude = 2.0 * sin.hypot(cos) / f64::from(samples);
        assert!(!processor.failed());
        assert!(
            voice_amplitude > 1000.0,
            "independent voice amplitude {voice_amplitude:.1}"
        );
    }
}
