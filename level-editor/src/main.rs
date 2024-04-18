use bottomless_pit::material::MaterialBuilder;
use bottomless_pit::engine_handle::EngineBuilder;
use level_editor::editor::Editor;


fn main() {
    let mut engine = EngineBuilder::new()
        .build()
        .unwrap();

    let material = MaterialBuilder::new()
        .build(&mut engine);

    let editor = Editor::new(material, &mut engine);

    engine.run(editor);
}