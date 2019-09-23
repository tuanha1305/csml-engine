use nom::types::*;
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<CompleteByteSlice<'a>>;

pub const PORT: &str = "3002";

pub const WHITE_SPACE: &str = " \t\n\r";
pub const INLINE_COMMENT: &str = "//";

pub const START_COMMENT: &str = "/*";
pub const END_COMMENT: &str = "*/";

pub const ADITION: &str = "+";
pub const SUBTRACTION: &str = "-";
pub const DIVIDE: &str = "/";
pub const MULTIPLY: &str = "*";
pub const REMAINDER: &str = "%";
pub const NOT: &str = "!";

pub const EQUAL: &str = "==";
pub const NOT_EQUAL: &str = "!=";
pub const ASSIGN: &str = "=";

pub const OR: &str = "||";
pub const AND: &str = "&&";

pub const GREATER_THAN_EQUAL: &str = ">=";
pub const LESS_THAN_EQUAL: &str = "<=";
pub const GREATER_THAN: &str = ">";
pub const LESS_THAN: &str = "<";

pub const COMMA: &str = ",";
pub const DOT: &str = ".";
pub const SEMICOLON: &str = ";";
pub const COLON: &str = ":";
pub const DOUBLE_QUOTE: &str = "\"";

pub const L_PAREN: &str = "(";
pub const R_PAREN: &str = ")";
pub const L_BRACE: &str = "{";
pub const R_BRACE: &str = "}";
pub const L_BRACKET: &str = "[";
pub const R_BRACKET: &str = "]";
pub const L2_BRACE: &str = "{{";
pub const R2_BRACE: &str = "}}";

pub const FOREACH: &str = "foreach";
pub const IF: &str = "if";
pub const ELSE: &str = "else";

pub const IMPORT: &str = "import";
pub const AS: &str = "as";
pub const IN: &str = "in";
pub const FROM: &str = "from";
pub const EVENT: &str = "event";
pub const RETRIES: &str = "retries";

pub const FLOW: &str = "flow";
pub const FILE: &str = "file";
pub const STEP: &str = "step";
pub const SAY: &str = "say";
pub const USE: &str = "use";
pub const ASK: &str = "ask";
pub const GOTO: &str = "goto";
pub const MATCH: &str = "match";
pub const DEFAULT: &str = "default";
pub const RESPONSE: &str = "response";
pub const REMEMBER: &str = "remember";

pub const TRUE: &str = "true";
pub const FALSE: &str = "false";
pub const NULL: &str = "NULL";

pub static RESERVED: &[&str] = &[
    FOREACH, IF, ELSE, IMPORT, AS, IN, FROM, EVENT, RETRIES, FLOW, FILE, STEP, SAY, USE, ASK, GOTO,
    MATCH, DEFAULT, RESPONSE, REMEMBER, TRUE, FALSE, NULL,
];

pub const TYPING: &str = "Typing";
pub const WAIT: &str = "Wait";
pub const TEXT: &str = "Text";
pub const URL: &str = "Url";
pub const IMAGE: &str = "Image";
pub const ONE_OF: &str = "OneOf";
pub const SHUFFLE: &str = "Shuffle";
pub const LENGTH: &str = "Length";
pub const FIND: &str = "Find";
pub const RANDOM: &str = "Random";
pub const FLOOR: &str = "Floor";

pub const QUESTION: &str = "Question";
pub const BUTTON: &str = "Button";
pub const OBJECT: &str = "Object";
pub const FN: &str = "Fn";

pub static BUILT_IN: &[&str] = &[
    TYPING, WAIT, TEXT, URL, IMAGE, ONE_OF, SHUFFLE, LENGTH, FIND, RANDOM, FLOOR, QUESTION, BUTTON,
    OBJECT, FN,
];

pub const PAST: &str = "past";
pub const MEMORY: &str = "memory";
pub const METADATA: &str = "metadata";

pub const FROMEFILE: &str = "FromFile";
pub const GET_VALUE: &str = "getvalue";
pub const FIRST: &str = "first";
