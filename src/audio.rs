use rodio::{OutputStream, Sink, source::Source, source::SineWave};
use std::time::Duration;
use lazy_static::lazy_static;
use std::sync::Mutex;

/// The Golden Ratio (Stria Synthesis)
pub const PHI: f32 = 1.618034;
/// Cymatic Reference Frequency (Solfeggio Harmony)
pub const BASE_RESONANCE: f32 = 432.0;

lazy_static! {
    /// Global Audio Context: Keeps the OutputStream alive and provides a shared Sink.
    /// On Android, this handles the CPAL/AAudio backend interaction.
    static ref AUDIO_CONTEXT: Mutex<Option<(OutputStream, Sink)>> = Mutex::new(None);
}

pub enum PlayEvent {
    /// Success: A "Golden" ascending tone using PHI.
    Success { base_freq: f32 },
    /// Failure: A dissonant "Hoot" pitch sweep.
    Failure { base_freq: f32 },
    /// Startled: A jagged tritone pulse (1.414 ratio).
    Startled { base_freq: f32 },
    /// Tide Bowl: Plate resonance eigenmodes. Bandwidth narrowed by stability.
    TideBowl { base_freq: f32, stability: f32 },
    /// Ember Chord: Structural tetrahedral intervals.
    EmberChord { frequencies: Vec<f32> },
}

pub struct OperatorSynth;

impl OperatorSynth {
    /// Lazy initialization of the audio hardware.
    pub fn init() {
        let mut ctx = AUDIO_CONTEXT.lock().unwrap();
        if ctx.is_none() {
            match OutputStream::try_default() {
                Ok((stream, handle)) => {
                    if let Ok(sink) = Sink::try_new(&handle) {
                        *ctx = Some((stream, sink));
                    }
                }
                Err(e) => eprintln!("Audio Init Failed: {}", e),
            }
        }
    }

    /// Primary entry point for triggering procedural audio events.
    pub fn play(event: PlayEvent) {
        Self::init();
        let ctx = AUDIO_CONTEXT.lock().unwrap();
        if let Some((_, sink)) = ctx.as_ref() {
            match event {
                PlayEvent::Success { base_freq } => {
                    Self::generate_golden_tone(sink, base_freq);
                }
                PlayEvent::Failure { base_freq } => {
                    Self::generate_dissonant_hoot(sink, base_freq);
                }
                PlayEvent::Startled { base_freq } => {
                    Self::generate_tritone_pulse(sink, base_freq);
                }
                PlayEvent::TideBowl { base_freq, stability } => {
                    Self::generate_plate_resonance(sink, base_freq, stability);
                }
                PlayEvent::EmberChord { frequencies } => {
                    Self::generate_tetrahedral_chord(sink, frequencies);
                }
            }
        }
    }

    /// Chowning’s Golden Ratio Synthesis (The "Stria" Method)
    fn generate_golden_tone(sink: &Sink, base: f32) {
        let freq = if base == 0.0 { BASE_RESONANCE } else { base };
        let s1 = SineWave::new(freq).take_duration(Duration::from_millis(400)).amplify(0.4);
        let s2 = SineWave::new(freq * PHI).take_duration(Duration::from_millis(600)).amplify(0.2);
        let s3 = SineWave::new(freq * PHI * PHI).take_duration(Duration::from_millis(800)).amplify(0.1);
        sink.append(s1.mix(s2).mix(s3));
    }

    /// Dissonant Hoot: A "Devil's Interval" pitch sweep simulator.
    fn generate_dissonant_hoot(sink: &Sink, base: f32) {
        let freq = if base == 0.0 { 200.0 } else { base };
        // Simulation of a fast downward sweep using two close dissonant frequencies
        let s1 = SineWave::new(freq).take_duration(Duration::from_millis(300)).amplify(0.5);
        let s2 = SineWave::new(freq * 1.06).take_duration(Duration::from_millis(300)).amplify(0.5); // Minor Second
        sink.append(s1.mix(s2));
    }

    /// Startled Pulse: Sharp tritone intervals.
    fn generate_tritone_pulse(sink: &Sink, base: f32) {
        let freq = if base == 0.0 { BASE_RESONANCE } else { base };
        let s1 = SineWave::new(freq).take_duration(Duration::from_millis(150)).amplify(0.7);
        let s2 = SineWave::new(freq * 1.414).take_duration(Duration::from_millis(150)).amplify(0.3); // Tritone
        sink.append(s1.mix(s2));
    }

    /// Chladni Figures & Eigenfrequencies (Plate Vibration)
    /// stability (0.0-1.0) determines the clarity of the overtones.
    fn generate_plate_resonance(sink: &Sink, base: f32, stability: f32) {
        let freq = if base == 0.0 { 256.0 } else { base };
        // Approximate eigenmodes of a square plate
        let m1 = freq * 1.0;
        let m2 = freq * 1.498; // Perfect Fifth
        let m3 = freq * 2.245; // Harmonic spread
        
        let amp1 = 0.5;
        let amp2 = 0.2 * stability;
        let amp3 = 0.1 * stability;

        let s1 = SineWave::new(m1).take_duration(Duration::from_secs(3)).amplify(amp1);
        let s2 = SineWave::new(m2).take_duration(Duration::from_secs(2)).amplify(amp2);
        let s3 = SineWave::new(m3).take_duration(Duration::from_secs(1)).amplify(amp3);
        
        sink.append(s1.mix(s2).mix(s3));
    }

    /// The "Tymoczko" Tetrahedron (Geometric Chord Families)
    fn generate_tetrahedral_chord(sink: &Sink, freqs: Vec<f32>) {
        if freqs.is_empty() { return; }
        let mut source = SineWave::new(freqs[0]).take_duration(Duration::from_millis(500)).amplify(0.3).boxed();
        
        for &f in freqs.iter().skip(1).take(3) {
            let next = SineWave::new(f).take_duration(Duration::from_millis(500)).amplify(0.2).boxed();
            source = source.mix(next).boxed();
        }
        
        sink.append(source);
    }
}
