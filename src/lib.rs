use std::os::raw::c_char;
use std::error::Error;
use std::io::{stdout};
use std::ffi::CStr;

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


#[no_mangle]
pub extern "C" fn print_html_ffi(input: *const c_char) -> () {
    let data: &CStr = unsafe { CStr::from_ptr(input) };
    let html = data.to_str().unwrap();
    print_html(html).unwrap();
}
