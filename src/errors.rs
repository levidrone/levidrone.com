use error_chain::*;

error_chain! {
    foreign_links {
        FsExtraError(fs_extra::error::Error);
        IoError(std::io::Error);
        HandlebarsRenderError(handlebars::RenderError);
        HandlebarsTemplateError(handlebars::TemplateFileError);
    }

    errors {
        InvalidTemplate {
            description("invalid template")
        }
    }
}
