//! Phosphor Loom first window. demo-tiny scaffold + one extra mesh (the shell).
//! Live loom off. Gun off. Occupancy interlace on γ. LF=0.

use anyhow::{Context, Result};
use glam::Vec3;
use qga_gpu::{
    hud_text, Camera, GpuContext, GpuParticle, HudVert, LineStyle, Renderer, UploadStats,
    VisualState,
};
use shellscan::scan::{
    apply_scan, energy_json, head_index, measure, odd_sites_dark, SCAN_DELTA, SCAN_EVEN_LAP,
    SCAN_FRAMES, SCAN_I0, SCAN_K_TAIL,
};
use shellscan::nest::{self, NEST_N};
use shellscan::{
    capture, scene, to_gpu_particle_on, Field, Phosphor, Trench, TwoClock, NEST_DELTA_R,
    NEST_LAYERS, N_OCCUPANCY, SPLAT_LOCK,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

/// Locked three-quarter for 00–03. Software fact of the testimony eye.
const EYE_YAW: f32 = 0.55;
const EYE_PITCH: f32 = 0.42;
const EYE_DIST: f32 = 4.2;

struct Args {
    headless: bool,
    frames: u32,
    stills: bool,
    orbit: bool,
    tick: bool,
    scan: bool,
    nest_headless: bool,
    nest_stills: bool,
    width: u32,
    height: u32,
}

fn parse_args() -> Args {
    let mut headless = false;
    let mut frames = 0;
    let mut stills = false;
    let mut orbit = false;
    let mut tick = false;
    let mut scan = false;
    let mut nest_headless = false;
    let mut nest_stills = false;
    let mut width = 0;
    let mut height = 0;
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--headless" => headless = true,
            "--stills" => {
                stills = true;
                headless = true;
            }
            "--orbit" => {
                orbit = true;
                headless = true;
            }
            "--tick" => {
                tick = true;
                headless = true;
            }
            "--scan" => {
                scan = true;
                headless = true;
            }
            "--nest-headless" => {
                nest_headless = true;
                headless = true;
            }
            "--nest-stills" => {
                nest_stills = true;
                headless = true;
            }
            "--frames" => {
                frames = it.next().and_then(|s| s.parse().ok()).unwrap_or(8);
            }
            "--width" => {
                width = it.next().and_then(|s| s.parse().ok()).unwrap_or(1280);
            }
            "--height" => {
                height = it.next().and_then(|s| s.parse().ok()).unwrap_or(720);
            }
            _ => {}
        }
    }
    if stills || orbit || tick || scan || nest_stills {
        if width == 0 {
            width = 1280;
        }
        if height == 0 {
            height = 720;
        }
    }
    if headless && frames == 0 {
        frames = if orbit {
            900
        } else if tick {
            180
        } else if scan {
            SCAN_FRAMES
        } else {
            8
        };
    }
    Args {
        headless,
        frames,
        stills,
        orbit,
        tick,
        scan,
        nest_headless,
        nest_stills,
        width,
        height,
    }
}

