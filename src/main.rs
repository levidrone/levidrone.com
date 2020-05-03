#[macro_use] mod log;

mod errors;

use std::fs::{self, File};
use std::io::Write;

use chrono::{Datelike, Utc};

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

    fs::create_dir_all(format!("{}/css", out_dir))?;
    let mut file = File::create(out_file)?;
    file.write_all(&css.into_bytes())?;
    Ok(())
}

fn render_html(out_dir: &str, name: &str, title: &str) -> Result<()> {
    let mut hbs = handlebars::Handlebars::new();
    hbs.set_strict_mode(true);
    hbs.register_templates_directory(".hbs", "src/templates")?;
    hbs.register_templates_directory(".hbs", "src/pages")?;

    let data = serde_json::json!({
        "title": title,
        "parent": "layout",
        "year": Utc::now().year(),

        // Styles
        "body": "center f3 mw-none mw8-m mw9-l ph3 w-100",
        "h1": "b f-5-l f1",
        "h2": "b f1-l f2 pb0",
        "h3": "b f2-l f3 mv2",
        "large": "f2-l f3 mv4",
    });

    let out_file = if name == "index" || name == "404" {
        format!("{}/{}.html", out_dir, name)
    } else {
        fs::create_dir_all(format!("{}/{}", out_dir, name))?;
        format!("{}/{}/index.html", out_dir, name)
    };
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
    render_css(out_dir, "fonts")?;

    render_html(out_dir, "index", "LeviDrone — Unmanned Flying Objects")?;
    render_html(out_dir, "404", "Oops — Page not found")?;


    render_html(out_dir, "why", "Why LeviDrone?")?;
    render_html(out_dir, "about", "About LeviDrone")?;

    render_html(out_dir, "products", "Products")?;
    render_html(out_dir, "products/brixx", "Products — Brixx")?;
    render_html(out_dir, "products/carrier-boards", "Products — Carrier Boards")?;

    copy_static(out_dir, "css")?;
    copy_static(out_dir, "fonts")?;
    copy_static(out_dir, "img")?;

    info!("write CNAME");
    let mut cname = File::create(format!("{}/CNAME", out_dir))?;
    cname.write_all(b"levidrone.com")?;

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