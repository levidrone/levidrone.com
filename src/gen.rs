use std::fs::{self, File};
use std::io::Write;

use chrono::{Datelike, Utc};

use crate::errors::*;

pub fn copy_static(out_dir: &str, static_dir: &str) -> Result<()> {
    use fs_extra::dir::{self, CopyOptions};

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    let source_dir = format!("static/{}", static_dir);
    info!("copy static directory: \"{}\"", source_dir);
    dir::copy(source_dir, out_dir, &options)?;

    Ok(())
}

pub fn render_css(out_dir: &str, name: &str) -> Result<()> {
    let source_file = format!("src/css/{}.scss", name);
    let out_file = format!("{}/css/{}.css", out_dir, name);

    info!("render css: \"{}\" -> \"{}\"", &source_file, &out_file);
    let css = sass_rs::compile_file(&source_file, sass_rs::Options::default())?;

    fs::create_dir_all(format!("{}/css", out_dir))?;
    let mut file = File::create(out_file)?;
    file.write_all(&css.into_bytes())?;
    Ok(())
}

pub fn render_html(out_dir: &str, name: &str, title: &str) -> Result<()> {
    let mut hbs = handlebars::Handlebars::new();
    hbs.set_strict_mode(true);
    hbs.register_templates_directory(".hbs", "src/templates")?;
    hbs.register_templates_directory(".hbs", "src/pages")?;

    let data = serde_json::json!({
        "title": title,
        "parent": "layout",
        "year": Utc::now().year(),

        // Styles
        "body": "center f4 mw-none mw7-m mw8-l ph3 w-100",
        "h1": "b f1",
        "h2": "b f2 pb0",
        "h3": "b f3 mv2",
        "large": "f3 mv4",
        "text": "lh-copy",
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

pub fn write_cname(out_dir: &str, domain: &str) -> Result<()> {
    info!("write CNAME -> \"{}\"", domain);
    let mut cname = File::create(format!("{}/CNAME", out_dir))?;
    write!(cname, "{}", domain)?;
    Ok(())
}