fn trench_path() -> PathBuf {
    if let Ok(p) = std::env::var("SHELLSCAN_TRENCH") {
        return PathBuf::from(p);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/shell_trench.bin")
}

fn load_trench() -> Result<Trench> {
    Trench::load(trench_path()).context("offline shell_trench.bin — run make export-shell")
}

fn pack(ph: &Phosphor, trench: &Trench) -> Vec<GpuParticle> {
    ph.pixels
        .iter()
        .map(|p| to_gpu_particle_on(*p, Some(trench)))
        .collect()
}

fn pack_lit(ph: &Phosphor, trench: &Trench) -> Vec<GpuParticle> {
    ph.pixels
        .iter()
        .filter(|p| p.persist > 1e-3)
        .map(|p| to_gpu_particle_on(*p, Some(trench)))
        .collect()
}

fn locked_eye(aspect: f32) -> Camera {
    let mut c = Camera::orbit(Vec3::ZERO, EYE_DIST);
    c.yaw = EYE_YAW;
    c.pitch = EYE_PITCH;
    c.aspect = aspect;
    c.cinematic = false;
    c
}

fn trench_eye(aspect: f32, target: Vec3) -> Camera {
    let mut c = Camera::orbit(target, 1.05);
    c.yaw = 1.05;
    c.pitch = 0.16;
    c.aspect = aspect;
    c.cinematic = false;
    c
}

fn silent_vis() -> VisualState {
    VisualState {
        glow: 0.0,
        pulse: 0.45,
        tube_radius: 0.03,
        ..VisualState::default()
    }
}

fn hold_field(ph: &mut Phosphor, field: Field, writes: u32) {
    let clock = TwoClock::windowed();
    let frame = field.bit();
    for _ in 0..writes {
        ph.tick(frame, clock);
    }
}

fn hold_both_8(ph: &mut Phosphor) {
    let clock = TwoClock::windowed();
    for i in 0..8 {
        ph.tick(i, clock);
    }
}

fn field_mass(ph: &Phosphor, gpu: &[GpuParticle], field: Field) -> f32 {
    ph.pixels
        .iter()
        .zip(gpu)
        .filter(|(p, _)| p.field() == field)
        .map(|(_, g)| g.mass)
        .sum()
}

fn hud_verts(field: Field, stats: UploadStats) -> Vec<HudVert> {
    let mut hud = Vec::new();
    let c = [0.92, 0.95, 1.0, 0.92];
    let s = 0.016;
    hud_text(
        &mut hud,
        -0.94,
        0.90,
        s,
        &format!("FIELD {}", field.bit()),
        c,
    );
    hud_text(&mut hud, -0.94, 0.82, s, "ELLIPTIC", c);
    hud_text(
        &mut hud,
        -0.94,
        0.74,
        s,
        &format!(
            "SU={} LF={} PS={}",
            stats.static_uploads, stats.live_fiber_writes, stats.particle_skipped
        ),
        c,
    );
    hud
}

fn upload_static(gpu: &GpuContext, renderer: &mut Renderer, trench: &Trench) -> Result<()> {
    let hull = scene::static_hull(trench, 1);
    renderer.update_faces(gpu, &hull.faces);
    renderer.update_line_segments(gpu, &hull.edges, LineStyle::black_hairline());
    renderer.retain_static_fibers(gpu, &hull.fibers, 0.03)?;
    Ok(())
}

fn accept_headless(
    frames: u32,
    ph: &Phosphor,
    gpu_parts: &[GpuParticle],
    stats: UploadStats,
) -> Result<()> {
    anyhow::ensure!(
        stats.static_uploads == 1,
        "static_uploads={} expected 1",
        stats.static_uploads
    );
    anyhow::ensure!(ph.elliptic_only(), "first window is elliptic only");
    let even = field_mass(ph, gpu_parts, Field::Even);
    let odd = field_mass(ph, gpu_parts, Field::Odd);
    let both: f32 = gpu_parts.iter().map(|g| g.mass).sum();
    anyhow::ensure!(
        both > even && both > odd,
        "composed mass {both} even {even} odd {odd}"
    );
    anyhow::ensure!(
        gpu_parts.iter().all(|g| (g.pad - 0.55).abs() < 1e-4),
        "palette must stay cyan / elliptic 0.55"
    );
    println!(
        "done frames={frames} static_uploads={} live_fiber_writes={} particle_skipped={} even={even:.3} odd={odd:.3} both={both:.3}",
        stats.static_uploads, stats.live_fiber_writes, stats.particle_skipped
    );
    Ok(())
}

fn grab_png(
    gpu: &mut GpuContext,
    renderer: &mut Renderer,
    camera: &Camera,
    vis: &VisualState,
    time: f32,
    path: &Path,
) -> Result<()> {
    grab_png_opt(gpu, renderer, camera, vis, time, path, true)
}

fn grab_png_opt(
    gpu: &mut GpuContext,
    renderer: &mut Renderer,
    camera: &Camera,
    vis: &VisualState,
    time: f32,
    path: &Path,
    log: bool,
) -> Result<()> {
    let frame = renderer
        .render(gpu, camera, vis, time, true)?
        .context("capture empty")?;
    capture::save_png(path, frame.width, frame.height, &frame.bgra)?;
    if log {
        println!("wrote {} {}x{}", path.display(), frame.width, frame.height);
    }
    Ok(())
}

fn run_stills(width: u32, height: u32) -> Result<()> {
    let trench = load_trench()?;
    let mut gpu = GpuContext::init_headless_extent(width, height).context("init_headless")?;
    println!("{}", gpu.report());
    let mut renderer = Renderer::new(&gpu)?;
    let vis = silent_vis();
    let eye = locked_eye(width as f32 / height.max(1) as f32);
    upload_static(&gpu, &mut renderer, &trench)?;
    renderer.write_hud(&gpu, &[])?;

    // 00 — envelope, zero motes.
    renderer.write_particles(&gpu, &[])?;
    grab_png(
        &mut gpu,
        &mut renderer,
        &eye,
        &vis,
        0.0,
        Path::new("output/png/00_hull.png"),
    )?;

    // 01 — even / feeling only.
    let mut even = Phosphor::on_trench(N_OCCUPANCY);
    hold_field(&mut even, Field::Even, 4);
    renderer.write_particles(&gpu, &pack_lit(&even, &trench))?;
    grab_png(
        &mut gpu,
        &mut renderer,
        &eye,
        &vis,
        0.0,
        Path::new("output/png/01_even.png"),
    )?;

    // 02 — odd / visual only.
    let mut odd = Phosphor::on_trench(N_OCCUPANCY);
    hold_field(&mut odd, Field::Odd, 4);
    renderer.write_particles(&gpu, &pack_lit(&odd, &trench))?;
    grab_png(
        &mut gpu,
        &mut renderer,
        &eye,
        &vis,
        0.0,
        Path::new("output/png/02_odd.png"),
    )?;

    // 03 — both fields, same 8-frame path as Phase 3.
    let mut both = Phosphor::on_trench(N_OCCUPANCY);
    hold_both_8(&mut both);
    let parts = pack_lit(&both, &trench);
    let e = field_mass(&both, &pack(&both, &trench), Field::Even);
    let o = field_mass(&both, &pack(&both, &trench), Field::Odd);
    let b: f32 = pack(&both, &trench).iter().map(|g| g.mass).sum();
    println!("03_both energy even={e:.3} odd={o:.3} both={b:.3} (odd-hot: frame 7 is odd)");
    renderer.write_particles(&gpu, &parts)?;
    grab_png(
        &mut gpu,
        &mut renderer,
        &eye,
        &vis,
        0.0,
        Path::new("output/png/03_both.png"),
    )?;

    // 04 — grazing trench. Same phosphor as 03. Aim at γ(s), not the origin.
    let graze = trench_eye(width as f32 / height.max(1) as f32, trench.gamma(0.18));
    grab_png(
        &mut gpu,
        &mut renderer,
        &graze,
        &vis,
        0.0,
        Path::new("output/png/04_trench.png"),
    )?;

    anyhow::ensure!(
        renderer.upload_stats().static_uploads == 1,
        "stills must keep static_uploads == 1"
    );
    Ok(())
}

/// Locked-eye tick. 2s field 0, 2s field 1, 2s both. Hard cuts. No orbit.
fn run_tick(width: u32, height: u32) -> Result<()> {
    const FPS: u32 = 30;
    const HOLD: u32 = 2 * FPS;
    let trench = load_trench()?;
    let mut gpu = GpuContext::init_headless_extent(width, height).context("init_headless")?;
    println!("{}", gpu.report());
    let mut renderer = Renderer::new(&gpu)?;
    let vis = silent_vis();
    let eye = locked_eye(width as f32 / height.max(1) as f32);
    upload_static(&gpu, &mut renderer, &trench)?;
    renderer.write_hud(&gpu, &[])?;

    let mut even = Phosphor::on_trench(N_OCCUPANCY);
    hold_field(&mut even, Field::Even, 4);
    let mut odd = Phosphor::on_trench(N_OCCUPANCY);
    hold_field(&mut odd, Field::Odd, 4);
    let mut both = Phosphor::on_trench(N_OCCUPANCY);
    hold_both_8(&mut both);
    let packs = [
        pack_lit(&even, &trench),
        pack_lit(&odd, &trench),
        pack_lit(&both, &trench),
    ];

    let dir = Path::new("output/png/tick");
    std::fs::create_dir_all(dir)?;
    let mut n = 0u32;
    for (seg, parts) in packs.iter().enumerate() {
        renderer.write_particles(&gpu, parts)?;
        for _ in 0..HOLD {
            let path = dir.join(format!("frame_{:04}.png", n));
            grab_png(
                &mut gpu,
                &mut renderer,
                &eye,
                &vis,
                n as f32 / FPS as f32,
                &path,
            )?;
            n += 1;
        }
        println!("tick segment {seg} frames={}", HOLD);
    }
    anyhow::ensure!(n == 3 * HOLD, "tick length {n}");
    anyhow::ensure!(
        renderer.upload_stats().static_uploads == 1,
        "tick must keep static_uploads == 1"
    );
    anyhow::ensure!(
        renderer.upload_stats().live_fiber_writes == 0,
        "tick must keep LF=0"
    );
    println!("tick frames={n} static_uploads=1 live_fiber_writes=0");
    Ok(())
}

/// Animation A. One persist peak on even occupancy sites. LF=0. Glow off. HUD off.
fn run_scan(width: u32, height: u32) -> Result<()> {
    let trench = load_trench()?;
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    anyhow::ensure!(ph.pixels.len() == N_OCCUPANCY, "occupancy table frozen at 256");
    anyhow::ensure!(ph.elliptic_only(), "scan stays elliptic");

    let mut rows = Vec::with_capacity(SCAN_FRAMES as usize);
    for t in 0..SCAN_FRAMES {
        let hi = head_index(t, SCAN_I0);
        apply_scan(&mut ph, hi);
        anyhow::ensure!(ph.elliptic_only(), "section flipped at t={t}");
        anyhow::ensure!(odd_sites_dark(&ph), "odd sites lit at t={t}");
        let p = ph.pixels[hi];
        anyhow::ensure!(
            (p.shell_s - hi as f32 / N_OCCUPANCY as f32).abs() < 1e-5,
            "head shell_s left occupancy sample"
        );
        let g = trench.gamma(p.shell_s) + Field::Even.cone_axis() * shellscan::RAIL_EPS;
        anyhow::ensure!(
            p.bind_shell(&trench).distance(g) < 1e-5,
            "head left the trench"
        );
        let e = measure(&ph, t, hi, SCAN_K_TAIL);
        anyhow::ensure!(e.other_frac() < 1e-4, "energy_other/total={}", e.other_frac());
        anyhow::ensure!(
            e.peak_is_peak(SCAN_K_TAIL),
            "peak is not a peak head={} tail/K={}",
            e.energy_head,
            e.energy_tail / SCAN_K_TAIL as f32
        );
        rows.push(e);
    }
    let lap = SCAN_EVEN_LAP as usize;
    anyhow::ensure!(rows[0].head_i == rows[lap].head_i);
    anyhow::ensure!((rows[0].energy_head - rows[lap].energy_head).abs() < 1e-5);
    anyhow::ensure!((rows[0].energy_tail - rows[lap].energy_tail).abs() < 1e-5);

    let energy_dir = Path::new("output/scan");
    std::fs::create_dir_all(energy_dir)?;
    let energy_path = energy_dir.join("energy.json");
    std::fs::write(&energy_path, energy_json(&rows))?;
    println!("wrote {}", energy_path.display());

    let e0 = rows[0];
    println!(
        "scan energy N={N_OCCUPANCY} K={SCAN_K_TAIL} delta={SCAN_DELTA}"
    );
    for e in rows.iter().take(8) {
        println!(
            " t={:<3} head_i={:<3} head_s={:.6} energy_head={:.3} energy_tail={:.3} energy_other={:.3}",
            e.t, e.head_i, e.head_s, e.energy_head, e.energy_tail, e.energy_other
        );
    }

    let mut gpu = GpuContext::init_headless_extent(width, height).context("init_headless")?;
    println!("{}", gpu.report());
    let mut renderer = Renderer::new(&gpu)?;
    let vis = silent_vis();
    let aspect = width as f32 / height.max(1) as f32;
    let eye = locked_eye(aspect);
    let graze = trench_eye(aspect, trench.gamma(0.18));
    upload_static(&gpu, &mut renderer, &trench)?;
    renderer.write_hud(&gpu, &[])?;

    let lock_dir = Path::new("output/png/scan_lock");
    let crop_dir = Path::new("output/png/scan_crop");
    std::fs::create_dir_all(lock_dir)?;
    std::fs::create_dir_all(crop_dir)?;

    for t in 0..SCAN_FRAMES {
        let hi = head_index(t, SCAN_I0);
        apply_scan(&mut ph, hi);
        renderer.write_particles(&gpu, &pack_lit(&ph, &trench))?;
        let log = t % 32 == 0 || t + 1 == SCAN_FRAMES;
        grab_png_opt(
            &mut gpu,
            &mut renderer,
            &eye,
            &vis,
            0.0,
            &lock_dir.join(format!("frame_{:04}.png", t)),
            log,
        )?;
        grab_png_opt(
            &mut gpu,
            &mut renderer,
            &graze,
            &vis,
            0.0,
            &crop_dir.join(format!("frame_{:04}.png", t)),
            log,
        )?;
        if log {
            println!("scan frame {t}/{}", SCAN_FRAMES - 1);
        }
    }

    let stats = renderer.upload_stats();
    anyhow::ensure!(stats.static_uploads == 1, "scan SU={}", stats.static_uploads);
    anyhow::ensure!(
        stats.live_fiber_writes == 0,
        "scan LF={}",
        stats.live_fiber_writes
    );
    anyhow::ensure!(
        stats.particle_skipped == 0,
        "scan PS={} (persist must dirty every frame)",
        stats.particle_skipped
    );
    println!(
        "scan: SU={} LF={} PS={} N={N_OCCUPANCY} K={SCAN_K_TAIL} delta={SCAN_DELTA}",
        stats.static_uploads, stats.live_fiber_writes, stats.particle_skipped
    );
    println!(
        "head_i={}  head_s={:.6}  energy_head={:.3}  energy_tail={:.3}  energy_other≈{:.3}",
        e0.head_i, e0.head_s, e0.energy_head, e0.energy_tail, e0.energy_other
    );
    Ok(())
}

/// N1 ledger. L=3, ΔR=0.08R. Same trench table. No PNG.
fn run_nest_headless() -> Result<()> {
    let trench = load_trench()?;
    let mut ph = Phosphor::even_layers(NEST_N, NEST_LAYERS);
    ph.light_persist(1.0);
    anyhow::ensure!(ph.elliptic_only(), "nest stays elliptic");
    anyhow::ensure!(nest::shell_s_not_folded(&ph), "layer folded into shell_s");
    anyhow::ensure!(nest::layers_on_radius(&ph, &trench), "mote left its radius");
    let r = nest::measure(&ph, &trench);
    anyhow::ensure!(
        (r.sep_01 - NEST_DELTA_R).abs() < 1e-3,
        "sep_01={} want {NEST_DELTA_R}",
        r.sep_01
    );
    anyhow::ensure!(
        (r.sep_12 - NEST_DELTA_R).abs() < 1e-3,
        "sep_12={} want {NEST_DELTA_R}",
        r.sep_12
    );
    anyhow::ensure!(r.sep_01 > SPLAT_LOCK, "sep_01 {} <= splat", r.sep_01);
    anyhow::ensure!(r.sep_12 > SPLAT_LOCK, "sep_12 {} <= splat", r.sep_12);
    let sum = r.energy_sum();
    anyhow::ensure!(
        (ph.composed_energy() - sum).abs() < 1e-3,
        "all {} != sum {sum}",
        ph.composed_energy()
    );

    let dir = Path::new("output/nest");
    std::fs::create_dir_all(dir)?;
    let energy_path = dir.join("energy.json");
    std::fs::write(&energy_path, nest::energy_json(r))?;
    println!("wrote {}", energy_path.display());

    let mut gpu = GpuContext::init_headless().context("init_headless")?;
    println!("{}", gpu.report());
    let mut renderer = Renderer::new(&gpu)?;
    let camera = locked_eye(16.0 / 9.0);
    let vis = silent_vis();
    upload_static(&gpu, &mut renderer, &trench)?;
    renderer.write_hud(&gpu, &[])?;
    renderer.write_particles(&gpu, &pack_lit(&ph, &trench))?;
    renderer.render(&mut gpu, &camera, &vis, 0.0, true)?;
    let stats = renderer.upload_stats();
    anyhow::ensure!(stats.static_uploads == 1, "nest SU={}", stats.static_uploads);
    anyhow::ensure!(
        stats.live_fiber_writes == 0,
        "nest LF={}",
        stats.live_fiber_writes
    );
    println!(
        "nest: L={NEST_LAYERS} dR={NEST_DELTA_R}R splat={SPLAT_LOCK}R SU={} LF={}",
        stats.static_uploads, stats.live_fiber_writes
    );
    println!(
        "sep_01={:.3} sep_12={:.3}   # min |p_ℓ - p_ℓ+1| / R",
        r.sep_01, r.sep_12
    );
    println!(
        "energy_L0={:.3} energy_L1={:.3} energy_L2={:.3}",
        r.energy_l0, r.energy_l1, r.energy_l2
    );
    Ok(())
}

/// N1 stills. Same locked eye as 00–03. Glow off. HUD off. LF=0.
fn run_nest_stills(width: u32, height: u32) -> Result<()> {
    let trench = load_trench()?;
    let mut gpu = GpuContext::init_headless_extent(width, height).context("init_headless")?;
    println!("{}", gpu.report());
    let mut renderer = Renderer::new(&gpu)?;
    let vis = silent_vis();
    let aspect = width as f32 / height.max(1) as f32;
    let eye = locked_eye(aspect);
    let graze = trench_eye(aspect, trench.gamma(0.18));
    upload_static(&gpu, &mut renderer, &trench)?;
    renderer.write_hud(&gpu, &[])?;

    renderer.write_particles(&gpu, &[])?;
    grab_png(
        &mut gpu,
        &mut renderer,
        &eye,
        &vis,
        0.0,
        Path::new("output/png/n0_hull.png"),
    )?;

    for ell in 0..NEST_LAYERS {
        let mut ph = Phosphor::even_layer(NEST_N, ell);
        ph.light_persist(1.0);
        renderer.write_particles(&gpu, &pack_lit(&ph, &trench))?;
        grab_png(
            &mut gpu,
            &mut renderer,
            &eye,
            &vis,
            0.0,
            Path::new(&format!("output/png/n1_L{ell}.png")),
        )?;
    }

    let mut all = Phosphor::even_layers(NEST_N, NEST_LAYERS);
    all.light_persist(1.0);
    let e0 = all.layer_energy(0);
    let e1 = all.layer_energy(1);
    let e2 = all.layer_energy(2);
    println!(
        "n1_all energy L0={e0:.3} L1={e1:.3} L2={e2:.3} sum={:.3}",
        e0 + e1 + e2
    );
    renderer.write_particles(&gpu, &pack_lit(&all, &trench))?;
    grab_png(
        &mut gpu,
        &mut renderer,
        &eye,
        &vis,
        0.0,
        Path::new("output/png/n1_all.png"),
    )?;
    grab_png(
        &mut gpu,
        &mut renderer,
        &graze,
        &vis,
        0.0,
        Path::new("output/png/n1_crop.png"),
    )?;

    anyhow::ensure!(
        renderer.upload_stats().static_uploads == 1,
        "nest stills SU"
    );
    anyhow::ensure!(
        renderer.upload_stats().live_fiber_writes == 0,
        "nest stills LF"
    );
    Ok(())
}

fn run_orbit(frames: u32, width: u32, height: u32) -> Result<()> {
    let trench = load_trench()?;
    let mut gpu = GpuContext::init_headless_extent(width, height).context("init_headless")?;
    println!("{}", gpu.report());
    let mut renderer = Renderer::new(&gpu)?;
    let vis = silent_vis();
    let mut camera = locked_eye(width as f32 / height.max(1) as f32);
    camera.cinematic = true;
    upload_static(&gpu, &mut renderer, &trench)?;
    renderer.write_hud(&gpu, &[])?;
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    hold_both_8(&mut ph);
    let clock = TwoClock::windowed();
    let n = frames.max(1);
    let dir = Path::new("output/png/orbit");
    std::fs::create_dir_all(dir)?;
    for i in 0..n {
        ph.tick(8 + i, clock);
        camera.tick_cinematic(1.0 / 30.0);
        renderer.write_particles(&gpu, &pack_lit(&ph, &trench))?;
        let path = dir.join(format!("frame_{:04}.png", i));
        grab_png(
            &mut gpu,
            &mut renderer,
            &camera,
            &vis,
            i as f32 / 30.0,
            &path,
        )?;
    }
    anyhow::ensure!(
        renderer.upload_stats().static_uploads == 1,
        "orbit must keep static_uploads == 1"
    );
    println!("orbit frames={n} static_uploads=1 live_fiber_writes=0");
    Ok(())
}

fn run_headless(frames: u32) -> Result<()> {
    let trench = load_trench()?;
    let mut gpu = GpuContext::init_headless().context("init_headless")?;
    println!("{}", gpu.report());
    let mut renderer = Renderer::new(&gpu)?;
    let camera = Camera::orbit(Vec3::ZERO, 4.2);
    let vis = VisualState {
        glow: 0.9,
        pulse: 0.45,
        tube_radius: 0.03,
        ..VisualState::default()
    };
    upload_static(&gpu, &mut renderer, &trench)?;
    let mut ph = Phosphor::on_trench(N_OCCUPANCY);
    let clock = TwoClock::windowed();
    let n = frames.max(1);
    let mut last = Vec::new();
    for i in 0..n {
        ph.tick(i, clock);
        last = pack(&ph, &trench);
        renderer.write_particles(&gpu, &last)?;
        let hud = hud_verts(clock.field(i), renderer.upload_stats());
        renderer.write_hud(&gpu, &hud)?;
        let grab = i == 0 || i + 1 == n;
        renderer.render(&mut gpu, &camera, &vis, i as f32 * 0.016, grab)?;
    }
    accept_headless(n, &ph, &last, renderer.upload_stats())
}

struct App {
    window: Option<Arc<Window>>,
    gpu: Option<GpuContext>,
    renderer: Option<Renderer>,
    camera: Camera,
    vis: VisualState,
    trench: Trench,
    ph: Phosphor,
    clock: TwoClock,
    last: Instant,
    time: f32,
    lmb: bool,
    cursor: [f32; 2],
    frame_limit: u32,
    frames_drawn: u32,
}

impl App {
    fn new(trench: Trench, frame_limit: u32) -> Self {
        Self {
            window: None,
            gpu: None,
            renderer: None,
            camera: Camera::orbit(Vec3::ZERO, 4.2),
            vis: VisualState {
                glow: 0.9,
                pulse: 0.45,
                tube_radius: 0.03,
                ..VisualState::default()
            },
            trench,
            ph: Phosphor::on_trench(N_OCCUPANCY),
            clock: TwoClock::windowed(),
            last: Instant::now(),
            time: 0.0,
            lmb: false,
            cursor: [0.0, 0.0],
            frame_limit,
            frames_drawn: 0,
        }
    }

    fn boot(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        let attrs = Window::default_attributes()
            .with_title("Phosphor Loom")
            .with_inner_size(winit::dpi::PhysicalSize::new(1280u32, 720u32));
        let window = Arc::new(event_loop.create_window(attrs)?);
        let gpu = GpuContext::init_windowed(window.clone())?;
        log::info!("{}", gpu.report());
        let size = window.inner_size();
        self.camera.aspect = size.width as f32 / size.height.max(1) as f32;
        let mut renderer = Renderer::new(&gpu)?;
        upload_static(&gpu, &mut renderer, &self.trench)?;
        let parts = pack(&self.ph, &self.trench);
        renderer.write_particles(&gpu, &parts)?;
        self.window = Some(window);
        self.gpu = Some(gpu);
        self.renderer = Some(renderer);
        self.last = Instant::now();
        Ok(())
    }

    fn tick(&mut self) -> Result<bool> {
        let dt = self.last.elapsed().as_secs_f32().clamp(0.0, 0.05);
        self.last = Instant::now();
        if !self.vis.paused {
            self.time += dt;
            self.camera.tick_cinematic(dt);
            self.ph.tick(self.frames_drawn, self.clock);
        }
        let gpu = self.gpu.as_mut().context("gpu")?;
        let renderer = self.renderer.as_mut().context("renderer")?;
        let parts = pack(&self.ph, &self.trench);
        renderer.write_particles(gpu, &parts)?;
        let hud = hud_verts(self.clock.field(self.frames_drawn), renderer.upload_stats());
        renderer.write_hud(gpu, &hud)?;
        renderer.render(gpu, &self.camera, &self.vis, self.time, false)?;
        self.frames_drawn += 1;
        if self.frame_limit > 0 && self.frames_drawn >= self.frame_limit {
            accept_headless(self.frames_drawn, &self.ph, &parts, renderer.upload_stats())?;
            return Ok(false);
        }
        Ok(true)
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            if let Err(e) = self.boot(event_loop) {
                log::error!("boot: {e:#}");
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(gpu) = self.gpu.as_mut() {
                    gpu.resize(size.width, size.height);
                    self.camera.aspect = size.width as f32 / size.height.max(1) as f32;
                }
            }
            WindowEvent::RedrawRequested => match self.tick() {
                Ok(true) => {}
                Ok(false) => event_loop.exit(),
                Err(e) => {
                    log::error!("frame: {e:#}");
                    event_loop.exit();
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    self.lmb = state == ElementState::Pressed;
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let x = position.x as f32;
                let y = position.y as f32;
                if self.lmb {
                    self.camera
                        .orbit_delta(x - self.cursor[0], y - self.cursor[1]);
                }
                self.cursor = [x, y];
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let d = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32 * 0.05,
                };
                self.camera.zoom(d);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }
                if let PhysicalKey::Code(code) = event.physical_key {
                    match code {
                        KeyCode::Escape => event_loop.exit(),
                        KeyCode::Space => self.vis.paused = !self.vis.paused,
                        KeyCode::KeyC => self.camera.cinematic = !self.camera.cinematic,
                        KeyCode::KeyG => {
                            self.vis.glow = if self.vis.glow > 0.5 { 0.2 } else { 0.9 }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(w) = self.window.as_ref() {
            w.request_redraw();
        }
    }
}

fn run_windowed(frames: u32) -> Result<()> {
    let trench = load_trench()?;
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(trench, frames);
    event_loop.run_app(&mut app)?;
    Ok(())
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = parse_args();
    if args.stills {
        run_stills(args.width.max(1280), args.height.max(720))
    } else if args.nest_stills {
        run_nest_stills(args.width.max(1280), args.height.max(720))
    } else if args.nest_headless {
        run_nest_headless()
    } else if args.scan {
        run_scan(args.width.max(1280), args.height.max(720))
    } else if args.tick {
        run_tick(args.width.max(1280), args.height.max(720))
    } else if args.orbit {
        run_orbit(args.frames, args.width.max(1280), args.height.max(720))
    } else if args.headless {
        run_headless(args.frames)
    } else {
        run_windowed(args.frames)
    }
}
