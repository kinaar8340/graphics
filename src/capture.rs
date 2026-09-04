//! PNG stills via ffmpeg (BGRA off the GPU). Same path as inner_cone, not a dep.

use anyhow::{bail, Context, Result};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn save_png(path: &Path, width: u32, height: u32, bgra: &[u8]) -> Result<PathBuf> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("mkdir {}", parent.display()))?;
    }
    let mut child = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "bgra",
            "-s",
            &format!("{width}x{height}"),
            "-i",
            "pipe:0",
            "-frames:v",
            "1",
            path.to_str().unwrap(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .context("spawn ffmpeg for PNG (is ffmpeg installed?)")?;
    {
        let mut stdin = child.stdin.take().context("ffmpeg stdin")?;
        stdin.write_all(bgra).context("write PNG frame")?;
    }
    let out = child.wait_with_output().context("ffmpeg png wait")?;
    if !out.status.success() {
        bail!(
            "ffmpeg png failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(path.to_path_buf())
}
