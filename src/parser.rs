use crate::types::*;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Keyword(String), //palabras o comandos como select where from insert etc
    Ident(String), // identificadores o nombres, nombre de un valor una tabla etc
    StrLit(String), // textos literales  escritos por el usuario port ejemplo WHERE name = 'fran', fran es un strlit
    IntLit(i64), // exactamente lo mismo que los strlit pero con nums, ej WHERE age > 18, 18 es intlit
    Star, // simbolos como *
    Comma, // comas ','
    LParen, // parentesis izquierdo '(' 
    RParen, // derecho ')'
    Eq, // de aca al final son todos de comparacion, eq es igual '='
    Neq, // distinto que '!=' o '<>'
    Gt, // mayor que '>'
    Lt, // menor que '<'
    Gte, // mayor o igual '>='
    Lte, // menor o igual '<='
}

fn kw(token: &Token, s: &str) -> bool {
    match token {
        Token::Keyword(k) => k == s, // separa kw del resto
        _ => false,
    }
}

fn read_number(chars: &mut Peekable<Chars<'_>>, first: char) -> Result<Token, String> {
    let mut num = String::new();
    if first == '-' {
        num.push('-');
    }
    while let Some(&d) = chars.peek() {
        if d.is_ascii_digit() {
            num.push(d);
            chars.next();
        } else {
            break;
        }
    }
    let n: i64 = num.parse().map_err(|_| format!("Invalid number: {}", num))?;
    Ok(Token::IntLit(n))
}

fn read_word(chars: &mut Peekable<Chars<'_>>, first: char) -> Token {
    let mut word = String::new();
    word.push(first);
    chars.next();
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            word.push(c);
            chars.next();
        } else {
            break;
        }
    }
    let upper = word.to_uppercase();
    const KEYWORDS: &[&str] = &[
        "CREATE", "TABLE", "INSERT", "INTO", "VALUES",
        "SELECT", "FROM", "WHERE", "DELETE", "TEXT",
        "INT", "INTEGER", "AND", "OR", "SET", "EXIT", "QUIT",
    ];
    if KEYWORDS.contains(&upper.as_str()) {
        Token::Keyword(upper)
    } else {
        Token::Ident(word)
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> { // recibe una referencia del string input, devuelve o un vector con todas las variantes del enum Token que se encuentran en el texto ingresado o un string con el error.
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable(); // peekable permite que un iterador que recorra chars use la fun .peek().

    while let Some(&ch) = chars.peek() {      // comprueba si existe un proximo caracter y lo devuelve si existe, o devuelve none si no existe. luego se comprueba si ese valor existe o es none.       
        if ch.is_whitespace() {
            chars.next();                     // recorre caracteres, si alguno es un espacio en blanco lo ignora y pasa al proximo
            continue;                         // si no es un caracter continua ejecutandose, en este caso en el match para ver a que caracter pertenece
        }

        match ch {
            ',' => { tokens.push(Token::Comma); chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }  // todo este bloque identifica caracteres unicos y operadores, luego los pushea dentor del vector.
            '*' => { tokens.push(Token::Star); chars.next(); }
            '=' => { tokens.push(Token::Eq); chars.next(); }
            '!' => {
                chars.next();
                match chars.next() {
                    Some('=') => tokens.push(Token::Neq),
                    _ => return Err("Expected '=' after '!'".into()), // si hay '!' avanza al proximo y lo chequea, si es un '=' lo pushea como  neq (no igual), si no hay '=' salta error.
                }
            }
            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::Gte);
                    chars.next();
                } else {
                    tokens.push(Token::Gt);
                }
            }               // en estos dos caracteres '<' y '>' verifica si el proximo elemento es un '=', si lo es pushea como menor o igual o mayor o igual segun corresponda, si no hay '=' pushea como mayor o menor que segun corresponda.
            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::Lte);
                    chars.next();
                } else {
                    tokens.push(Token::Lt);
                }
            }
            '\'' => {
                chars.next();
                let mut s = String::new();
                loop {
                    match chars.next() {
                        Some('\'') => {
                            if chars.peek() == Some(&'\'') {
                                chars.next();
                                s.push('\'');
                            } else {
                                break;
                            }
                        }
                        Some(c) => s.push(c),
                        None => return Err("Unterminated string literal".into()),
                    }
                }
                tokens.push(Token::StrLit(s));
            }
            _ if ch.is_ascii_digit() || ch == '-' => {
                tokens.push(read_number(&mut chars, ch)?);
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                tokens.push(read_word(&mut chars, ch));
            }
            _ => return Err(format!("Unexpected character: '{}'", ch)),
        }
    }

    Ok(tokens)
}

