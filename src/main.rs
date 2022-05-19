use chrono::offset::Local;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

type ClipError = Box<dyn std::error::Error>;
type ClipResult = Result<(), ClipError>;

fn set_clip_contents(str: String) -> ClipResult {
    let mut clipboard: ClipboardContext = ClipboardProvider::new()?;
    clipboard.set_contents(str)?;
    println!("Set new context: {:?}", clipboard.get_contents()?);
    Ok(())
}

fn main() {
    let time = Local::now().format("%Y-%m-%d %H-%M-%S").to_string();
    
    if let Err(error) = set_clip_contents(time) {
        eprintln!("Error: {}", error.as_ref().to_string());
        std::process::exit(1);
    }
}
