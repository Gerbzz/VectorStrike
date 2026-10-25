//! SVGZ in the app: Place takes it, and a .svgz name (Export As SVG, Save As) writes it.

use std::cell::RefCell;
use std::rc::Rc;

use serde_json::{Value, json};
use vectorcraft_engine::Session;

use crate::{Services, VectorcraftApp, io};

pub(super) type Written = Rc<RefCell<Vec<(String, Vec<u8>)>>>;

/// An app with a rectangle whose save dialog answers `picked`; the writer records its files.
pub(super) fn app(picked: &str) -> (VectorcraftApp, Written) {
    let written = Written::default();
    let w = written.clone();
    let picked = picked.to_string();
    let services = Services {
        write: Some(Box::new(move |p: &str, b: &[u8]| {
            w.borrow_mut().push((p.to_string(), b.to_vec()));
            Ok(())
        })),
        pick_save: Some(Box::new(move |_: &crate::FilePick| Some(picked.clone()))),
        ..Default::default()
    };
    let mut app = VectorcraftApp::new(Session::new(), services);
    app.run("file.new", json!({"width": 100, "height": 80})).unwrap();
    app.run("shape.rectangle", json!({"x": 10, "y": 10, "width": 40, "height": 30})).unwrap();
    (app, written)
}

/// Files owned by one disk-backed desktop-save test.
pub(super) struct DiskFiles {
    root: std::path::PathBuf,
}

impl DiskFiles {
    pub(super) fn new(tag: &str) -> Self {
        let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("vc-ui-save-{tag}-{}-{stamp}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        Self { root }
    }

    pub(super) fn path(&self, name: &str) -> String {
        self.root.join(name).to_string_lossy().into_owned()
    }

    pub(super) fn attach(&self, app: &mut VectorcraftApp, written: &Written) {
        let (read_root, write_root, w) = (self.root.clone(), self.root.clone(), written.clone());
        app.services.read = Some(Box::new(move |path: &str| {
            if std::path::Path::new(path).parent() != Some(read_root.as_path()) {
                return Err("test read is outside its owned fixture".into());
            }
            std::fs::read(path).map_err(|e| e.to_string())
        }));
        app.services.write = Some(Box::new(move |path: &str, bytes: &[u8]| {
            if std::path::Path::new(path).parent() != Some(write_root.as_path()) {
                return Err("test write is outside its owned fixture".into());
            }
            std::fs::write(path, bytes).map_err(|e| e.to_string())?;
            w.borrow_mut().push((path.to_string(), bytes.to_vec()));
            Ok(())
        }));
    }
}

impl Drop for DiskFiles {
    fn drop(&mut self) {
        // create_dir reserved this directory exclusively for this fixture.
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// Opt in to the native host's physical read/write contract for repeated saves.
pub(super) fn disk_app(name: &str) -> (VectorcraftApp, Written, DiskFiles) {
    let files = DiskFiles::new("native-options");
    let (mut app, written) = app(&files.path(name));
    files.attach(&mut app, &written);
    (app, written, files)
}

#[test]
fn svgz_places_like_svg() {
    let (mut app, _) = app("/tmp/unused.svg");
    let svg = app.session.execute("document.serialize", &json!({"format": "svg"})).unwrap()["text"].as_str().unwrap().to_string();
    app.run("file.new", json!({"width": 100, "height": 80})).unwrap();
    io::place_bytes(&mut app, "art.svgz", &vectorcraft_svg::compress(&svg)).unwrap();
    assert_eq!(app.session.doc().unwrap().selection.objects.len(), 1, "the rectangle was placed");
}

#[test]
fn a_svgz_name_writes_compressed_svg() {
    let (mut app, written) = app("/tmp/art.svgz");
    // Export As SVG, named .svgz in the save dialog.
    app.run("file.export.svg", Value::Null).unwrap();
    crate::dialogs::confirm(&mut app).unwrap();
    let (path, bytes) = written.borrow()[0].clone();
    assert_eq!(path, "/tmp/art.svgz");
    assert!(vectorcraft_svg::text_of(&bytes).unwrap().contains("<style>"), "the SVG Options apply");
    assert!(vectorcraft_svg::is_svgz(&bytes));
    // Save As .svgz asks for SVG Options, then saves compressed and keeps the path.
    let r = app.run("file.saveAs", Value::Null).unwrap();
    assert_eq!(r["path"], "/tmp/art.svgz");
    crate::dialogs::confirm(&mut app).unwrap();
    assert!(vectorcraft_svg::is_svgz(&written.borrow()[1].1));
    assert_eq!(app.session.active().unwrap().path.as_deref(), Some("/tmp/art.svgz"));
}
