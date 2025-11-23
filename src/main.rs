mod editor;
use editor::Editor;

use crate::editor::view::buffer::Buffer;
use editor::view::View;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("untitled.txt")
    };
    let buffer = if args.len() > 1 {
        Buffer::read_from_file(&file_path).unwrap_or_default()
    } else {
        Buffer::default()
    };

    let mut editor = Editor {
        view: View {
            buffer,
            ..Default::default()
        },
        file_path,

        ..Default::default()
    };

    editor.run().unwrap();
}
