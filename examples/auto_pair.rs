use lineeditor::DefaultAutoPair;
use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::StringPrompt;

fn main() {
    let prompt = StringPrompt::new("gql> ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));
    line_editor.set_autopair(Some(Box::<DefaultAutoPair>::default()));

    match line_editor.read_line() {
        Ok(LineEditorResult::Success(line)) => {
            println!("Line {}", line);
        }
        _ => {}
    }
}