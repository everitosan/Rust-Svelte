extern crate regex;

use regex::Regex;
extern crate image_base64;


fn replace_assets(text: &str) -> String {
    let mut res = String::from(text);
    let png_regex = match Regex::new(r"[\w/\.\s]+\.png") {
        Ok(reg) => reg,
        Err(e) => panic!("{}", e)
    };

    let jpeg_regex = match Regex::new(r"[\w/\.\s]+\.jp?eg") {
        Ok(reg) => reg,
        Err(e) => panic!("{}", e)
    };

    for png in png_regex.captures_iter(text) {
        let image = format!("front/public/{}", &png[0]);
        let remplacemet = image_base64::to_base64(&image);
        res = res.replace(&png[0], &remplacemet);
    }

    for jpeg in jpeg_regex.captures_iter(text) {
        let image = format!("front/public/{}", &jpeg[0]);
        let remplacemet = image_base64::to_base64(&image);
        res = res.replace(&jpeg[0], &remplacemet);
    }

    return res
}

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
            .replace("<script defer src='/build/bundle.js'></script>", &replace_assets(&js_bundle))
    );

}