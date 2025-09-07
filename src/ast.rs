/// Represents a parsed Ween program, which may contain configuration,
/// style definitions, and layout blocks.
#[derive(Debug, Clone)]
pub struct Program {
    /// The kind of program (full, style-only, layout-only, or config-only).
    pub kind: ProgramKind,

    /// Optional language declaration (e.g., `lang="en"`).
    pub lang: Option<String>,

    /// Optional list of style definitions (`def` blocks).
    pub styles: Option<Vec<StyleDefinition>>,

    /// Optional list of layout blocks (`body`, `header`, etc.).
    pub blocks: Option<Vec<Block>>,
}

/// Describes the structural intent of a Ween program.
#[derive(Debug, Clone)]
pub enum ProgramKind {
    /// A complete Ween document with styles and layout.
    Full,

    /// A style-only file (like a `.css` module).
    StyleOnly,

    /// A layout-only file (like a `.html` structure).
    LayoutOnly,

    /// A config-only file (e.g., just `lang="en"`).
    ConfigOnly,
}

/// Represents a top-level construct in a `.wn` file.
#[derive(Debug, Clone)]
pub enum TopLevel {
    /// Language declaration (must be a string literal).
    LangDecl(String),

    /// A reusable style definition block.
    StyleDef(StyleDefinition),

    /// A layout block (e.g., `body`, `header`).
    Block(Block),
}

/// Defines a named style block with parameters and properties.
#[derive(Debug, Clone)]
pub struct StyleDefinition {
    /// The name of the style (e.g., `important-text`).
    pub name: String,

    /// Optional parameters for the style (e.g., `TextFamily`).
    pub params: Vec<String>,

    /// List of style properties (e.g., `font-size = 16px`).
    pub properties: Vec<StyleProperty>,
}

/// Represents a single style property inside a style block or inline options.
#[derive(Debug, Clone)]
pub struct StyleProperty {
    /// The CSS-like key (e.g., `font-size`, `color`).
    pub key: String,

    /// The value assigned to the property.
    pub value: Value,
}

/// Represents a layout block such as `body`, `header`, or custom containers.
#[derive(Debug, Clone)]
pub struct Block {
    /// The name of the block (e.g., `body`, `header`).
    pub name: String,

    /// The child elements contained within the block.
    pub childer: Vec<Element>, // typo: should be `children`
}

/// Represents an HTML-like element with optional styling and content.
#[derive(Debug, Clone)]
pub struct Element {
    /// The tag name (e.g., `h1`, `div`).
    pub tag: String,

    /// Optional style class applied to the element (e.g., `<important-text>`).
    pub style_class: Option<String>,

    /// The content inside the element (text or nested blocks).
    pub content: Vec<Content>,

    /// Optional inline style properties (`opt = { ... }`).
    pub options: Option<Vec<StyleProperty>>,
}

/// Represents the content of an element: either text or a nested block.
#[derive(Debug, Clone)]
pub enum Content {
    /// A string literal (e.g., `"Hello, World!"`).
    Text(String),

    /// A nested block inside an element.
    Block(Block),
}

/// Represents a value assigned to a property or declaration.
#[derive(Debug, Clone)]
pub enum Value {
    /// A quoted string (e.g., `"Red"`, `"UTF-8"`).
    String(String),

    /// A numeric value (e.g., `16px`, `1.0`).
    Number(String),

    /// A bare identifier (e.g., `bold`, `viewport`).
    Identifier(String),
}
