use skia_safe::{paint, surfaces, EncodedImageFormat};
use skia_safe::{
    pdf,
    textlayout::{
        FontCollection, ParagraphBuilder, ParagraphStyle, PlaceholderAlignment, PlaceholderStyle,
        TextAlign, TextBaseline, TextStyle, TypefaceFontProvider,
    },
    Canvas, Color, Font, FontMgr, FontStyle, Paint, Point, Rect, TextBlob,
};
use std::fs;
mod inch;
use inch::*;

const IN_8_5: In = In(8.5);
const IN_11: In = In(11.0);
const IN_1: In = In(1.0);
const IN_HALF: In = In(0.5);
static DOMINE: &[u8] = include_bytes!("../fonts/Domine.ttf");

fn main() {
    draw_pdf("rectangle", rotated_rectangle);
    draw_pdf("hello", hello);
    draw_pdf("paragraph1", paragraph1);
    draw_pdf("paragraph2", paragraph2);
    draw_pdf("paragraph2_1", paragraph2_1);
    draw_pdf("paragraph3", paragraph3);
    draw_pdf("paragraph4", paragraph4);
    draw_svg("svg1", hello_svg);
    draw_svg("svg2", hello_svg2);
    hello_svg3();
    hello_svg4();
    draw_webp("rectangle", rotated_rectangle);
    draw_webp("hello", hello);
}

fn rotated_rectangle(canvas: &Canvas) {
    canvas.save();
    canvas.translate((128.0, 128.0)).rotate(45.0, None);
    let rect = Rect::from_point_and_size((-90.5, -90.5), (181.0, 181.0));
    let mut paint = Paint::default();
    paint.set_color(Color::BLUE);
    canvas.draw_rect(rect, &paint);
    canvas.restore();
}

fn hello(canvas: &Canvas) {
    let font_mgr = FontMgr::new();
    let face = font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap();
    let paint2 = Paint::default();
    let text = TextBlob::from_str("Hello, Skia!", &Font::from_typeface(face, 18.0)).unwrap();
    canvas.draw_text_blob(&text, (50, 25), &paint2);
}

fn paragraph1(canvas: &Canvas) {
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::new(), None);
    let paragraph_style = ParagraphStyle::new();
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    let mut ts = TextStyle::new();
    ts.set_foreground_paint(&Paint::default());
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text(LOREM_IPSUM);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(256.0);
    paragraph.paint(canvas, Point::default());
}

fn paragraph2(canvas: &Canvas) {
    const TYPEFACE_ALIAS: &str = "Domine-Regular";
    let typeface_font_provider = {
        let mut typeface_font_provider = TypefaceFontProvider::new();
        // We need a system font manager to be able to load typefaces.
        let font_mgr = FontMgr::new();
        let typeface = font_mgr
            .new_from_data(DOMINE, None)
            .expect("Failed to load Domine font");
        typeface_font_provider.register_typeface(typeface, TYPEFACE_ALIAS);
        typeface_font_provider
    };

    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(Some(typeface_font_provider.into()), None);
    let paragraph_style = ParagraphStyle::new();
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    let mut ts = TextStyle::new();
    ts.set_foreground_paint(&Paint::default())
        .set_font_families(&[TYPEFACE_ALIAS]);
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text(LOREM_IPSUM);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout((IN_8_5 - In(2.0)).pt());
    paragraph.paint(canvas, Point::default());
}

fn paragraph2_1(canvas: &Canvas) {
    const TYPEFACE_ALIAS: &str = "Domine-Regular";
    let typeface_font_provider = {
        let mut typeface_font_provider = TypefaceFontProvider::new();
        // We need a system font manager to be able to load typefaces.
        let font_mgr = FontMgr::new();
        let typeface = font_mgr
            .new_from_data(DOMINE, None)
            .expect("Failed to load Domine font");
        typeface_font_provider.register_typeface(typeface, TYPEFACE_ALIAS);
        typeface_font_provider
    };

    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(Some(typeface_font_provider.into()), None);
    let paragraph_style = ParagraphStyle::new();
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    let mut ts = TextStyle::new();
    ts.set_foreground_paint(&Paint::default())
        .set_font_families(&[TYPEFACE_ALIAS]);
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text(LOREM_IPSUM);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout((IN_8_5 - In(2.0)).pt());
    paragraph.paint(canvas, Point::default());
}

