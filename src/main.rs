mod editor;

fn main() {
    let editor = editor::Editor::default();
    editor.run().unwrap();
}
