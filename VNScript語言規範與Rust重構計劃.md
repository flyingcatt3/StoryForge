# VNScript 語言規範與 Rust 重構計劃

## 目錄
1. [VNScript 語言設計](#vnscript-語言設計)
2. [語法規範](#語法規範)
3. [Rust 重構架構](#rust-重構架構)
4. [實作計劃](#實作計劃)
5. [範例專案](#範例專案)

---

## VNScript 語言設計

### 設計理念

VNScript 是一種專為視覺小說設計的領域特定語言 (DSL)，具備以下特點：

1. **簡潔易讀**: 非程式設計師也能輕鬆編寫劇本
2. **強型別**: 編譯期錯誤檢查，減少執行期錯誤
3. **表達力強**: 支援條件分支、變數系統、函數定義
4. **向後相容**: 支援現有的簡單指令格式
5. **可擴展**: 支援自定義指令和插件系統

### 語言特性

- 宣告式語法設計
- 支援註解和多行字串
- 內建變數系統和表達式
- 支援條件分支和選擇
- 支援函數定義和呼叫
- 支援場景標籤和跳轉
- 支援動畫和過渡效果
- 內建本地化支援

---

## 語法規範

### 1. 基礎語法

#### 註解
```vnscript
# 這是單行註解

### 
這是多行註解
可以跨多行
###
```

#### 場景定義
```vnscript
@scene prologue
  @title "序章"
  @description "故事的開始"
  @background "prologue_bg.jpg"
  @music "opening_theme.mp3" volume:70 loop:true
```

### 2. 角色系統

#### 角色定義
```vnscript
@character alice
  @name "愛麗絲"
  @color #4A90E2
  @default_sprite "alice_neutral.png"
  @voice "alice_voice"

@character bob
  @name "鮑勃"
  @color #E24A4A
  @default_sprite "bob_neutral.png"
```

#### 角色對話
```vnscript
# 基本對話
alice: "你好，很高興見到你。"

# 帶表情的對話
alice(happy): "太好了！我等你很久了！"

# 多行對話
alice: """
這是一段很長的對話，
可以跨越多行，
讓劇本更容易閱讀。
"""

# 旁白（無角色名）
: "天空逐漸暗了下來..."
```

### 3. 角色動作與位置

#### 顯示角色
```vnscript
@show alice at left with fadeIn
@show bob at center with slideIn
@show charlie at right

# 帶表情和動畫
@show alice(happy) at left with bounce
```

#### 隱藏角色
```vnscript
@hide alice with fadeOut
@hide bob
@hide all  # 隱藏所有角色
```

#### 移動角色
```vnscript
@move alice from:left to:center duration:1.0
@move bob to:right with slideOut
```

### 4. 場景控制

#### 背景切換
```vnscript
@background "sunset.jpg"
@background "night_sky.jpg" with dissolve duration:2.0
```

#### 特效
```vnscript
@effect fade_white duration:1.5
@effect fade_black duration:2.0
@effect flash
@effect shake intensity:0.5 duration:0.3
@effect blur radius:5 duration:1.0
```

#### 視訊播放
```vnscript
@video "opening_cutscene.mp4"
  @skip_allowed true
  @on_complete -> main_story
```

### 5. 音訊控制

#### 背景音樂
```vnscript
@music "battle_theme.mp3" volume:80 loop:true fadeIn:2.0
@music stop fadeOut:3.0
@music pause
@music resume
```

#### 音效
```vnscript
@sound "door_open.wav" volume:100
@sound "footsteps.wav" volume:60 loop:true
@sound stop:"footsteps.wav"
```

#### 語音
```vnscript
alice: "這段對話有配音。" @voice:"alice_001.wav"
```

### 6. 變數系統

#### 變數定義與操作
```vnscript
# 定義變數
@var player_name = "玩家"
@var affection_alice = 0
@var game_completed = false
@var current_chapter = 1

# 數學運算
@set affection_alice = affection_alice + 10
@set current_chapter += 1

# 字串操作
@set full_greeting = "你好，" + player_name
```

### 7. 條件分支

#### if-else 語句
```vnscript
@if affection_alice >= 50
  alice(happy): "我真的很喜歡你！"
  @set alice_route = true
@elif affection_alice >= 20
  alice(neutral): "你是個不錯的朋友。"
@else
  alice(sad): "抱歉，我們不太合適..."
@endif
```

#### 條件表達式
```vnscript
@show alice(happy) if:affection_alice > 30 else:alice(neutral)
```

### 8. 選擇系統

#### 基本選擇
```vnscript
@choice "你要做什麼？"
  @option "幫助愛麗絲" -> help_alice
  @option "保持中立" -> stay_neutral
  @option "拒絕幫忙" -> refuse_help condition:courage >= 5

@label help_alice
  @set affection_alice += 20
  alice(happy): "謝謝你願意幫我！"
  -> continue_story

@label stay_neutral
  alice(neutral): "好吧，我理解。"
  -> continue_story

@label refuse_help
  @set affection_alice -= 10
  alice(sad): "我明白了..."
  -> continue_story

@label continue_story
  # 繼續主線劇情
```

#### 計時選擇
```vnscript
@choice timeout:10 default:option_1
  @option "快速反應" -> quick_response
  @option "仔細思考" -> slow_response
```

### 9. 跳轉與標籤

#### 標籤定義
```vnscript
@label main_menu
  # 主選單邏輯

@label game_start
  # 遊戲開始

@label bad_ending
  # 壞結局
```

#### 跳轉
```vnscript
@jump game_start
@jump bad_ending if:affection_alice < 10
@call common_event  # 呼叫後會返回
```

### 10. UI 控制

#### 文字框樣式
```vnscript
@textbox style:default
@textbox style:transparent opacity:0.5
@textbox hide
@textbox show
```

#### 顯示位置資訊
```vnscript
@location "東京，秋葉原" duration:3.0
```

#### 標題顯示
```vnscript
@title """
第一章
命運的相遇
""" duration:5.0
```

### 11. 動畫與過渡效果

#### 預定義過渡效果
```vnscript
# 淡入淡出
with fadeIn
with fadeOut
with dissolve

# 滑動
with slideIn
with slideOut
with slideLeft
with slideRight

# 特殊效果
with bounce
with zoom
with rotate
```

#### 自定義動畫
```vnscript
@animate alice
  @property opacity from:0 to:1 duration:1.0 easing:ease_in_out
  @property x from:0 to:100 duration:0.5
```

### 12. 存檔與成就

#### 存檔點
```vnscript
@savepoint "chapter_1_complete"
@autosave
```

#### 成就解鎖
```vnscript
@achievement unlock:"first_meeting"
@achievement progress:"collector" value:1
```

### 13. 函數與巨集

#### 函數定義
```vnscript
@function greet(character_name, mood)
  @if mood == "happy"
    @show {character_name}(happy) at center
    {character_name}: "很高興見到你！"
  @else
    @show {character_name}(neutral) at center
    {character_name}: "你好。"
  @endif
@endfunction

# 呼叫函數
@call greet("alice", "happy")
```

#### 巨集
```vnscript
@macro scene_transition(new_bg, new_music)
  @effect fade_black duration:1.0
  @background {new_bg}
  @music {new_music}
  @effect fade_black duration:1.0
@endmacro

# 使用巨集
@use scene_transition("forest.jpg", "forest_theme.mp3")
```

### 14. 進階功能

#### 平行事件
```vnscript
@parallel
  @task
    @show alice at left with fadeIn
  @task
    @show bob at right with fadeIn
  @task
    @music "meeting_theme.mp3" fadeIn:2.0
@endparallel
```

#### 等待
```vnscript
@wait 2.0  # 等待 2 秒
@wait_for_click  # 等待玩家點擊
@wait_for_voice  # 等待語音播放完畢
```

#### 相機控制
```vnscript
@camera zoom:1.5 duration:2.0
@camera pan:left duration:1.0
@camera shake intensity:0.3 duration:0.5
@camera reset duration:1.0
```

### 15. 本地化支援

#### 多語言文本
```vnscript
@text greeting
  @lang zh-TW "你好，歡迎來到這個世界！"
  @lang en-US "Hello, welcome to this world!"
  @lang ja-JP "こんにちは、この世界へようこそ！"

alice: ${greeting}
```

---

## Rust 重構架構

### 整體架構圖

```
StoryForge (Rust)
│
├── vnscript/                  # VNScript 語言核心
│   ├── lexer.rs              # 詞法分析器
│   ├── parser.rs             # 語法分析器
│   ├── ast.rs                # 抽象語法樹定義
│   ├── semantic.rs           # 語義分析
│   ├── compiler.rs           # 編譯器
│   └── runtime.rs            # 運行時
│
├── engine/                    # 遊戲引擎核心
│   ├── mod.rs
│   ├── game_state.rs         # 遊戲狀態管理
│   ├── scene_manager.rs      # 場景管理
│   ├── character_manager.rs  # 角色管理
│   ├── resource_manager.rs   # 資源管理
│   └── command_executor.rs   # 指令執行器
│
├── media/                     # 多媒體系統
│   ├── mod.rs
│   ├── audio_player.rs       # 音訊播放
│   ├── video_player.rs       # 視訊播放
│   ├── image_renderer.rs     # 圖片渲染
│   └── animation.rs          # 動畫系統
│
├── ui/                        # 使用者介面
│   ├── mod.rs
│   ├── main_menu.rs          # 主選單
│   ├── game_screen.rs        # 遊戲畫面
│   ├── textbox.rs            # 對話框
│   ├── choice_menu.rs        # 選擇選單
│   └── settings_menu.rs      # 設定選單
│
├── save/                      # 存檔系統
│   ├── mod.rs
│   ├── save_manager.rs       # 存檔管理
│   ├── save_data.rs          # 存檔資料結構
│   └── serialization.rs      # 序列化
│
├── localization/              # 本地化
│   ├── mod.rs
│   └── i18n.rs               # 國際化支援
│
└── main.rs                    # 程式入口
```

### 核心模組設計

#### 1. VNScript 詞法分析器 (Lexer)

```rust
// vnscript/lexer.rs
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // 關鍵字
    #[token("@scene")]
    Scene,
    
    #[token("@character")]
    Character,
    
    #[token("@show")]
    Show,
    
    #[token("@hide")]
    Hide,
    
    #[token("@background")]
    Background,
    
    #[token("@music")]
    Music,
    
    #[token("@sound")]
    Sound,
    
    #[token("@if")]
    If,
    
    #[token("@elif")]
    Elif,
    
    #[token("@else")]
    Else,
    
    #[token("@endif")]
    EndIf,
    
    #[token("@choice")]
    Choice,
    
    #[token("@option")]
    Option,
    
    #[token("@label")]
    Label,
    
    #[token("@jump")]
    Jump,
    
    #[token("@call")]
    Call,
    
    #[token("@var")]
    Var,
    
    #[token("@set")]
    Set,
    
    // 位置關鍵字
    #[token("at")]
    At,
    
    #[token("with")]
    With,
    
    #[token("to")]
    To,
    
    #[token("from")]
    From,
    
    // 字面值
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    String(String),
    
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),
    
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),
    
    #[regex(r"true|false", |lex| lex.slice() == "true")]
    Boolean(bool),
    
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),
    
    // 運算符
    #[token("=")]
    Assign,
    
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Multiply,
    
    #[token("/")]
    Divide,
    
    #[token("==")]
    Equal,
    
    #[token("!=")]
    NotEqual,
    
    #[token(">")]
    Greater,
    
    #[token("<")]
    Less,
    
    #[token(">=")]
    GreaterEqual,
    
    #[token("<=")]
    LessEqual,
    
    // 分隔符
    #[token(":")]
    Colon,
    
    #[token(",")]
    Comma,
    
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
    
    #[token("->")]
    Arrow,
    
    // 註解和空白
    #[regex(r"#[^\n]*", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    
    // 錯誤
    #[error]
    Error,
}

pub struct Lexer<'a> {
    tokens: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            tokens: Token::lexer(input),
        }
    }
    
    pub fn next_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}
```

#### 2. 抽象語法樹 (AST)

```rust
// vnscript/ast.rs
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
```

#### 3. 遊戲引擎核心

```rust
// engine/game_state.rs
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub variables: HashMap<String, Value>,
    pub current_scene: String,
    pub current_label: Option<String>,
    pub statement_index: usize,
    pub call_stack: Vec<CallFrame>,
    pub choices_made: Vec<ChoiceRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallFrame {
    pub return_scene: String,
    pub return_label: Option<String>,
    pub return_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceRecord {
    pub scene: String,
    pub index: usize,
    pub selected: usize,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            current_scene: String::new(),
            current_label: None,
            statement_index: 0,
            call_stack: Vec::new(),
            choices_made: Vec::new(),
        }
    }
    
    pub fn get_var(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    
    pub fn set_var(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    
    pub fn evaluate_condition(&self, expr: &Expression) -> Result<bool, String> {
        match self.evaluate_expression(expr)? {
            Value::Boolean(b) => Ok(b),
            _ => Err("Expression did not evaluate to boolean".to_string()),
        }
    }
    
    pub fn evaluate_expression(&self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Integer(i) => Ok(Value::Integer(*i)),
            Expression::Float(f) => Ok(Value::Float(*f)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Boolean(b) => Ok(Value::Boolean(*b)),
            Expression::Variable(name) => {
                self.get_var(name)
                    .cloned()
                    .ok_or_else(|| format!("Variable '{}' not found", name))
            }
            Expression::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.apply_binary_op(&left_val, op, &right_val)
            }
            Expression::UnaryOp(op, expr) => {
                let val = self.evaluate_expression(expr)?;
                self.apply_unary_op(op, &val)
            }
        }
    }
    
    fn apply_binary_op(&self, left: &Value, op: &BinaryOperator, right: &Value) -> Result<Value, String> {
        match (left, op, right) {
            (Value::Integer(l), BinaryOperator::Add, Value::Integer(r)) => Ok(Value::Integer(l + r)),
            (Value::Integer(l), BinaryOperator::Subtract, Value::Integer(r)) => Ok(Value::Integer(l - r)),
            (Value::Integer(l), BinaryOperator::Multiply, Value::Integer(r)) => Ok(Value::Integer(l * r)),
            (Value::Integer(l), BinaryOperator::Divide, Value::Integer(r)) => Ok(Value::Integer(l / r)),
            (Value::Integer(l), BinaryOperator::Equal, Value::Integer(r)) => Ok(Value::Boolean(l == r)),
            (Value::Integer(l), BinaryOperator::NotEqual, Value::Integer(r)) => Ok(Value::Boolean(l != r)),
            (Value::Integer(l), BinaryOperator::Greater, Value::Integer(r)) => Ok(Value::Boolean(l > r)),
            (Value::Integer(l), BinaryOperator::Less, Value::Integer(r)) => Ok(Value::Boolean(l < r)),
            (Value::Integer(l), BinaryOperator::GreaterEqual, Value::Integer(r)) => Ok(Value::Boolean(l >= r)),
            (Value::Integer(l), BinaryOperator::LessEqual, Value::Integer(r)) => Ok(Value::Boolean(l <= r)),
            (Value::Boolean(l), BinaryOperator::And, Value::Boolean(r)) => Ok(Value::Boolean(*l && *r)),
            (Value::Boolean(l), BinaryOperator::Or, Value::Boolean(r)) => Ok(Value::Boolean(*l || *r)),
            _ => Err("Type mismatch in binary operation".to_string()),
        }
    }
    
    fn apply_unary_op(&self, op: &UnaryOperator, val: &Value) -> Result<Value, String> {
        match (op, val) {
            (UnaryOperator::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
            (UnaryOperator::Negate, Value::Integer(i)) => Ok(Value::Integer(-i)),
            (UnaryOperator::Negate, Value::Float(f)) => Ok(Value::Float(-f)),
            _ => Err("Type mismatch in unary operation".to_string()),
        }
    }
}
```

#### 4. 資源管理器

```rust
// engine/resource_manager.rs
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use image::DynamicImage;
use anyhow::Result;

pub struct ResourceManager {
    base_path: PathBuf,
    image_cache: HashMap<String, Arc<DynamicImage>>,
    audio_cache: HashMap<String, Arc<Vec<u8>>>,
}

impl ResourceManager {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            image_cache: HashMap::new(),
            audio_cache: HashMap::new(),
        }
    }
    
    pub fn load_image(&mut self, path: &str) -> Result<Arc<DynamicImage>> {
        if let Some(cached) = self.image_cache.get(path) {
            return Ok(Arc::clone(cached));
        }
        
        let full_path = self.base_path.join(path);
        let image = image::open(full_path)?;
        let arc_image = Arc::new(image);
        self.image_cache.insert(path.to_string(), Arc::clone(&arc_image));
        
        Ok(arc_image)
    }
    
    pub fn load_audio(&mut self, path: &str) -> Result<Arc<Vec<u8>>> {
        if let Some(cached) = self.audio_cache.get(path) {
            return Ok(Arc::clone(cached));
        }
        
        let full_path = self.base_path.join(path);
        let data = std::fs::read(full_path)?;
        let arc_data = Arc::new(data);
        self.audio_cache.insert(path.to_string(), Arc::clone(&arc_data));
        
        Ok(arc_data)
    }
    
    pub fn clear_cache(&mut self) {
        self.image_cache.clear();
        self.audio_cache.clear();
    }
    
    pub fn preload_resources(&mut self, script: &[Statement]) -> Result<()> {
        for statement in script {
            match statement {
                Statement::Background(cmd) => {
                    self.load_image(&cmd.image)?;
                }
                Statement::Show(cmd) => {
                    // 預載入角色精靈圖
                }
                Statement::Music(cmd) => {
                    if let Some(file) = &cmd.file {
                        self.load_audio(file)?;
                    }
                }
                Statement::Sound(cmd) => {
                    self.load_audio(&cmd.file)?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
```

#### 5. 存檔系統

```rust
// save/save_manager.rs
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::Result;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub timestamp: DateTime<Utc>,
    pub game_state: GameState,
    pub screenshot: Option<Vec<u8>>,
    pub play_time: u64,
}

pub struct SaveManager {
    save_dir: PathBuf,
}

impl SaveManager {
    pub fn new(save_dir: impl AsRef<Path>) -> Self {
        Self {
            save_dir: save_dir.as_ref().to_path_buf(),
        }
    }
    
    pub fn save(&self, slot: usize, data: &SaveData) -> Result<()> {
        let path = self.save_dir.join(format!("save_{}.bin", slot));
        let bytes = bincode::serialize(data)?;
        std::fs::write(path, bytes)?;
        Ok(())
    }
    
    pub fn load(&self, slot: usize) -> Result<SaveData> {
        let path = self.save_dir.join(format!("save_{}.bin", slot));
        let bytes = std::fs::read(path)?;
        let data = bincode::deserialize(&bytes)?;
        Ok(data)
    }
    
    pub fn delete(&self, slot: usize) -> Result<()> {
        let path = self.save_dir.join(format!("save_{}.bin", slot));
        std::fs::remove_file(path)?;
        Ok(())
    }
    
    pub fn list_saves(&self) -> Result<Vec<(usize, SaveData)>> {
        let mut saves = Vec::new();
        
        for entry in std::fs::read_dir(&self.save_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("save_") && name.ends_with(".bin") {
                    if let Some(slot_str) = name.strip_prefix("save_").and_then(|s| s.strip_suffix(".bin")) {
                        if let Ok(slot) = slot_str.parse::<usize>() {
                            if let Ok(data) = self.load(slot) {
                                saves.push((slot, data));
                            }
                        }
                    }
                }
            }
        }
        
        saves.sort_by_key(|(slot, _)| *slot);
        Ok(saves)
    }
    
    pub fn auto_save(&self, data: &SaveData) -> Result<()> {
        self.save(0, data)
    }
    
    pub fn load_auto_save(&self) -> Result<SaveData> {
        self.load(0)
    }
}
```

---

## 實作計劃

### Phase 1: VNScript 語言核心 (2-3週)

#### Week 1: 詞法與語法分析
- [ ] 實作 Lexer (使用 logos crate)
- [ ] 設計並實作 AST 結構
- [ ] 實作基礎 Parser
- [ ] 撰寫單元測試

#### Week 2: 語義分析與編譯器
- [ ] 實作語義分析器（型別檢查、變數檢查）
- [ ] 實作編譯器（AST -> 字節碼）
- [ ] 錯誤處理與報告系統
- [ ] 撰寫整合測試

#### Week 3: 運行時與最佳化
- [ ] 實作運行時系統
- [ ] 實作變數系統和表達式求值
- [ ] 最佳化編譯產物
- [ ] 效能測試與調優

### Phase 2: 遊戲引擎核心 (3-4週)

#### Week 4-5: 核心系統
- [ ] 遊戲狀態管理 (GameState)
- [ ] 場景管理器 (SceneManager)
- [ ] 角色管理器 (CharacterManager)
- [ ] 資源管理器 (ResourceManager)
- [ ] 指令執行器 (CommandExecutor)

#### Week 6: 多媒體系統
- [ ] 音訊播放器（使用 rodio）
- [ ] 圖片渲染（使用 image）
- [ ] 動畫系統（補間動畫）
- [ ] 視訊播放器（使用 gstreamer 或 ffmpeg）

#### Week 7: 存檔與設定
- [ ] 存檔系統
- [ ] 設定系統
- [ ] 成就系統
- [ ] 統計追蹤

### Phase 3: UI 層實作 (3-4週)

#### 選項 A: 使用 egui（推薦）
```rust
// 優勢：純 Rust、跨平台、即時模式 GUI
dependencies = [
    "egui",
    "eframe",
]
```

#### 選項 B: 使用 iced
```rust
// 優勢：宣告式 UI、優雅的架構
dependencies = [
    "iced",
]
```

#### Week 8-9: 核心 UI 組件
- [ ] 主選單
- [ ] 遊戲畫面
- [ ] 對話框組件
- [ ] 選擇選單
- [ ] 快速選單

#### Week 10-11: 進階 UI
- [ ] 設定介面
- [ ] 存讀檔介面
- [ ] 歷史記錄
- [ ] 圖庫/音樂鑑賞

### Phase 4: 整合與優化 (2-3週)

#### Week 12: 功能整合
- [ ] 整合所有模組
- [ ] 端到端測試
- [ ] Bug 修復

#### Week 13: 效能優化
- [ ] 資源預載入優化
- [ ] 記憶體使用優化
- [ ] 渲染效能優化
- [ ] 啟動時間優化

#### Week 14: 打包與發布
- [ ] 跨平台建置（Windows/macOS/Linux）
- [ ] 安裝程式製作
- [ ] 文件編寫
- [ ] 發布準備

### 技術選型建議

#### 核心依賴
```toml
[dependencies]
# 語言解析
logos = "0.13"
pest = "2.7"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"

# 錯誤處理
anyhow = "1.0"
thiserror = "1.0"

# 圖像處理
image = "0.24"

# 音訊
rodio = "0.17"

# UI（選擇其一）
egui = "0.23"
eframe = "0.23"
# 或
iced = "0.10"

# 非同步運行時
tokio = { version = "1.0", features = ["full"] }

# 其他工具
chrono = "0.4"
uuid = { version = "1.0", features = ["v4"] }
log = "0.4"
env_logger = "0.10"
```

---

## 範例專案

### 完整的 VNScript 範例

```vnscript
# ========================================
# 視覺小說範例：命運的相遇
# ========================================

### 
這是一個展示 VNScript 所有功能的範例專案
包含：對話、選擇、分支、變數、動畫等
###

# ----------------------------------------
# 角色定義
# ----------------------------------------

@character alice
  @name "愛麗絲"
  @color #4A90E2
  @default_sprite "alice_neutral.png"

@character bob
  @name "鮑勃"
  @color #E24A4A
  @default_sprite "bob_neutral.png"

@character narrator
  @name "旁白"
  @color #888888

# ----------------------------------------
# 變數初始化
# ----------------------------------------

@var player_name = "玩家"
@var affection_alice = 0
@var affection_bob = 0
@var courage = 5
@var current_day = 1

# ----------------------------------------
# 場景：序章
# ----------------------------------------

@scene prologue
  @title "序章"
  @description "故事的開始"
  @background "school_entrance.jpg"
  @music "peaceful_morning.mp3" volume:70 loop:true

@label start

# 淡入效果
@effect fade_black duration:2.0

# 顯示地點
@location "櫻花高中，校門口" duration:3.0

# 旁白
narrator: """
這是一個春天的早晨，
櫻花隨風飄落，
你踏入了新的學校。
"""

@wait_for_click

# 顯示標題
@title """
命運的相遇
第一章
""" duration:5.0

@wait 2.0

# ----------------------------------------
# 第一次遇見愛麗絲
# ----------------------------------------

@label meet_alice

@background "school_hallway.jpg" with dissolve duration:1.5
@music "daily_life.mp3" fadeIn:2.0

narrator: "你走在走廊上，突然..."

@sound "bump.wav" volume:80

# 愛麗絲出現
@show alice(surprised) at center with fadeIn

alice(surprised): "啊！對不起！"

@wait 0.5

alice(embarrassed): """
我剛才沒注意看路，
有沒有撞到你？
"""

# 玩家選擇
@choice "你要怎麼回應？"
  @option "沒關係，你還好嗎？" -> kind_response
  @option "下次小心一點。" -> neutral_response
  @option "你走路不看路嗎？" -> rude_response condition:courage >= 10

@label kind_response
  @set affection_alice += 20
  
  alice(happy): """
  謝謝你這麼體貼！
  你是新來的學生吧？
  """
  
  : "愛麗絲的好感度上升了！"
  
  -> introduce_alice

@label neutral_response
  @set affection_alice += 5
  
  alice(neutral): "嗯，我會的。你是新生？"
  
  -> introduce_alice

@label rude_response
  @set affection_alice -= 10
  
  alice(sad): "對不起...我真的不是故意的..."
  
  @hide alice with fadeOut
  
  narrator: "愛麗絲傷心地離開了..."
  
  -> bad_first_impression

@label introduce_alice

alice(happy): """
對了，我叫愛麗絲。
很高興認識你！
你叫什麼名字？
"""

# 這裡可以實作輸入系統
@input player_name prompt:"請輸入你的名字："

alice(happy): "{{player_name}}...好特別的名字！"

@wait_for_click

alice(neutral): """
我得趕去上課了，
下次再聊吧！
"""

@hide alice with slideOut

narrator: "就這樣，你和愛麗絲第一次見面了。"

-> chapter_1_end

# ----------------------------------------
# 分支：第一印象不好
# ----------------------------------------

@label bad_first_impression

narrator: """
你覺得有些後悔自己的態度...
也許應該更友善一些。
"""

@set courage -= 5

-> chapter_1_end

# ----------------------------------------
# 第一章結束
# ----------------------------------------

@label chapter_1_end

@effect fade_black duration:2.0

@background "black.jpg"
@music stop fadeOut:3.0

@title """
第一章 完
待續...
""" duration:5.0

@wait 3.0

# 顯示統計
@function show_stats()
  @textbox style:transparent
  
  narrator: """
  ===== 章節統計 =====
  愛麗絲好感度: {{affection_alice}}
  鮑勃好感度: {{affection_bob}}
  勇氣值: {{courage}}
  ==================
  """
@endfunction

@call show_stats()

@wait_for_click

# 自動存檔
@autosave

# 跳轉到下一章或主選單
@choice "你想要..."
  @option "繼續下一章" -> chapter_2
  @option "返回主選單" -> main_menu
  @option "退出遊戲" -> game_exit

@label chapter_2
  # 第二章內容...
  @jump chapter_2_start

@label main_menu
  @jump main_menu

@label game_exit
  @exit
```

### Cargo.toml 配置

```toml
[package]
name = "storyforge"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A modern visual novel engine powered by VNScript and Rust"
license = "MIT"

[dependencies]
# 語言解析
logos = "0.13"
pest = "2.7"
pest_derive = "2.7"

# 序列化
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
bincode = "1.3"
toml = "0.8"

# 錯誤處理
anyhow = "1.0"
thiserror = "1.0"

# 圖像處理
image = "0.24"

# 音訊
rodio = "0.17"

# UI 框架
egui = "0.23"
eframe = { version = "0.23", features = ["default_fonts", "persistence"] }

# 非同步運行時
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"

# 其他工具
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
log = "0.4"
env_logger = "0.10"
directories = "5.0"

# 視訊處理（可選）
# gstreamer = { version = "0.21", optional = true }

[dev-dependencies]
criterion = "0.5"
proptest = "1.0"

[[bench]]
name = "script_parsing"
harness = false

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.dev]
opt-level = 1

[features]
default = []
video = []  # 視訊支援功能標誌
```

---

## 總結

這份文件提供了：

1. **完整的 VNScript 語言規範**
   - 簡潔易讀的語法設計
   - 豐富的功能支援（對話、選擇、變數、動畫等）
   - 向後相容現有的簡單指令格式

2. **Rust 重構架構設計**
   - 模組化的專案結構
   - 清晰的職責分離
   - 高效能的資源管理

3. **詳細的實作計劃**
   - 分階段開發策略
   - 12-14週完成
   - 包含測試和優化

4. **完整的範例專案**
   - 展示所有語言特性
   - 可直接用於測試
   - 包含配置檔案

### 下一步行動

1. 審查並確認語言規範
2. 建立專案骨架
3. 開始實作 Phase 1（VNScript 核心）
4. 持續迭代和改進

這個方案利用 Rust 的優勢（效能、記憶體安全、並發性），結合專為視覺小說設計的 VNScript 語言，將大幅提升專案的品質、可維護性和效能。