fn paragraph3(canvas: &Canvas) {
    // Define font.
    const TYPEFACE_ALIAS: &str = "domine-regular";
    let typeface_font_provider = {
        let mut typeface_font_provider = TypefaceFontProvider::new();
        // We need a system font manager to be able to load typefaces.
        let font_mgr = FontMgr::new();
        let typeface = font_mgr
            .new_from_data(DOMINE, None)
            .expect("Failed to load font");
        typeface_font_provider.register_typeface(typeface, TYPEFACE_ALIAS);
        typeface_font_provider
    };
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(Some(typeface_font_provider.into()), None);

    // Define paragraph style.
    let mut paragraph_style = ParagraphStyle::new();
    paragraph_style.set_text_align(TextAlign::Justify);
    // paragraph_style.set_max_lines(Some(1));

    // Define text style.
    let mut ts = TextStyle::new();
    // ts.set_font_style(FontStyle::italic());
    // ts.set_font_style(FontStyle::bold());
    // ts.set_font_style(FontStyle::bold_italic());
    ts.set_font_size(12.0);
    ts.set_foreground_paint(&Paint::default())
        .set_font_families(&[TYPEFACE_ALIAS]);

    // Define the placeholder style.
    let ps = PlaceholderStyle {
        width: IN_HALF.pt(),
        height: 0.0,
        alignment: PlaceholderAlignment::Baseline,
        baseline_offset: 0.0,
        baseline: TextBaseline::Alphabetic,
    };

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_placeholder(&ps);
    paragraph_builder.add_text(TXT);

    let mut paragraph = paragraph_builder.build();

    paragraph.layout((IN_8_5 - In(2.0)).pt());
    paragraph.paint(
        canvas,
        Point {
            x: IN_1.pt(),
            y: 0.0,
        },
    );
}

fn paragraph4(canvas: &Canvas) {
    // Define font.
    const TYPEFACE_ALIAS: &str = "domine-regular";
    let typeface_font_provider = {
        let mut typeface_font_provider = TypefaceFontProvider::new();
        // We need a system font manager to be able to load typefaces.
        let font_mgr = FontMgr::new();
        let typeface = font_mgr
            .new_from_data(DOMINE, None)
            .expect("Failed to load font");
        typeface_font_provider.register_typeface(typeface, TYPEFACE_ALIAS);
        typeface_font_provider
    };
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(Some(typeface_font_provider.into()), None);

    // Define paragraph style.
    let mut paragraph_style = ParagraphStyle::new();
    paragraph_style.set_text_align(TextAlign::Justify);
    // paragraph_style.set_max_lines(Some(1));

    // Define text style.
    let font_size: f32 = 12.0;
    let line_height_multiplier: f32 = 1.65;
    let mut ts = TextStyle::new();
    // ts.set_font_style(FontStyle::italic());
    // ts.set_font_style(FontStyle::bold());
    // ts.set_font_style(FontStyle::bold_italic());
    ts.set_font_size(font_size);
    ts.set_height(line_height_multiplier);
    ts.set_height_override(true);
    ts.set_foreground_paint(&Paint::default())
        .set_font_families(&[TYPEFACE_ALIAS]);

    // Define the placeholder style.
    let ps = PlaceholderStyle {
        width: IN_HALF.pt(),
        height: 0.0,
        alignment: PlaceholderAlignment::Baseline,
        baseline_offset: 0.0,
        baseline: TextBaseline::Alphabetic,
    };

    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, &font_collection);
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_placeholder(&ps);
    paragraph_builder.add_text(TXT);

    let mut paragraph = paragraph_builder.build();
    paragraph.layout((IN_8_5 - In(2.0)).pt());
    paragraph.paint(
        canvas,
        Point {
            x: IN_1.pt(),
            y: 0.0,
        },
    );
    // eprintln!("paragraph.height:{}", paragraph.height());
    // for (i, line_metric) in paragraph.get_line_metrics().iter().enumerate() {
    //     eprintln!("Line {}: height = {}", i + 1, line_metric.height);
    // }

    let mut y = paragraph.height();
    y += paragraph.get_line_metrics_at(0).unwrap().height as f32;

    let mut paragraph_builder2 = ParagraphBuilder::new(&paragraph_style, &font_collection);
    paragraph_builder2.push_style(&ts);
    paragraph_builder2.add_placeholder(&ps);
    paragraph_builder2.add_text(TXT2);

    let mut paragraph2 = paragraph_builder2.build();
    paragraph2.layout((IN_8_5 - In(2.0)).pt());
    paragraph2.paint(canvas, Point { x: IN_1.pt(), y });
}

