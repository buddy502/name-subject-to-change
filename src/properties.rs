///*
//    // Every CSS html property I'm implementing
//    Layout & sizing:
//        display, width, height, min-width, max-width, min-height, max-height, box-sizing
//        
//    Spacing:
//        margin, margin-left, margin-right, margin-top, margin-bottom, padding,
//        padding-left, padding-right, padding-top, padding-bottom
//        
//    Borders:
//        border, border-width, border-style, border-color, border-top, border-right, etc.
//        
//    Backgrounds:
//        background-color, background-image, background-repeat, background-position, background-size
//        
//    Text & typography:
//        color, font-size, font-family, font-weight, line-height, text-align, text-decoration
//        
//    Positioning:
//        position, top, right, bottom, left, z-index, transform, translate, rotate
//        
//    Box model:
//        box-shadow, outline, outline-offset
//        
//    Flexbox/Grid:
//        display (flex | grid), flex-direction, justify-content, align-items,
//        grid-template-columns, grid-template-rows
//        
//    Pseudo-elements:
//        ::before, ::after (for content inside <div>)
//*/
//
///*
//    Every CSS property is going to be in the same file as the html declarations
//        It looks like a lot of .'s but it's needed for readability trust, if not then a
//    struct holds like 100 different CSS properties, removed uneeded .style struct
//    e.g. let div = create_div(); div.spacing.margin = 
//    Used for inline CSS, no external file css needed
//
//    Div creates a default implementation that the user can change
//*/

use std::panic::Location;

enum Size {
    // default
    Auto,
    None,

    // Absolute Lengths
    Px(f32),
    Cm(f32),
    Mm(f32),

    // Relative Lengths
    Em(f32),
    Rem(f32),
    Vw(f32),
    Vh(f32),
    Vmin(f32),
    Vmax(f32),
    Fr(f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Repeat {
    Repeat,
    NoRepeat,
    RepeatX,
    RepeatY,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TextDecoration {
    None,
    Underline,
    Overline,
    LineThrough,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum Display {
    #[default]
    None,
    Block,
    Inline,
    InlineBlock,
    Flex,
    Grid,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Justify {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Align {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

struct Color {
    r: u16,
    g: u16,
    b: u16,
}

struct Shadow {
    size: Vec<Size>,
    color: Color,
    blur: u16,
    spread: i16,
}

impl Shadow {
    // returning a generic like but for how many parameters
    // rather than type
    #[track_caller]
    // UNIMPLEMENTED: Base function, requires winapi functions
    fn box_shadow(shadow: &mut Shadow) -> Result<Option<&mut Shadow>, String> {
        // options 1 vec, 2 vec, 4 vec, requiring color

        let loc = Location::caller();
        let shadow_len = shadow.size.len();
        if !(2..=4).contains(&shadow_len) {
            return Err(format!("ERR at {}: shadow count requires 4, 2, or 1 inputs", loc.line()));
        }

        for size in shadow.size.iter() {
            match size {
                Size::None => {return Ok(None)}
                _ => {return Ok(None)} // UNIMPLEMENTED rest of Size values
        }
    }

        Ok(Some(shadow))
    }
}

pub fn px(size: f32) -> Size {Size::Px(size)}
pub fn cm(size: f32) -> Size {Size::Cm(size)}
pub fn mm(size: f32) -> Size {Size::Mm(size)}

pub fn em(size: f32) -> Size {Size::Px(size)}
pub fn rem(size: f32) -> Size {Size::Px(size)}
pub fn vw(size: f32) -> Size {Size::Vw(size)}
pub fn vh(size: f32) -> Size {Size::Vh(size)}
pub fn vmin(size: f32) -> Size {Size::Vmin(size)}
pub fn vmax(size: f32) -> Size {Size::Vmax(size)}
pub fn fr(size: f32) -> Size {Size::Fr(size)}

// UNIMPLEMENTED
pub fn auto() -> Size {Size::Auto}
impl Default for Size {
    fn default() -> Self { Size::Px(0.0) }
}

#[derive(Default)]
struct Sizing {
    display: Display,
    width: Size,
    height: Size,
    min_width: Size,
    max_width: Size,
    min_height: Size,
    max_height: Size,
    box_sizing: Size,
}

#[derive(Default)]
struct Spacing {
    margin: Size,
    margin_left: Size,
    margin_right: Size,
    margin_top: Size,
    margin_bottom: Size,
    padding: Size,
    padding_left: Size,
    padding_right: Size,
    padding_top: Size,
    padding_bottom: Size
}

#[derive(Default)]
struct Border {
    border_width: Size,
    border_style: Option<BorderStyle>,
    border_color: Option<Color>,
    border_top: Size,
    border_right: Size,
    border_bottom: Size,
    border_left: Size,
    // box model
    box_shadow: Option<Shadow>,
    outline: Size,
    outline_offset: Size,
}

#[derive(Default)]
struct Background {
    background_color: Option<Color>,
    background_image: Option<String>,        // path or URL
    background_repeat: Option<Repeat>,
    background_position: Option<(Size, Size)>,
    background_size: Option<(Size, Size)>,
}

#[derive(Default)]
struct Typography {
    color: Option<Color>,
    font_size: Size,
    font_family: Option<String>,
    font_weight: Option<u16>,                // 100 to 900
    line_height: Size,
    text_align: Option<TextAlign>,
    text_decoration: Option<TextDecoration>,
}

#[derive(Default)]
struct Positioning {
    position: Option<Position>,
    top: Size,
    right: Size,
    bottom: Size,
    left: Size,
    z_index: i32,
    transform: Option<String>,
    translate: Option<(Size, Size)>,
    rotate: f32,                             // degrees
}

#[derive(Default)]
struct Layout {
    display: Option<Display>,                // Flex | Grid | ...
    flex_direction: Option<FlexDirection>,
    justify_content: Option<Justify>,
    align_items: Option<Align>,
    grid_template_columns: Vec<Size>,
    grid_template_rows: Vec<Size>,
}

#[derive(Default)]
struct Pseudo {
    before: Option<String>,                  // ::before content
    after: Option<String>,                   // ::after content
}

pub struct Styles {
    pub spacing: Option<Spacing>,
    pub sizing: Option<Sizing>,
    pub border: Option<Border>,
    pub background: Option<Background>,
    pub typography: Option<Typography>,
    pub positioning: Option<Positioning>,
    pub layout: Option<Layout>,
    pub pseudo: Option<Pseudo>
}
