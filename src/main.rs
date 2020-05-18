use web_view::*;
mod svelte;

fn main() {

    let html_content = match svelte::load::include_front() {
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