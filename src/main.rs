mod editor;
use editor::Editor;

use crate::editor::view::buffer::Buffer;
use editor::view::View;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let buffer = if args.len() > 1 {
        Buffer::read_from_file(&args[1]).unwrap_or_default()
    } else {
        Buffer::default()
    };

    let mut editor = Editor {
        view: View {
            buffer,
            ..Default::default()
        },
        ..Default::default()
    };

    editor.run().unwrap();
}
