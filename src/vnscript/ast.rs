use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Scene(SceneDecl),
    Character(CharacterDecl),
    Dialogue(Dialogue),
    Show(ShowCommand),
    Hide(HideCommand),
    Background(BackgroundCommand),
    Music(MusicCommand),
    Sound(SoundCommand),
    If(IfStatement),
    Choice(ChoiceStatement),
    Label(String),
    Jump(JumpCommand),
    Call(String),
    VarDecl(VarDecl),
    Assignment(Assignment),
    Effect(EffectCommand),
    Wait(WaitCommand),
    Video(VideoCommand),
    Exit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDecl {
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub background: Option<String>,
    pub music: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDecl {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub default_sprite: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub character: Option<String>,
    pub emotion: Option<String>,
    pub text: String,
    pub voice: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowCommand {
    pub character: String,
    pub emotion: Option<String>,
    pub position: Position,
    pub transition: Option<Transition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HideCommand {
    pub target: HideTarget,
    pub transition: Option<Transition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HideTarget {
    Character(String),
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position {
    Left,
    Center,
    Right,
    Custom(f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub name: String,
    pub duration: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundCommand {
    pub image: String,
    pub transition: Option<Transition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicCommand {
    pub action: MusicAction,
    pub file: Option<String>,
    pub volume: Option<f32>,
    pub fade: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicAction {
    Play,
    Stop,
    Pause,
    Resume,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundCommand {
    pub file: String,
    pub volume: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Vec<Statement>,
    pub elif_blocks: Vec<(Expression, Vec<Statement>)>,
    pub else_block: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceStatement {
    pub prompt: String,
    pub options: Vec<ChoiceOption>,
    pub timeout: Option<f32>,
    pub default: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub text: String,
    pub target: String,
    pub condition: Option<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpCommand {
    pub target: String,
    pub condition: Option<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarDecl {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Variable(String),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
    UnaryOp(UnaryOperator, Box<Expression>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Negate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectCommand {
    pub effect_type: EffectType,
    pub duration: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    FadeWhite,
    FadeBlack,
    Flash,
    Shake { intensity: f32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitCommand {
    pub wait_type: WaitType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WaitType {
    Duration(f32),
    Click,
    Voice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCommand {
    pub file: String,
    pub skip_allowed: bool,
}
