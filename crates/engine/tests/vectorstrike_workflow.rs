use serde_json::{Value, json};
use vectorcraft_engine::Session;

#[test]
fn draw_undo_save_reopen_export_and_reject_bad_file() {
    let mut editor = Session::new();
    editor.execute("file.new", &json!({"width": 160, "height": 120, "artboards": 1})).unwrap();
    editor.execute("shape.rectangle", &json!({"x": 12, "y": 18, "width": 70, "height": 35})).unwrap();
    assert_eq!(editor.doc().unwrap().doc.node_count(), 2);
    editor.execute("edit.undo", &json!({})).unwrap();
    assert_eq!(editor.doc().unwrap().doc.node_count(), 1);
    editor.execute("edit.redo", &json!({})).unwrap();

    let dir = vectorcraft_testkit::temp_dir("vectorstrike-workflow");
    let native = dir.join("poster.vstrike");
    editor.execute("document.save", &json!({"path": native.to_string_lossy()})).unwrap();
    let bytes = std::fs::read(&native).unwrap();
    assert_eq!(serde_json::from_slice::<Value>(&bytes).unwrap()["format"], "vstrike");

    let mut reopened = Session::new();
    reopened.execute("document.open", &json!({"path": native.to_string_lossy()})).unwrap();
    assert_eq!(reopened.doc().unwrap().doc.node_count(), 2);
    let svg = dir.join("poster.svg");
    reopened.execute("document.export", &json!({"path": svg.to_string_lossy()})).unwrap();
    assert!(std::fs::read_to_string(svg).unwrap().contains("<svg"));

    let invalid = vectorcraft_format::base64_encode(b"not a document");
    assert!(reopened.execute("document.open", &json!({"name": "bad.vstrike", "dataBase64": invalid})).is_err());
    assert_eq!(reopened.doc().unwrap().doc.node_count(), 2);
}