pub fn parse(input: &str) -> Result<Command, String> {
    let input = input.trim().strip_suffix(';').unwrap_or(input.trim());
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Err("Empty input".into());
    }

    match &tokens[0] {
        Token::Keyword(k) if k == "CREATE" => parse_create(&tokens),
        Token::Keyword(k) if k == "INSERT" => parse_insert(&tokens),
        Token::Keyword(k) if k == "SELECT" => parse_select(&tokens), // funs de las kw
        Token::Keyword(k) if k == "DELETE" => parse_delete(&tokens),
        Token::Keyword(k) if k == "EXIT" || k == "QUIT" => Ok(Command::Exit),
        t => Err(format!("Unexpected token: {:?}", t)),
    }
}
 // de aca en adelante se declaran esas funs
fn parse_create(tokens: &[Token]) -> Result<Command, String> {
    if tokens.len() < 6 {
        return Err("Incomplete CREATE TABLE statement".into());
    }
    if !kw(&tokens[1], "TABLE") {
        return Err("Expected TABLE after CREATE".into());
    }
    let name = match &tokens[2] {
        Token::Ident(n) => n.clone(),
        _ => return Err("Expected table name after CREATE TABLE".into()),
    };
    if tokens[3] != Token::LParen {
        return Err("Expected ( after table name".into());
    }
    if *tokens.last().unwrap() != Token::RParen {
        return Err("Expected ) at end of column definitions".into());
    }

    let col_tokens = &tokens[4..tokens.len() - 1];
    let mut columns = Vec::new();
    let mut i = 0;

    while i < col_tokens.len() {
        if i + 1 >= col_tokens.len() {
            return Err("Incomplete column definition".into());
        }
        let col_name = match &col_tokens[i] {
            Token::Ident(n) => n.clone(),
            t => return Err(format!("Expected column name, got {:?}", t)),
        };
        let col_type= match &col_tokens[i + 1] {
            Token::Keyword(t) => t.parse::<Type>()?,
            t => return Err(format!("Expected type, got {:?}", t)),
        };
        columns.push(Column { name: col_name, col_type });
        i += 2;
        if i < col_tokens.len() && col_tokens[i] == Token::Comma {
            i += 1;
        }
    }

    Ok(Command::CreateTable { name, columns })
}

fn parse_insert(tokens: &[Token]) -> Result<Command, String> {
    if tokens.len() < 7 {
        return Err("Incomplete INSERT statement".into());
    }
    if !kw(&tokens[1], "INTO") {
        return Err("Expected INTO after INSERT".into());
    }
    let name = match &tokens[2] {
        Token::Ident(n) => n.clone(),
        _ => return Err("Expected table name after INSERT INTO".into()),
    };
    if !kw(&tokens[3], "VALUES") {
        return Err("Expected VALUES".into());
    }
    if tokens[4] != Token::LParen {
        return Err("Expected ( after VALUES".into());
    }

    let close = tokens[5..]
        .iter()
        .position(|t| *t == Token::RParen)
        .map(|p| p + 5)
        .ok_or_else(|| "Expected )".to_string())?;

    let val_tokens = &tokens[5..close];
    let mut values = Vec::new();
    let mut i = 0;

    while i < val_tokens.len() {
        let val = match &val_tokens[i] {
            Token::StrLit(s) => Value::Text(s.clone()),
            Token::IntLit(n) => Value::Int(*n),
            t => return Err(format!("Expected value, got {:?}", t)),
        };
        values.push(val);
        i += 1;
        if i < val_tokens.len() && val_tokens[i] == Token::Comma {
            i += 1;
        }
    }

    Ok(Command::Insert { table: name, values })
}

