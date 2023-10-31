use mathjax_svg::Converter;

fn main() {
    let mut converter = Converter::new();
    println!(
        "{}",
        converter.convert_to_svg(r#"\int_{-\infty}^\infty e^{-x^2}\,\mathrm dx"#)
    );
}
