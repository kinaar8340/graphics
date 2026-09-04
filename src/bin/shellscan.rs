//! Phosphor Loom first window. demo-tiny scaffold + one extra mesh (the shell).
//! Live loom off. Gun off. 4k elliptic motes on the trench.

use anyhow::{Context, Result};
use glam::Vec3;
use qga_gpu::{
    hud_text, Camera, GpuContext, GpuParticle, HudVert, LineStyle, Renderer, UploadStats,
    VisualState,
};
use shellscan::{scene, to_gpu_particle_on, Field, Phosphor, Trench, TwoClock, N_MOTES};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

struct Args {
    headless: bool,
    frames: u32,
}

fn parse_args() -> Args {
    let mut headless = false;
    let mut frames = 0;
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--headless" => headless = true,
            "--frames" => {
                frames = it.next().and_then(|s| s.parse().ok()).unwrap_or(8);
            }
            _ => {}
        }
    }
    if headless && frames == 0 {
        frames = 8;
    }
    Args { headless, frames }
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
    let mut ph = Phosphor::on_trench(N_MOTES);
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
            ph: Phosphor::on_trench(N_MOTES),
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
    if args.headless {
        run_headless(args.frames)
    } else {
        run_windowed(args.frames)
    }
}
