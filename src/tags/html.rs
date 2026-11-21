/*
 * Default Html properties and tag
 * display	inline
 * position	static
 * color	depends on the UA (usually black)
 * background-color	transparent
 * width / height	auto
 * margin / padding	0
 * border-width / border-style	medium / none
 * font-size	medium (16px)
 * font-weight	normal (400)
 * font-family	UA-dependent (usually serif)
 * line-height	normal
 * text-align	start
 * float	none
 * overflow	visible
 * visibility	visible
 * opacity	1
*/

use crate::properties::*;

#[derive(Debug)]
pub struct HtmlBase {
    pub styles: Styles,
}

impl HtmlBase {
    pub fn init_html() -> Self {
        HtmlBase {
            styles: Styles {
                spacing: Some(Spacing { margin: px(0.0), padding: px(0.0), ..Default::default() }),
                sizing: Some(Sizing { width: Size::Auto, height: Size::Auto, ..Default::default() }),
                border: Some(Border { border_width: px(3.0), border_style: Some(BorderStyle::None),
                        ..Default::default()
                }),
                background: Some(Background {
                    background_color: Color { r: 0, g: 0, b: 0, a: 0 }, // transparent
                    ..Default::default()
                }),
                typography: Some(Typography {
                    color: Color { r: 0, g: 0, b: 0, a: 255 },
                    font_size: px(16.0),
                    font_weight: 400,
                    font_family: String::from("serif"),
                    line_height: Size::None,
                    text_align: Some(TextAlign::Start),
                    ..Default::default()
                }),
                positioning: Some(Positioning {
                    position: Some(Position::Static),
                    float: Some(Float::None),
                    ..Default::default()
                }),
                layout: Some(Layout {
                    display: Some(Display::Inline),
                    overflow: Some(Overflow::Visible),
                    visible: true,
                    opacity: 1.0,
                    ..Default::default()
                }),
                pseudo: None,
            },
        }
    }
}
