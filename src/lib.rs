use std::error::Error;
use std::io::{stdout};

use pulldown_cmark::{Options, Parser};
use syntect::parsing::SyntaxSet;
use html2md::parse_html;
use mdcat::{ResourceAccess, TerminalCapabilities, TerminalSize};

pub fn print_md(md_text: &str) -> Result<(), Box<dyn Error>> {
    let terminal_capabilities = TerminalCapabilities::detect();
    let size = TerminalSize::detect().unwrap_or_default();
    let columns = size.width;

    let work_dir = std::env::current_dir()?;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&md_text, options);


    let syntax_set = SyntaxSet::load_defaults_newlines();
    mdcat::push_tty(
        &mut stdout(),
        &terminal_capabilities,
        TerminalSize {
            width: columns,
            ..size
        },
        parser,
        &work_dir,
        ResourceAccess::RemoteAllowed,
        syntax_set,
    )?;
    Ok(())
}

pub fn print_html(html: &str) -> Result<(), Box<dyn Error>> {
    let md = parse_html(html);
    print_md(&md)
}


/// returns a pair of error code and error text
#[no_mangle]
pub fn print_html_ffi(html: &str) -> (u32, String) {
    match print_html(html) {
        Ok(_) => (0, "".to_string()),
        Err(e) => (1, e.to_string())
    }
}
