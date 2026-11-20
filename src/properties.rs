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
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::Div;
use crate::HtmlBase;

pub enum HtmlTag {
    Html(HtmlBase),
    Div(Div),
}

pub struct Tree {
    pub value: HtmlTag,
    pub parent: RefCell<Weak<Tree>>,
    pub children: RefCell<Vec<Rc<Tree>>>,
}

pub enum Size {
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
pub enum BorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Repeat {
    Repeat,
    NoRepeat,
    RepeatX,
    RepeatY,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextDecoration {
    None,
    Underline,
    Overline,
    LineThrough,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Float {
    None,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Display {
    #[default]
    None,
    Block,
    Inline,
    InlineBlock,
    Flex,
    Grid,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Justify {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Align {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

pub struct Shadow {
    pub size: Vec<Size>,
    pub color: Color,
    pub blur: u16,
    pub spread: i16,
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
pub struct Sizing {
    pub display: Display,
    pub width: Size,
    pub height: Size,
    pub min_width: Size,
    pub max_width: Size,
    pub min_height: Size,
    pub max_height: Size,
    pub box_sizing: Size,
}

#[derive(Default)]
pub struct Spacing {
    pub margin: Size,
    pub margin_left: Size,
    pub margin_right: Size,
    pub margin_top: Size,
    pub margin_bottom: Size,
    pub padding: Size,
    pub padding_left: Size,
    pub padding_right: Size,
    pub padding_top: Size,
    pub padding_bottom: Size
}

#[derive(Default)]
pub struct Border {
    pub border_width: Size,
    pub border_style: Option<BorderStyle>,
    pub border_color: Color,
    pub border_top: Size,
    pub border_right: Size,
    pub border_bottom: Size,
    pub border_left: Size,
    // box model
    pub box_shadow: Option<Shadow>,
    pub outline: Size,
    pub outline_offset: Size,
}

#[derive(Default)]
pub struct Background {
    pub background_color: Color,
    pub background_image: String,        // path or URL
    pub background_repeat: Option<Repeat>,
    pub background_position: (Size, Size),
    pub background_size: (Size, Size),
}

#[derive(Default)]
pub struct Typography {
    pub color: Color,
    pub font_size: Size,
    pub font_family: String,
    pub font_weight: u16,                // 100 to 900
    pub line_height: Size,
    pub text_align: Option<TextAlign>,
    pub text_decoration: Option<TextDecoration>,
}

#[derive(Default)]
pub struct Positioning {
    pub position: Option<Position>,
    pub float: Option<Float>,
    pub top: Size,
    pub right: Size,
    pub bottom: Size,
    pub left: Size,
    pub z_index: i32,
    pub transform: String,
    pub translate: (Size, Size),
    pub rotate: f32,                             // degrees
}

#[derive(Default)]
pub struct Layout {
    pub display: Option<Display>,                // Flex | Grid
    pub overflow: Option<Overflow>,
    pub visible: bool,
    pub opacity: f32,
    pub flex_direction: Option<FlexDirection>,
    pub justify_content: Option<Justify>,
    pub align_items: Option<Align>,
    pub grid_template_columns: Vec<Size>,
    pub grid_template_rows: Vec<Size>,
}

#[derive(Default)]
pub struct Pseudo {
    pub before: Option<String>,                  // ::before content
    pub after: Option<String>,                   // ::after content
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