fn parse_select(tokens: &[Token]) -> Result<Command, String> {
    let mut pos = 1;

    let columns = if pos < tokens.len() && tokens[pos] == Token::Star {
        pos += 1;
        None
    } else {
        let mut cols = Vec::new();
        while pos < tokens.len() {
            match &tokens[pos] {
                Token::Ident(c) => cols.push(c.clone()),
                t => return Err(format!("Expected column name, got {:?}", t)),
            }
            pos += 1;
            if pos < tokens.len() && tokens[pos] == Token::Comma {
                pos += 1;
            } else {
                break;
            }
        }
        if cols.is_empty() {
            return Err("Expected column name(s) after SELECT".into());
        }
        Some(cols)
    };

    if pos >= tokens.len() || !kw(&tokens[pos], "FROM") {
        return Err("Expected FROM".into());
    }
    pos += 1;
    if pos >= tokens.len() {
        return Err("Expected table name after FROM".into());
    }
    let table = match &tokens[pos] {
        Token::Ident(t) => t.clone(),
        _ => return Err("Expected table name".into()),
    };
    pos += 1;

    let where_clause = if pos < tokens.len() {
        if !kw(&tokens[pos], "WHERE") {
            return Err(format!("Unexpected token after table name: {:?}", tokens[pos]));
        }
        pos += 1;
        if pos + 2 >= tokens.len() {
            return Err("Incomplete WHERE clause".into());
        }
        let col = match &tokens[pos] {
            Token::Ident(c) => c.clone(),
            t => return Err(format!("Expected column name in WHERE, got {:?}", t)),
        };
        pos += 1;
        let op = match &tokens[pos] {
            Token::Eq => Operator::Eq,
            Token::Neq => Operator::Neq,
            Token::Gt => Operator::Gt,
            Token::Lt => Operator::Lt,
            Token::Gte => Operator::Gte,
            Token::Lte => Operator::Lte,
            t => return Err(format!("Expected operator in WHERE, got {:?}", t)),
        };
        pos += 1;
        let val = match &tokens[pos] {
            Token::StrLit(s) => Value::Text(s.clone()),
            Token::IntLit(n) => Value::Int(*n),
            t => return Err(format!("Expected value in WHERE, got {:?}", t)),
        };
        Some((col, op, val))
    } else {
        None
    };

    Ok(Command::Select { table, columns, where_clause })
}

fn parse_delete(tokens: &[Token]) -> Result<Command, String> {
    if tokens.len() < 3 {
        return Err("Incomplete DELETE statement".into());
    }
    if !kw(&tokens[1], "FROM") {
        return Err("Expected FROM after DELETE".into());
    }
    let table = match &tokens[2] {
        Token::Ident(t) => t.clone(),
        _ => return Err("Expected table name after DELETE FROM".into()),
    };

    let where_clause = if tokens.len() > 3 {
        if !kw(&tokens[3], "WHERE") {
            return Err(format!("Unexpected token: {:?}", tokens[3]));
        }
        if tokens.len() < 7 {
            return Err("Incomplete WHERE clause".into());
        }
        let col = match &tokens[4] {
            Token::Ident(c) => c.clone(),
            t => return Err(format!("Expected column name in WHERE, got {:?}", t)),
        };
        let op = match &tokens[5] {
            Token::Eq => Operator::Eq,
            Token::Neq => Operator::Neq,
            Token::Gt => Operator::Gt,
            Token::Lt => Operator::Lt,
            Token::Gte => Operator::Gte,
            Token::Lte => Operator::Lte,
            t => return Err(format!("Expected operator in WHERE, got {:?}", t)),
        };
        let val = match &tokens[6] {
            Token::StrLit(s) => Value::Text(s.clone()),
            Token::IntLit(n) => Value::Int(*n),
            t => return Err(format!("Expected value in WHERE, got {:?}", t)),
        };
        Some((col, op, val))
    } else {
        None
    };

    Ok(Command::Delete { table, where_clause })
}

