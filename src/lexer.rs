#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // ... loop tokens ...
    WheelTurns,
    FromFirstDawn,
    UntilMiddleDawn(i64),
    LetChronicler,
    WithinHouseCounter,
    EachStep,
    MustBeWitnessed,

    // Dynamic numeric conditionals
    IfStarsAlignGeneric(i64, i64), // "If the stars align and the cycle bows to both the [X] and the [Y] simultaneously"
    ButIfDivisibleGeneric(i64),    // "But if the number of the current cycle can be divided cleanly by the [X] secret keys"
    YetIfDivisibleGeneric(i64),    // "Yet if the cycle can instead be divided by the [X] fingers"

    Otherwise,
    PrintCommand(String),
    WhisperRaw,
    Almsivi,

    RebelUserps(String, i64),       // "The Rebel [Name] usurps the King [Value]"
    AnticipationReflects(String, i64), // "The Anticipation of [Name] reflects the change of [Value]"

    // New Math Operations
    WalkingBrassDenies(String, i64),      // Subtraction
    HeartAmplifies(String, i64),          // Multiplication
    ToolsSunder(String, i64),             // Division

    WhileSharmatMaintains(String),   // "While the Sharmat maintains the House of [Name]"
    EndHouse,                       // "The House is closed"
}

pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut text = self.input.trim();

        while !text.is_empty() {
            if text.starts_with("The Wheel turns") {
                tokens.push(Token::WheelTurns);
                text = text["The Wheel turns".len()..].trim();
            } else if text.starts_with("from the first dawn") {
                tokens.push(Token::FromFirstDawn);
                text = text["from the first dawn".len()..].trim();
            } else if text.starts_with("until the ") {
                text = text["until the ".len()..].trim();
                let end_num_idx = text.find("iteration").unwrap();
                let raw_num_str = &text[..end_num_idx].trim();
                let num_str: String = raw_num_str.chars().filter(|c| c.is_ascii_digit()).collect();
                let num = num_str.parse::<i64>().unwrap();
                tokens.push(Token::UntilMiddleDawn(num));
                text = text[end_num_idx + "iteration of the Middle Dawn".len()..].trim();
            } else if text.starts_with("Let the chronicler track the passing of the cycles") {
                tokens.push(Token::LetChronicler);
                text = text["Let the chronicler track the passing of the cycles".len()..].trim();
            } else if text.starts_with("within the house of the Counter") {
                tokens.push(Token::WithinHouseCounter);
                text = text["within the house of the Counter".len()..].trim();
            } else if text.starts_with("Each step along the Walking Way") {
                tokens.push(Token::EachStep);
                text = text["Each step along the Walking Way".len()..].trim();
            } else if text.starts_with("must be witnessed by the sleeping deity") {
                tokens.push(Token::MustBeWitnessed);
                text = text["must be witnessed by the sleeping deity".len()..].trim();
            } else if text.starts_with("If the stars align and the cycle bows to both the") {
                text = text["If the stars align and the cycle bows to both the".len()..].trim();

                // Extract first number
                let next_space = text.find(char::is_whitespace).unwrap_or(text.len());
                let first_num = text[..next_space].parse::<i64>().unwrap();
                text = text[next_space..].trim();

                if text.starts_with("and the") {
                    text = text["and the".len()..].trim();

                    // Extract second number
                    let next_space = text.find(char::is_whitespace).unwrap_or(text.len());
                    let second_num = text[..next_space].parse::<i64>().unwrap();
                    text = text[next_space..].trim();

                    if text.starts_with("simultaneously") {
                        text = text["simultaneously".len()..].trim();
                        tokens.push(Token::IfStarsAlignGeneric(first_num, second_num));
                    }
                }

                if text.starts_with("the houses collapse into a singular state and the terminal must shout") {
                    text = text["the houses collapse into a singular state and the terminal must shout".len()..].trim();
                    let next_space = text.find(|c: char| c.is_whitespace() || c == '\n').unwrap_or(text.len());
                    let word = text[..next_space].trim().to_string();
                    tokens.push(Token::PrintCommand(word.clone()));
                    text = text[word.len()..].trim();
                }
            } else if text.starts_with("But if the number of the current cycle can be divided cleanly by the") {
                text = text["But if the number of the current cycle can be divided cleanly by the".len()..].trim();
                let next_space = text.find(char::is_whitespace).unwrap_or(text.len());
                let num = text[..next_space].parse::<i64>().unwrap();
                tokens.push(Token::ButIfDivisibleGeneric(num));
                text = text[next_space..].trim();

                if text.starts_with("secret keys of the Tribunal leaving no remainder in the math of the Earth-Bones then let the voice of the machine proclaim") {
                    text = text["secret keys of the Tribunal leaving no remainder in the math of the Earth-Bones then let the voice of the machine proclaim".len()..].trim();
                    let next_space = text.find(|c: char| c.is_whitespace() || c == '\n').unwrap_or(text.len());
                    let word = text[..next_space].trim().to_string();
                    tokens.push(Token::PrintCommand(word.clone()));
                    text = text[word.len()..].trim();
                }
            } else if text.starts_with("Yet if the cycle can instead be divided by the") {
                text = text["Yet if the cycle can instead be divided by the".len()..].trim();
                let next_space = text.find(char::is_whitespace).unwrap_or(text.len());
                let num = text[..next_space].parse::<i64>().unwrap();
                tokens.push(Token::YetIfDivisibleGeneric(num));
                text = text[next_space..].trim();

                if text.starts_with("fingers of the Left Hand let it cry") {
                    text = text["fingers of the Left Hand let it cry".len()..].trim();
                    let next_space = text.find(|c: char| c.is_whitespace() || c == '\n').unwrap_or(text.len());
                    let word = text[..next_space].trim().to_string();
                    tokens.push(Token::PrintCommand(word.clone()));
                    text = text[word.len()..].trim();
                }
            } else if text.starts_with("Otherwise") {
                tokens.push(Token::Otherwise);
                text = text["Otherwise".len()..].trim();
                if text.starts_with("let the machine simply whisper the raw naked number of the counter to the void") {
                    tokens.push(Token::WhisperRaw);
                    text = text["let the machine simply whisper the raw naked number of the counter to the void".len()..].trim();
                }
            } else if text.starts_with("The Rebel") {
                text = text["The Rebel".len()..].trim();
                let next_space = text.find(char::is_whitespace).unwrap_or(text.len());
                let var_name = text[..next_space].to_string();
                text = text[var_name.len()..].trim();

                if text.starts_with("usurps the King") {
                    text = text["usurps the King".len()..].trim();
                    let end_num_idx = text.find(|c: char| c.is_whitespace() || c == '\n').unwrap_or(text.len());
                    let num = text[..end_num_idx].parse::<i64>().unwrap();
                    tokens.push(Token::RebelUserps(var_name, num));
                    text = text[end_num_idx..].trim();
                }
            } else if text.starts_with("The Anticipation of") {
                text = text["The Anticipation of".len()..].trim();
                let next_space = text.find(char::is_whitespace).unwrap_or(text.len());
                let var_name = text[..next_space].to_string();
                text = text[var_name.len()..].trim();

                if text.starts_with("reflects the change of") {
                    text = text["reflects the change of".len()..].trim();
                    let end_num_idx = text.find(|c: char| c.is_whitespace() || c == '\n').unwrap_or(text.len());
                    let num = text[..end_num_idx].parse::<i64>().unwrap();
                    tokens.push(Token::AnticipationReflects(var_name, num));
                    text = text[end_num_idx..].trim();
                }
            } else if text.starts_with("The Walking Brass denies") {
                text = text["The Walking Brass denies".len()..].trim();
                let next_space = text.find(char::is_whitespace).unwrap_or(text.len());
                let var_name = text[..next_space].to_string();
                text = text[var_name.len()..].trim();
                if text.starts_with("by the decrement of") {
                    text = text["by the decrement of".len()..].trim();
                    let end_idx = text.find(|c: char| c.is_whitespace() || c == '\n').unwrap_or(text.len());
                    let num = text[..end_idx].parse::<i64>().unwrap();
                    tokens.push(Token::WalkingBrassDenies(var_name, num));
                    text = text[end_idx..].trim();
                }
            } else if text.starts_with("The Heart of Lorkhan amplifies") {
                text = text["The Heart of Lorkhan amplifies".len()..].trim();
                let next_space = text.find(char::is_whitespace).unwrap_or(text.len());
                let var_name = text[..next_space].to_string();
                text = text[var_name.len()..].trim();
                if text.starts_with("by the iteration of") {
                    text = text["by the iteration of".len()..].trim();
                    let end_idx = text.find(|c: char| c.is_whitespace() || c == '\n').unwrap_or(text.len());
                    let num = text[..end_idx].parse::<i64>().unwrap();
                    tokens.push(Token::HeartAmplifies(var_name, num));
                    text = text[end_idx..].trim();
                }
            } else if text.starts_with("The Tools of Kagrenac sunder") {
                text = text["The Tools of Kagrenac sunder".len()..].trim();
                let next_space = text.find(char::is_whitespace).unwrap_or(text.len());
                let var_name = text[..next_space].to_string();
                text = text[var_name.len()..].trim();
                if text.starts_with("across the fractions of") {
                    text = text["across the fractions of".len()..].trim();
                    let end_idx = text.find(|c: char| c.is_whitespace() || c == '\n').unwrap_or(text.len());
                    let num = text[..end_idx].parse::<i64>().unwrap();
                    tokens.push(Token::ToolsSunder(var_name, num));
                    text = text[end_idx..].trim();
                }
            } else if text.starts_with("While the Sharmat maintains the House of") {
                text = text["While the Sharmat maintains the House of".len()..].trim();
                let next_space = text.find(char::is_whitespace).unwrap_or(text.len());
                let var_name = text[..next_space].to_string();
                tokens.push(Token::WhileSharmatMaintains(var_name.clone()));
                text = text[var_name.len()..].trim();
            } else if text.starts_with("The House is closed") {
                tokens.push(Token::EndHouse);
                text = text["The House is closed".len()..].trim();
            } else if text.starts_with("The ending of the words is ALMSIVI") {
                tokens.push(Token::Almsivi);
                break;
            } else {
                // If it hits text it doesn't recognize, capture the typo and trigger an error!
                let unknown_snippet = if text.len() > 30 { &text[..30] } else { text };
                return Err(format!("Unrecognized cosmic script segment: \"{}...\"", unknown_snippet));
            }
        }
        Ok(tokens) // Return successful tokens vector wrapped in Ok
    }
}

