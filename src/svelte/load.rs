pub fn include_front() -> Result<String, String> {

    let html_content = include_str!("../../front/public/index.html");
    let global_css = format!(r#"<style type="text/css">{}</style>"#,  include_str!("../../front/public/global.css"));
    let css_bundle = format!(r#"<style type="text/css">{}</style>"#,  include_str!("../../front/public/build/bundle.css"));
    let js_bundle = format!(r#"<script type="text/javascript">{}</script>"#,  include_str!("../../front/public/build/bundle.js"));

    // Make replace
    return Ok(
        html_content
            .replace("<link rel='stylesheet' href='/global.css'>", &global_css)
            .replace("<link rel='stylesheet' href='/build/bundle.css'>", &css_bundle)
            .replace("<script defer src='/build/bundle.js'></script>", &js_bundle)
    );

}