fn hello_svg(canvas: &Canvas) {
    let font_mgr = FontMgr::new();
    let face = font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap();
    let paint1 = Paint::default();
    let text = TextBlob::from_str("Hello, Skia!", &Font::from_typeface(face, 18.0)).unwrap();
    canvas.draw_text_blob(&text, (50, 25), &paint1);
}

fn hello_svg2(canvas: &Canvas) {
    let font_mgr = FontMgr::new();
    let face = font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap();
    let mut paint1 = Paint::default();
    paint1
        .set_anti_alias(true)
        .set_color(Color::from_rgb(255, 255, 255))
        .set_style(paint::Style::Fill);
    let text = TextBlob::from_str("Hello, Skia!", &Font::from_typeface(face, 18.0)).unwrap();
    canvas.draw_text_blob(&text, (50, 25), &paint1);
}

fn hello_svg3() {
    let font_mgr = FontMgr::new();
    let face = font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap();
    let mut paint1 = Paint::default();
    paint1
        .set_anti_alias(true)
        .set_color(Color::from_rgb(255, 255, 255))
        .set_style(paint::Style::Fill);
    let text = TextBlob::from_str("Hello, Skia!", &Font::from_typeface(face, 18.0)).unwrap();

    let size = text.bounds().size();
    // eprintln!("{:?}", text.bounds());
    let canvas = skia_safe::svg::Canvas::new(Rect::from_size(size), None);
    canvas.draw_text_blob(&text, (0f32, size.height), &paint1);
    let data = canvas.end();
    fs::write("svg3.svg", data.as_bytes()).unwrap();
}

fn hello_svg4() {
    let font_mgr = FontMgr::new();
    let face = font_mgr
        .legacy_make_typeface(None, FontStyle::default())
        .unwrap();
    let font = &Font::from_typeface(face, 18.0);
    let mut paint1 = Paint::default();
    paint1
        .set_anti_alias(true)
        .set_color(Color::from_rgb(255, 255, 255))
        .set_style(paint::Style::Fill);

    let s = "Hello, Skia!";
    let (_, rect) = font.measure_str(s, Some(&paint1));
    let size = rect.size();
    // eprintln!("{:?}, {:?}", rect, rect.size());
    let canvas = skia_safe::svg::Canvas::new(Rect::from_size(size), None);
    canvas.draw_str(s, (0f32, size.height - rect.bottom), font, &paint1);
    let data = canvas.end();
    fs::write("svg4.svg", data.as_bytes()).unwrap();
}

fn draw_pdf(name: &str, func: impl Fn(&Canvas)) {
    let size = (IN_8_5.pt(), IN_11.pt());
    let mut memory = Vec::new();
    let mut document = pdf::new_document(&mut memory, None).begin_page(size, None);

    func(document.canvas());
    document.end_page().close();

    fs::write(format!("{}.pdf", name), &memory).unwrap();
}

fn draw_svg(name: &str, func: impl Fn(&Canvas)) {
    let size = (IN_8_5.pt(), IN_11.pt());
    let canvas = skia_safe::svg::Canvas::new(Rect::from_size(size), None);
    func(&canvas);
    let data = canvas.end();
    fs::write(format!("{}.svg", name), data.as_bytes()).unwrap();
}

fn draw_webp(name: &str, func: impl Fn(&Canvas)) {
    let size = (IN_8_5.pt() as i32, IN_11.pt() as i32);
    let mut surface = surfaces::raster_n32_premul(size).unwrap();
    let canvas = surface.canvas();
    func(canvas);
    let image = surface.image_snapshot();
    let data = image
        .encode(
            &mut surface.direct_context(),
            EncodedImageFormat::WEBP,
            None,
        )
        .unwrap();
    fs::write(format!("{}.webp", name), data.as_bytes()).unwrap();
}

static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur at leo at nulla tincidunt placerat. Proin eget purus augue. Quisque et est ullamcorper, pellentesque felis nec, pulvinar massa. Aliquam imperdiet, nulla ut dictum euismod, purus dui pulvinar risus, eu suscipit elit neque ac est. Nullam eleifend justo quis placerat ultricies. Vestibulum ut elementum velit. Praesent et dolor sit amet purus bibendum mattis. Aliquam erat volutpat.";

static TXT: &str = "Despite military, scientific, and political statements to the contrary, we believe that UFOs or UAPs do not represent a threat of any kind to national or global security or humanity at large.";

static TXT2: &str ="There is overwhelming and well-researched evidence and thousands of eyewitness accounts that these objects have been present throughout Earth’s history without detrimental effect on the day-to-day affairs of humanity.";
