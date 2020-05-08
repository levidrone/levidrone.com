#[macro_use] mod log;

mod errors;
mod gen;

use std::fs;

use crate::errors::*;

fn render(out_dir: &str) -> Result<()> {
    if fs::metadata(out_dir).is_ok() {
        info!("delete previous output: \"{}\"", out_dir);
        fs::remove_dir_all(out_dir)?;
    }
    info!("create output directory: \"{}\"", out_dir);
    fs::create_dir_all(out_dir)?;

    gen::render_css(out_dir, "main")?;
    gen::render_css(out_dir, "fonts")?;

    gen::render_html(out_dir, "index", "LeviDrone — Unmanned Flying Objects")?;
    gen::render_html(out_dir, "404", "Oops — Page not found")?;

    gen::render_html(out_dir, "why", "Why LeviDrone?")?;
    gen::render_html(out_dir, "about", "About LeviDrone")?;

    gen::render_html(out_dir, "products", "Products")?;
    gen::render_html(out_dir, "products/brixx", "Products — Brixx")?;
    gen::render_html(out_dir, "products/carrier-boards", "Products — Carrier Boards")?;

    gen::copy_static(out_dir, "css")?;
    gen::copy_static(out_dir, "fonts")?;
    gen::copy_static(out_dir, "img")?;

    gen::write_cname(out_dir, "levidrone.com")?;

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