use web_view::*;
use std::fs::File;
use std::io::Read;


fn read_file(path: String) -> String {
    let mut content = String::new();
    let mut file = File::open(path).expect("No se ecuentra el archivo");

    file.read_to_string(&mut content)
        .expect("No de puede leer el archivo {}");
    return content;
}

fn build_front(path: String) -> Result<String, String> {
    // Index del front
    let mut index_path = String::from(&path);
    index_path.push_str("index.html");

    // Global css file
    let mut global_css_path = String::from(&path);
    global_css_path.push_str("global.css");

    // Bundle css
    let mut css_bundle_path = String::from(&path);
    css_bundle_path.push_str("build/bundle.css");

    // Js bundle
    let mut js_bundle_path = String::from(&path);
    js_bundle_path.push_str("/build/bundle.js");

    let html_content = read_file(index_path);
    let global_css = format!(r#"<style type="text/css">{}</style>"#,  read_file(global_css_path));
    let css_bundle = format!(r#"<style type="text/css">{}</style>"#,  read_file(css_bundle_path));
    let js_bundle = format!(r#"<script type="text/javascript">{}</script>"#,  read_file(js_bundle_path));

    // Make replace
    return Ok(
        html_content
            .replace("<link rel='stylesheet' href='/global.css'>", &global_css)
            .replace("<link rel='stylesheet' href='/build/bundle.css'>", &css_bundle)
            .replace("<script defer src='/build/bundle.js'></script>", &js_bundle)
    );

}

fn main() {

    let html_content = match build_front("front/public/".to_string()) {
        Ok(c) => c,
        _ => panic!("No se pudo construir el front.")
    };

    web_view::builder()
        .title("My Project")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}