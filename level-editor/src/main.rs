use bottomless_pit::material::MaterialBuilder;
use bottomless_pit::engine_handle::EngineBuilder;
use level_editor::editor::MainEditor;


fn main() {
    let mut engine = EngineBuilder::new()
        .build()
        .unwrap();

    let editor = MainEditor::new(&mut engine);

    engine.run(editor);
}