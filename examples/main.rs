use mathjax_svg::convert_to_svg;

fn main() {
    println!(
        "{}",
        convert_to_svg(r#"\int_{-\infty}^\infty e^{-x^2}\,\mathrm dx"#)
    );
}
