use std::path::PathBuf;
use t_rex::editor::Editor;

fn main() {
    let editor = Editor::default();
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).map(|path| PathBuf::from(path));
    editor.run(path).unwrap();
}
