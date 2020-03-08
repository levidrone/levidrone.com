#[macro_use] mod log;

mod errors;

use std::fs::{self, File};
use std::io::Write;

use crate::errors::*;

fn copy_static(out_dir: &str, static_dir: &str) -> Result<()> {
    use fs_extra::dir::{self, CopyOptions};

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    let source_dir = format!("static/{}", static_dir);
    info!("copy static directory: \"{}\"", source_dir);
    dir::copy(source_dir, out_dir, &options)?;

    Ok(())
}

fn render_css(out_dir: &str, name: &str) -> Result<()> {
    let source_file = format!("src/css/{}.scss", name);
    let out_file = format!("{}/css/{}.css", out_dir, name);

    info!("render css: \"{}\" -> \"{}\"", &source_file, &out_file);
    let css = sass_rs::compile_file(&source_file, sass_rs::Options::default())?;

    fs::create_dir(format!("{}/css", out_dir))?;
    let mut file = File::create(out_file)?;
    file.write_all(&css.into_bytes())?;
    Ok(())
}

fn render_html(out_dir: &str, name: &str, title: &str) -> Result<()> {
    let mut hbs = handlebars::Handlebars::new();
    hbs.set_strict_mode(true);
    hbs.register_templates_directory(".hbs", "src/templates")?;

    let data = serde_json::json!({
        "title": title,
        "parent": "layout",
    });

    let out_file = format!("{}/{}.html", out_dir, name);
    info!("render html: \"{}\"", out_file);
    
    let file = File::create(out_file)?;
    hbs.render_to_write(name, &data, file)?;
    Ok(())
}

fn render(out_dir: &str) -> Result<()> {
    if fs::metadata(out_dir).is_ok() {
        info!("delete previous output: \"{}\"", out_dir);
        fs::remove_dir_all(out_dir)?;
    }
    info!("create output directory: \"{}\"", out_dir);
    fs::create_dir_all(out_dir)?;

    render_css(out_dir, "main")?;
    render_html(out_dir, "index", "LeviDrone — Unmanned Flying Objects")?;

    copy_static(out_dir, "css")?;
    copy_static(out_dir, "img")?;
    Ok(())
}

fn main() {
    debug!("running LeviDrone builder in debug mode");
    if let Err(e) = render("site") {
        err!("{}", e);
        for e in e.iter().skip(1) {
            errln!("{}", e);
        }
        std::process::exit(1);
    }
}