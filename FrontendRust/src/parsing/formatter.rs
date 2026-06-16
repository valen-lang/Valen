// From Frontend/ParsingPass/src/dev/vale/parsing/Formatter.scala
// Code formatting utilities



// mig: enum IClass
// mig: case objects W, Ab, Ext, Fn, FnName, FnTplSep, Rune
pub enum IClass {
    W,
    Ab,
    Ext,
    Fn,
    FnName,
    FnTplSep,
    Rune,
}


// mig: enum IElement
pub enum IElement {
    Span(Span),
    Text(Text),
}


// mig: struct Span
pub struct Span {
    pub classs: IClass,
    pub elements: Vec<IElement>,
}


// mig: struct Text
pub struct Text {
    pub string: String,
}

