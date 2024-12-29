use lightningcss::{
  stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
  targets::{Features, Targets},
};

fn main() {
  let css = ".logo { background: image-set(url(logo.png) 2x, url(logo.png) 1x);}";

  let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).unwrap();

  stylesheet.minify(MinifyOptions {
      targets: Targets {
          include: Features::VendorPrefixes,
          ..Default::default()
      },
      ..Default::default()
    }).unwrap();
  let res = stylesheet
      .to_css(PrinterOptions {
          targets: Targets {
              include: Features::VendorPrefixes,
              ..Default::default()
          },
          ..Default::default()
      })
      .unwrap();
  println!("{}", res.code);
}
