use crate::vnscript::ast::*;
use crate::vnscript::lexer::{Lexer, Token};
use anyhow::{anyhow, Result};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let lexer = Lexer::new(input);
        let tokens: Vec<Token> = lexer.collect();
        Self { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        
        Ok(statements)
    }
    
    fn parse_statement(&mut self) -> Result<Statement> {
        match self.peek() {
            Some(Token::Scene) => self.parse_scene(),
            Some(Token::Character) => self.parse_character(),
            Some(Token::Show) => self.parse_show(),
            Some(Token::Hide) => self.parse_hide(),
            Some(Token::Background) => self.parse_background(),
            Some(Token::Music) => self.parse_music(),
            Some(Token::Sound) => self.parse_sound(),
            Some(Token::Label) => self.parse_label(),
            Some(Token::Jump) => self.parse_jump(),
            Some(Token::Effect) => self.parse_effect(),
            Some(Token::Wait) => self.parse_wait(),
            Some(Token::Video) => self.parse_video(),
            Some(Token::Exit) => {
                self.advance();
                Ok(Statement::Exit)
            }
            Some(Token::Identifier(_)) => self.parse_dialogue(),
            _ => Err(anyhow!("Unexpected token: {:?}", self.peek())),
        }
    }
    
    fn parse_scene(&mut self) -> Result<Statement> {
        self.advance(); // consume @scene
        
        let name = self.expect_identifier()?;
        
        Ok(Statement::Scene(SceneDecl {
            name,
            title: None,
            description: None,
            background: None,
            music: None,
        }))
    }
    
    fn parse_character(&mut self) -> Result<Statement> {
        self.advance(); // consume @character
        
        let id = self.expect_identifier()?;
        
        Ok(Statement::Character(CharacterDecl {
            id,
            name: String::new(),
            color: None,
            default_sprite: None,
        }))
    }
    
    fn parse_show(&mut self) -> Result<Statement> {
        self.advance(); // consume @show
        
        let character = self.expect_identifier()?;
        
        let position = if self.match_token(&Token::At) {
            self.advance();
            match self.peek() {
                Some(Token::Left) => { self.advance(); Position::Left }
                Some(Token::Center) => { self.advance(); Position::Center }
                Some(Token::Right) => { self.advance(); Position::Right }
                _ => Position::Center,
            }
        } else {
            Position::Center
        };
        
        Ok(Statement::Show(ShowCommand {
            character,
            emotion: None,
            position,
            transition: None,
        }))
    }
    
    fn parse_hide(&mut self) -> Result<Statement> {
        self.advance(); // consume @hide
        
        let target = if let Some(Token::Identifier(name)) = self.peek() {
            let name = name.clone();
            self.advance();
            if name == "all" {
                HideTarget::All
            } else {
                HideTarget::Character(name)
            }
        } else {
            HideTarget::All
        };
        
        Ok(Statement::Hide(HideCommand {
            target,
            transition: None,
        }))
    }
    
    fn parse_background(&mut self) -> Result<Statement> {
        self.advance(); // consume @background
        
        let image = self.expect_string()?;
        
        Ok(Statement::Background(BackgroundCommand {
            image,
            transition: None,
        }))
    }
    
    fn parse_music(&mut self) -> Result<Statement> {
        self.advance(); // consume @music
        
        let file = if let Some(Token::String(s)) = self.peek() {
            let s = s.clone();
            self.advance();
            Some(s)
        } else if let Some(token) = self.peek() {
            if let Token::Identifier(action) = token {
                if action == "stop" {
                    self.advance();
                    return Ok(Statement::Music(MusicCommand {
                        action: MusicAction::Stop,
                        file: None,
                        volume: None,
                        fade: None,
                    }));
                }
            }
            None
        } else {
            None
        };
        
        Ok(Statement::Music(MusicCommand {
            action: MusicAction::Play,
            file,
            volume: Some(70.0),
            fade: None,
        }))
    }
    
    fn parse_sound(&mut self) -> Result<Statement> {
        self.advance(); // consume @sound
        
        let file = self.expect_string()?;
        
        Ok(Statement::Sound(SoundCommand {
            file,
            volume: Some(100.0),
        }))
    }
    
    fn parse_label(&mut self) -> Result<Statement> {
        self.advance(); // consume @label
        
        let name = self.expect_identifier()?;
        
        Ok(Statement::Label(name))
    }
    
    fn parse_jump(&mut self) -> Result<Statement> {
        self.advance(); // consume @jump
        
        let target = self.expect_identifier()?;
        
        Ok(Statement::Jump(JumpCommand {
            target,
            condition: None,
        }))
    }
    
    fn parse_effect(&mut self) -> Result<Statement> {
        self.advance(); // consume @effect
        
        let effect_type = if let Some(Token::Identifier(name)) = self.peek() {
            let name = name.clone();
            self.advance();
            match name.as_str() {
                "fade_white" => EffectType::FadeWhite,
                "fade_black" => EffectType::FadeBlack,
                "flash" => EffectType::Flash,
                _ => EffectType::FadeBlack,
            }
        } else {
            EffectType::FadeBlack
        };
        
        Ok(Statement::Effect(EffectCommand {
            effect_type,
            duration: Some(1.0),
        }))
    }
    
    fn parse_wait(&mut self) -> Result<Statement> {
        self.advance(); // consume @wait
        
        let wait_type = if let Some(Token::Float(duration)) = self.peek() {
            let d = *duration as f32;
            self.advance();
            WaitType::Duration(d)
        } else if let Some(Token::Integer(duration)) = self.peek() {
            let d = *duration as f32;
            self.advance();
            WaitType::Duration(d)
        } else {
            WaitType::Click
        };
        
        Ok(Statement::Wait(WaitCommand { wait_type }))
    }
    
    fn parse_video(&mut self) -> Result<Statement> {
        self.advance(); // consume @video
        
        let file = self.expect_string()?;
        
        Ok(Statement::Video(VideoCommand {
            file,
            skip_allowed: true,
        }))
    }
    
    fn parse_dialogue(&mut self) -> Result<Statement> {
        let character = if let Some(Token::Identifier(name)) = self.peek() {
            let name = name.clone();
            self.advance();
            Some(name)
        } else {
            None
        };
        
        self.expect(&Token::Colon)?;
        
        let text = self.expect_string()?;
        
        Ok(Statement::Dialogue(Dialogue {
            character,
            emotion: None,
            text,
            voice: None,
        }))
    }
    
    // Helper methods
    
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
    
    fn match_token(&self, token: &Token) -> bool {
        if let Some(t) = self.peek() {
            std::mem::discriminant(t) == std::mem::discriminant(token)
        } else {
            false
        }
    }
    
    fn expect(&mut self, token: &Token) -> Result<()> {
        if self.match_token(token) {
            self.advance();
            Ok(())
        } else {
            Err(anyhow!("Expected {:?}, got {:?}", token, self.peek()))
        }
    }
    
    fn expect_identifier(&mut self) -> Result<String> {
        if let Some(Token::Identifier(name)) = self.peek() {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(anyhow!("Expected identifier, got {:?}", self.peek()))
        }
    }
    
    fn expect_string(&mut self) -> Result<String> {
        if let Some(Token::String(s)) = self.peek() {
            let s = s.clone();
            self.advance();
            Ok(s)
        } else {
            Err(anyhow!("Expected string, got {:?}", self.peek()))
        }
    }
}
