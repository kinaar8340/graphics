//! Phase 6 writer. Hemisphere sets (θ,φ). Emits 32-byte `QgaPixel`.
//! Not the faceplate. Not a Philogb gallery. Gun off.

use anyhow::{Context, Result};
use glam::Vec3;
use qga_gpu::{
    hud_quad, hud_text, Camera, FaceVert, GpuContext, GpuFiber, HudVert, LineStyle, Mesh, Renderer,
    VisualState,
};
use qga_math::{hopf_coordinates, stereographic};
use shellscan::{Field, QgaPixel};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

const TAU: f32 = std::f32::consts::TAU;
const HEM_R: f32 = 1.0;

struct Pick {
    pixel: QgaPixel,
    lock4: bool,
    cursor: [f32; 2],
    size: [f32; 2],
}

impl Pick {
    fn new() -> Self {
        Self {
            pixel: QgaPixel::new(0.3, 0.0, 0.0, 0.4, 1.0, Field::Odd, 0.5),
            lock4: true,
            cursor: [0.0, 0.0],
            size: [1280.0, 720.0],
        }
    }

    fn witness_rgb(&self) -> [f32; 3] {
        let mut p = self.pixel;
        p.persist = 1.0;
        if self.lock4 {
            p.rgb_preview()
        } else {
            p.rgb_preview_mix()
        }
    }

    fn export(&self) -> Result<()> {
        let dir = Path::new("output/pick");
        std::fs::create_dir_all(dir)?;
        let mut dump = self.pixel;
        dump.persist = 0.0;
        std::fs::write(dir.join("qga_pixel.bin"), dump.to_bytes())?;
        std::fs::write(dir.join("qga_pixel.json"), dump.to_json())?;
        println!(
            "wrote output/pick/qga_pixel.bin (32 bytes) and qga_pixel.json  section={} field={} layer={}",
            dump.section().name(),
            dump.field().bit(),
            dump.layer()
        );
        Ok(())
    }
}

fn hemisphere_faces() -> Vec<FaceVert> {
    let t = Mesh::sphere(HEM_R)
        .colored([0.22, 0.28, 0.36])
        .tessellate(2);
    t.faces
        .into_iter()
        .filter(|v| v.pos[1] >= -0.02)
        .map(|mut v| {
            v.alpha = 0.55;
            v
        })
        .collect()
}

fn hemisphere_edges() -> Vec<[Vec3; 2]> {
    Mesh::sphere(HEM_R)
        .tessellate(1)
        .edges
        .into_iter()
        .filter(|[a, b]| a.y >= -0.02 && b.y >= -0.02)
        .collect()
}

fn fiber_for(p: QgaPixel) -> GpuFiber {
    let n = 64usize;
    let mut points = Vec::with_capacity(n);
    for i in 0..n {
        let psi = i as f32 / n as f32 * TAU;
        let q = hopf_coordinates(p.theta, p.phi, psi);
        points.push(stereographic(q, 0.85));
    }
    let mut vis = p;
    vis.persist = 1.0;
    let rgb = vis.rgb_preview();
    GpuFiber {
        points,
        color: Vec3::new(rgb[0], rgb[1], rgb[2]),
    }
}

fn pick_hud(pick: &Pick) -> Vec<HudVert> {
    let mut hud = Vec::new();
    let c = [0.92, 0.95, 1.0, 0.92];
    let s = 0.014;
    hud_text(
        &mut hud,
        -0.96,
        0.92,
        s,
        if pick.lock4 { "LOCK4 ON" } else { "LOCK4 OFF" },
        c,
    );
    hud_text(
        &mut hud,
        -0.96,
        0.84,
        s,
        &format!("SHELL {:.2}", pick.pixel.shell_s),
        c,
    );
    hud_text(&mut hud, -0.96, 0.76, s, "E EXPORT", c);
    hud_text(
        &mut hud,
        -0.96,
        -0.72,
        s,
        &format!(
            "{}  FIELD BIT {}  (PACKED NOT A PICTURE)",
            pick.pixel.section().name().to_uppercase(),
            pick.pixel.field().bit()
        ),
        c,
    );
    hud_text(&mut hud, -0.96, -0.80, s, "WITNESS", c);
    let rgb = pick.witness_rgb();
    hud_quad(
        &mut hud,
        -0.72,
        -0.88,
        -0.52,
        -0.76,
        [rgb[0], rgb[1], rgb[2], 1.0],
    );
    hud
}

fn ray_sphere(eye: Vec3, dir: Vec3, radius: f32) -> Option<Vec3> {
    let d = dir.normalize_or_zero();
    let b = 2.0 * eye.dot(d);
    let c = eye.length_squared() - radius * radius;
    let disc = b * b - 4.0 * c;
    if disc < 0.0 {
        return None;
    }
    let s = disc.sqrt();
    let t0 = (-b - s) * 0.5;
    let t1 = (-b + s) * 0.5;
    let t = if t0 > 0.02 { t0 } else { t1 };
    if t <= 0.02 {
        return None;
    }
    Some(eye + d * t)
}

