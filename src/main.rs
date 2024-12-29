use lightningcss::{
    rules::{self, CssRule}, stylesheet::{ParserOptions, PrinterOptions, StyleSheet}, traits::IntoOwned, values::{length::LengthValue, url::Url}, visit_types, visitor::{Visit, VisitTypes, Visitor}
};
use std::convert::Infallible;

fn main() {
    let mut stylesheet = StyleSheet::parse(
        r#"
@import './base.css';
@import url(./index.css);
@import url("./extension.css");
@import '/public.css';
@import url(https://remote.css);


body {
  background: url('./background.png');
}

.home {
  background: url('./img/home.png') no-repeat;
}

div {
  background: url('/@/img/logo.png');
}

p {
  background: url('@/img/logo.png');
  /* top: -8px/2 + 1; */
}

.home {filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20)}
  "#,
        ParserOptions::default(),
    )
    .unwrap();

    struct MyVisitor;
    impl<'i> Visitor<'i> for MyVisitor {
        type Error = Infallible;

        fn visit_types(&self) -> VisitTypes {
            visit_types!(URLS | LENGTHS)
        }

        fn visit_url(&mut self, url: &mut Url<'i>) -> Result<(), Self::Error> {
            println!("Visiting url: {}", url.url);
            url.url = format!("https://mywebsite.com/{}", url.url).into();
            Ok(())
        }

        fn visit_length(&mut self, length: &mut LengthValue) -> Result<(), Self::Error> {
            match length {
                LengthValue::Px(px) => *length = LengthValue::Rem(*px / 16.0),
                _ => {}
            }

            Ok(())
        }
    }

    stylesheet.visit(&mut MyVisitor).unwrap();

    let res = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            ..Default::default()
        })
        .unwrap();

    let rules = stylesheet.rules;

    for rule in rules.0.iter() {
        if let CssRule::Import(import) = rule {
            println!("Found import: {}", import.url);
        }
    }

}
