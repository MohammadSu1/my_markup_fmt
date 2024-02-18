use insta::{assert_snapshot, glob, Settings};
use std::{fs, path::Path, process::Command};

#[test]
fn integration_with_dprint_ts_snapshot() {
    glob!("integration/**/*.{html,vue,svelte,astro,jinja}", |path| {
        let file = fs::File::open(path).unwrap();

        let output = Command::new("../node_modules/.bin/dprint")
            .arg("fmt")
            .arg("--stdin")
            .arg(path)
            .arg("--plugins")
            .arg("../target/wasm32-unknown-unknown/debug/dprint_plugin_markup.wasm")
            .arg("https://plugins.dprint.dev/g-plane/malva-v0.1.4.wasm")
            .arg("https://plugins.dprint.dev/typescript-0.88.9.wasm")
            .stdin(file)
            .output()
            .unwrap()
            .stdout;

        build_settings(path.parent().unwrap().join("dprint_ts")).bind(|| {
            let name = path.file_name().unwrap().to_str().unwrap();
            assert_snapshot!(name, String::from_utf8(output).unwrap());
        });
    });
}

#[test]
fn integration_with_biome_snapshot() {
    glob!("integration/**/*.{html,vue,svelte,astro,jinja}", |path| {
        let file = fs::File::open(path).unwrap();

        let output = Command::new("../node_modules/.bin/dprint")
            .arg("fmt")
            .arg("--stdin")
            .arg(path)
            .arg("--plugins")
            .arg("../target/wasm32-unknown-unknown/debug/dprint_plugin_markup.wasm")
            .arg("https://plugins.dprint.dev/g-plane/malva-v0.1.4.wasm")
            .arg("https://plugins.dprint.dev/biome-0.3.2.wasm")
            .stdin(file)
            .output()
            .unwrap()
            .stdout;

        build_settings(path.parent().unwrap().join("biome")).bind(|| {
            let name = path.file_name().unwrap().to_str().unwrap();
            assert_snapshot!(name, String::from_utf8(output).unwrap());
        });
    });
}

fn build_settings(path: impl AsRef<Path>) -> Settings {
    let mut settings = Settings::clone_current();
    settings.set_snapshot_path(path);
    settings.remove_snapshot_suffix();
    settings.set_prepend_module_to_snapshot(false);
    settings.remove_input_file();
    settings.set_omit_expression(true);
    settings.remove_input_file();
    settings.remove_info();
    settings
}
