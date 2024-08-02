use pdfium_render::prelude::*;

fn export_pdf_to_jpegs(path: &impl AsRef<Path>, password: Option<&str>) -> Result<(), PdfiumError> {
    // Renders each page in the PDF file at the given path to a separate JPEG file.

    // Bind to a Pdfium library in the same directory as our Rust executable;
    // failing that, fall back to using a Pdfium library provided by the operating system.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    // Load the document from the given path...

    let document = pdfium.load_pdf_from_file(path, password)?;

    // ... set rendering options that will be applied to all pages...

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    // ... then render each page to a bitmap image, saving each image to a JPEG file.

    for (index, page) in document.pages().iter().enumerate() {
        page.render_with_config(&render_config)?
            .as_image() // Renders this page to an image::DynamicImage...
            .into_rgb8() // ... then converts it to an image::Image...
            .save_with_format(
                format!("test-page-{}.jpg", index), 
                image::ImageFormat::Jpeg
            ) // ... and saves it to a file.
            .map_err(|_| PdfiumError::ImageError)?;
    }

    Ok(())
}