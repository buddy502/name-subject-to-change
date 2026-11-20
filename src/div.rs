/*
    Html Div functionality

    Implement Html tags using trait format
    e.g. trait Width {
        fn create_width_for_tag(&self) -> Box_control(idk);
    }
    // Every CSS html property I'm implementing
    Layout & sizing:
        display, width, height, min-width, max-width, min-height, max-height, box-sizing
        
    Spacing:
        margin, margin-left, margin-right, margin-top, margin-bottom, padding,
        padding-left, padding-right, padding-top, padding-bottom
        
    Borders:
        border, border-width, border-style, border-color, border-top, border-right, etc.
        
    Backgrounds:
        background-color, background-image, background-repeat, background-position, background-size
        
    Text & typography:
        color, font-size, font-family, font-weight, line-height, text-align, text-decoration
        
    Positioning:
        position, top, right, bottom, left, z-index, transform, translate, rotate
        
    Box model:
        box-shadow, outline, outline-offset
        
    Flexbox/Grid:
        display (flex | grid), flex-direction, justify-content, align-items,
        grid-template-columns, grid-template-rows
        
    Pseudo-elements:
        ::before, ::after (for content inside <div>)
*/
/*
    Every CSS property is going to be in the same file as the html declarations
        It looks like a lot of .'s but it's needed for readability trust, if not then a
    struct holds like 100 different CSS properties
    e.g. let div = create_div(); div.style.spacing.margin = px(20.0);
    Used for inline CSS, no external file css needed

    Div creates a default implementation that the user can change
*/

use crate::properties::Styles;

// can only use certain CSS/html 
pub struct Div {
    pub class: Option<String>,
    pub id: Option<String>,

    // !! Default Div implementation of CSS properties required
    pub styles: Option<Styles>
}

impl Div {
    pub fn create_div() {
    }
}
