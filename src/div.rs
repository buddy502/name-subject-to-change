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
