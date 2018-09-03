// use book::{Book, BookItem};
// use config::{Config};

mod config;

use errors::*;
use renderer::{RenderContext, Renderer};
use std::fs;
use utils;

/// The PDFREnderer. Requires a flat Single Page MD Preprocessor
#[derive(Default)]
pub struct PDFRenderer;

impl PDFRenderer {
    /// Create a new PDF Renderer
    pub fn new() -> Self {
        PDFRenderer
    }
}

impl Renderer for PDFRenderer {

    // Return the "pdf" named renderer
    fn name(&self) -> &str {
        "pdf"
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        //let config: config::PdfConfig = ctx.config.get_deserialized("output.pdf").ok().unwrap_or_default();
        //let src_dir = ctx.root.join(&ctx.config.book.src);
        let destination = &ctx.destination;
        let book = &ctx.book;

        trace!("render");

        debug!("Register the index handlebars template");

        fs::create_dir_all(&destination)
            .chain_err(|| "Unexpected error when constructing destination path")?;

        for item in book.iter() {
            debug!("{:?}",item);
        }

        utils::fs::write_file(&destination, "sample.pdf.txt", "hello pdf".as_bytes())?;
        Ok(())
    }
}