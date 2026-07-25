//! Shared HTTP fetch + on-disk cache for Arnis-hosted archetype models.

use reqwest::blocking::ClientBuilder;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

const CACHE_SUBDIR: &str = "arnis/custom_models";
const REQUEST_TIMEOUT_SECS: u64 = 20;
const MAX_GLB_BYTES: u64 = 16 * 1024 * 1024;

pub(crate) fn cache_root() -> PathBuf {
    dirs::cache_dir()
        .map(|d| d.join(CACHE_SUBDIR))
        .unwrap_or_else(|| PathBuf::from("./.arnis_custom_cache"))
}

pub(super) fn fetch_glb(_upstream_url: &str, filename: &str) -> Result<Vec<u8>, String> {
    // Fork note (ComboCraft World Builder): the archetype .glb models are
    // hosted on upstream's server (arnismc.com) and this fork must not fetch
    // from there. Custom archetypes (stadium/plane) stay disabled and render
    // procedurally, unless you host the models yourself and point
    // COMBOCRAFT_CUSTOM_MODELS_URL_BASE at your own server (URL = base/file).
    let Ok(url_base) = std::env::var("COMBOCRAFT_CUSTOM_MODELS_URL_BASE") else {
        return Err(
            "custom archetype models disabled in ComboCraft World Builder fork \
             (set COMBOCRAFT_CUSTOM_MODELS_URL_BASE to your own model host to enable)"
                .to_string(),
        );
    };
    let url = format!("{}/{}", url_base.trim_end_matches('/'), filename);

    let dir = cache_root();
    let path = dir.join(filename);
    if let Ok(bytes) = fs::read(&path) {
        if !bytes.is_empty() {
            return Ok(bytes);
        }
    }

    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .user_agent(concat!(
            "ComboCraftWorldBuilder/",
            env!("CARGO_PKG_VERSION"),
            " (+https://github.com/Flolu82/combocraft-world-builder)"
        ))
        .build()
        .map_err(|e| e.to_string())?;
    let mut resp = client.get(&url).send().map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }
    if let Some(len) = resp.content_length() {
        if len > MAX_GLB_BYTES {
            return Err(format!(
                "exceeds {MAX_GLB_BYTES}-byte cap (advertised {len})"
            ));
        }
    }
    let mut buf: Vec<u8> = Vec::new();
    let mut taken = (&mut resp).take(MAX_GLB_BYTES + 1);
    taken.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    if buf.len() as u64 > MAX_GLB_BYTES {
        return Err(format!("exceeds {MAX_GLB_BYTES}-byte cap"));
    }
    let _ = fs::create_dir_all(&dir);
    let _ = fs::write(&path, &buf);
    Ok(buf)
}
