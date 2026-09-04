//! Two-clock persistence. Hypothesis: this reads as a display, not a demo.
//! Software fact of the CPU clock: composed energy > either field.

use shellscan::{Field, Phosphor, TwoClock};

#[test]
fn compose_exceeds_either_field() {
    let mut ph = Phosphor::interlaced_chart(32, 32);
    let clock = TwoClock::interlaced();
    ph.tick(0, clock); // even write
    ph.tick(1, clock); // odd write
    let even = ph.field_energy(Field::Even);
    let odd = ph.field_energy(Field::Odd);
    let composed = ph.composed_energy();
    assert!(even > 0.0);
    assert!(odd > 0.0);
    assert!(composed > even);
    assert!(composed > odd);
}

#[test]
fn silent_field_decays() {
    let mut ph = Phosphor::interlaced_chart(16, 16);
    let clock = TwoClock::interlaced();
    ph.tick(0, clock);
    let even_after_write = ph.field_energy(Field::Even);
    ph.tick(1, clock); // even is silent
    let even_after_silent = ph.field_energy(Field::Even);
    assert!(even_after_silent < even_after_write);
    assert!(ph.field_energy(Field::Odd) > 0.0);
}

#[test]
fn static_once_live_per_field() {
    let mut ph = Phosphor::interlaced_chart(8, 8);
    let clock = TwoClock::interlaced();
    assert_eq!(ph.static_uploads, 1);
    for f in 0..4 {
        ph.tick(f, clock);
    }
    assert_eq!(ph.static_uploads, 1);
    assert_eq!(ph.live_writes, 4);
    assert_eq!(ph.live_fiber_writes, 2);
}

#[test]
fn elliptic_first_scene_chart() {
    let ph = Phosphor::interlaced_chart(4, 4);
    assert!(ph.elliptic_only());
}

#[test]
fn odd_is_visual_even_is_feeling() {
    assert!(Field::from_frame(1).is_visual());
    assert!(!Field::from_frame(0).is_visual());
}