fn hit_to_pixel(hit: Vec3, pick: &mut Pick) {
    let n = hit.normalize_or_zero();
    if n.y < 0.0 {
        return;
    }
    let theta = n.y.clamp(-1.0, 1.0).acos();
    let phi = n.x.atan2(n.z);
    pick.pixel.theta = theta;
    pick.pixel.phi = phi;
    pick.pixel.reclassify();
}

struct App {
    window: Option<Arc<Window>>,
    gpu: Option<GpuContext>,
    renderer: Option<Renderer>,
    camera: Camera,
    vis: VisualState,
    pick: Pick,
    last: Instant,
}

impl App {
    fn new() -> Self {
        let mut camera = Camera::orbit(Vec3::ZERO, 2.8);
        camera.yaw = 1.57;
        camera.pitch = 0.72;
        Self {
            window: None,
            gpu: None,
            renderer: None,
            camera,
            vis: VisualState {
                glow: 0.35,
                pulse: 0.3,
                tube_radius: 0.025,
                ..VisualState::default()
            },
            pick: Pick::new(),
            last: Instant::now(),
        }
    }

    fn boot(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        let attrs = Window::default_attributes()
            .with_title("qga_pixel pick")
            .with_inner_size(winit::dpi::PhysicalSize::new(1280u32, 720u32));
        let window = Arc::new(event_loop.create_window(attrs)?);
        let gpu = GpuContext::init_windowed(window.clone())?;
        log::info!("{}", gpu.report());
        let size = window.inner_size();
        self.camera.aspect = size.width as f32 / size.height.max(1) as f32;
        self.pick.size = [size.width as f32, size.height as f32];
        let mut renderer = Renderer::new(&gpu)?;
        renderer.update_faces(&gpu, &hemisphere_faces());
        renderer.update_line_segments(&gpu, &hemisphere_edges(), LineStyle::black_hairline());
        renderer.write_live_fibers(&gpu, &[fiber_for(self.pick.pixel)], 0.02)?;
        renderer.write_hud(&gpu, &pick_hud(&self.pick))?;
        self.window = Some(window);
        self.gpu = Some(gpu);
        self.renderer = Some(renderer);
        self.last = Instant::now();
        Ok(())
    }

    fn apply_click(&mut self) {
        let (w, h) = (self.pick.size[0].max(1.0), self.pick.size[1].max(1.0));
        let ndc_x = (self.pick.cursor[0] / w) * 2.0 - 1.0;
        let ndc_y = 1.0 - (self.pick.cursor[1] / h) * 2.0;
        let inv = (self.camera.proj() * self.camera.view()).inverse();
        let near = inv.project_point3(Vec3::new(ndc_x, ndc_y, 0.0));
        let far = inv.project_point3(Vec3::new(ndc_x, ndc_y, 1.0));
        let dir = (far - near).normalize_or_zero();
        let eye = self.camera.eye();
        if let Some(hit) = ray_sphere(eye, dir, HEM_R) {
            hit_to_pixel(hit, &mut self.pick);
        }
    }

    fn tick(&mut self) -> Result<()> {
        let dt = self.last.elapsed().as_secs_f32().clamp(0.0, 0.05);
        self.last = Instant::now();
        if self.vis.paused {
            // still present
        } else {
            self.camera.tick_cinematic(dt);
        }
        let gpu = self.gpu.as_mut().context("gpu")?;
        let renderer = self.renderer.as_mut().context("renderer")?;
        renderer.write_live_fibers(gpu, &[fiber_for(self.pick.pixel)], 0.02)?;
        renderer.write_hud(gpu, &pick_hud(&self.pick))?;
        renderer.render(gpu, &self.camera, &self.vis, 0.0, false)?;
        Ok(())
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
                    self.pick.size = [size.width as f32, size.height as f32];
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(e) = self.tick() {
                    log::error!("frame: {e:#}");
                    event_loop.exit();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.pick.cursor = [position.x as f32, position.y as f32];
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => self.apply_click(),
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
                        KeyCode::KeyL => self.pick.lock4 = !self.pick.lock4,
                        KeyCode::KeyF => {
                            let next = if self.pick.pixel.field() == Field::Even {
                                Field::Odd
                            } else {
                                Field::Even
                            };
                            self.pick.pixel.set_field(next);
                        }
                        KeyCode::BracketLeft => {
                            self.pick.pixel.shell_s =
                                (self.pick.pixel.shell_s - 0.02).clamp(0.0, 1.0);
                        }
                        KeyCode::BracketRight => {
                            self.pick.pixel.shell_s =
                                (self.pick.pixel.shell_s + 0.02).clamp(0.0, 1.0);
                        }
                        KeyCode::KeyE => {
                            if let Err(e) = self.pick.export() {
                                log::error!("export: {e:#}");
                            }
                        }
                        KeyCode::KeyC => self.camera.cinematic = !self.camera.cinematic,
                        KeyCode::Space => self.vis.paused = !self.vis.paused,
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

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    if std::env::args().any(|a| a == "--dump") {
        return Pick::new().export();
    }
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
