// https://github.com/bbkane/dotfiles/blob/master/bin_common/bin_common/format_shell_cmd.py
use std::fmt;
use std::io::{self, BufRead, Write};
use std::process;

enum Kind {
    Pipe,
    Opt,
    SingleQuoteString,
    DoubleQuoteString,
    Cmd,
}

struct Token {
    text: String,
    kind: Kind,
}

impl Token {
    fn new(text: String, kind: Kind) -> Self {
        Self { text, kind }
    }
}

struct Expr {
    content: Vec<char>,
    len_expr: usize,
}

struct Iter<'a> {
    start: usize,
    end: usize,
    expr: &'a Expr,
}

impl Expr {
    fn new(content: &str) -> Self {
        let content = content.chars().collect::<Vec<_>>();
        let len_expr = content.len();
        Self { content, len_expr }
    }
    fn iter(&self) -> Iter {
        Iter {
            start: 0,
            end: 0,
            expr: self,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        while self.end < self.expr.len_expr && self.expr.content[self.end].is_whitespace() {
            self.start += 1;
            self.end += 1;
        }
        if self.end == self.expr.len_expr {
            return None;
        }

        match self.expr.content[self.end] {
            '|' => {
                self.end += 1;
                let text = self.expr.content[self.start..self.end].iter().collect();
                self.start = self.end;
                Some(Token::new(text, Kind::Pipe))
            }
            '-' => {
                while self.end < self.expr.len_expr && !self.expr.content[self.end].is_whitespace()
                {
                    self.end += 1
                }
                let text = self.expr.content[self.start..self.end].iter().collect();
                self.start = self.end;
                Some(Token::new(text, Kind::Opt))
            }
            '"' => {
                loop {
                    self.end += 1;
                    if self.end == self.expr.len_expr {
                        panic!("Double quote at column {} unmatched", self.start)
                    }
                    if self.expr.content[self.end] == '"' {
                        break;
                    }
                }
                self.end += 1;
                let text = self.expr.content[self.start..self.end].iter().collect();
                self.start = self.end;
                Some(Token::new(text, Kind::DoubleQuoteString))
            }
            '\'' => {
                loop {
                    self.end += 1;
                    if self.end == self.expr.len_expr {
                        panic!("Single quote at column {} unmatched", self.start);
                    }
                    if self.expr.content[self.end] == '\'' {
                        break;
                    }
                }
                self.end += 1;
                let text = self.expr.content[self.start..self.end].iter().collect();
                self.start = self.end;
                Some(Token::new(text, Kind::SingleQuoteString))
            }
            _ => {
                // # not space, not anything else, must be cmd
                while self.end < self.expr.len_expr && !self.expr.content[self.end].is_whitespace()
                {
                    self.end += 1;
                }
                let text = self.expr.content[self.start..self.end].iter().collect();
                self.start = self.end;
                Some(Token::new(text, Kind::Cmd))
            }
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in self.iter() {
            match token.kind {
                Kind::Pipe => write!(f, "|\n    ")?,
                Kind::Opt => write!(f, "\\\n        {} ", token.text)?,
                Kind::Cmd | Kind::DoubleQuoteString | Kind::SingleQuoteString => {
                    write!(f, "{} ", token.text)?
                }
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    if let Some(content) = stdin.lock().lines().next() {
        let content = content?;
        let expr = Expr::new(&content);
        write!(io::stdout(), "{}", expr)?;
        return Ok(());
    }
    write!(io::stderr(), "No input")?;
    process::exit(1)
}

#[cfg(test)]
mod tests {
    /* """Return a list of tokens
    >>> list(tokenize(''))
    []
    >>> list(tokenize(' '))
    []
    >>> list(tokenize('|'))
    [Token(text='|', kind='PIPE')]
    >>> list(tokenize(' | '))
    [Token(text='|', kind='PIPE')]
    >>> list(tokenize(' | -'))
    [Token(text='|', kind='PIPE'), Token(text='-', kind='OPTION')]
    >>> list(tokenize(' | -bob'))
    [Token(text='|', kind='PIPE'), Token(text='-bob', kind='OPTION')]
    >>> list(tokenize(' | -bob "dillon"'))
    [Token(text='|', kind='PIPE'), Token(text='-bob', kind='OPTION'), Token(text='"dillon"', kind='DOUBLE_QUOTE_STRING')]
    >>> list(tokenize('echo | openssl s_client -connect www.example.com:443'))
    [Token(text='echo', kind='CMD'), Token(text='|', kind='PIPE'), Token(text='openssl', kind='CMD'), Token(text='s_client', kind='CMD'), Token(text='-connect', kind='OPTION'), Token(text='www.example.com:443', kind='CMD')]
    >>> list(tokenize('"bob'))
    Traceback (most recent call last):
    ...
    ValueError: Double quote at column 0 unmatched
    >>> list(tokenize('"'))
    Traceback (most recent call last):
    ...
    ValueError: Double quote at column 0 unmatched
    >>> list(tokenize('" "'))
    [Token(text='" "', kind='DOUBLE_QUOTE_STRING')]
    >>> list(tokenize('echo "hi there" | awk "{print $1}"'))
    [Token(text='echo', kind='CMD'), Token(text='"hi there"', kind='DOUBLE_QUOTE_STRING'), Token(text='|', kind='PIPE'), Token(text='awk', kind='CMD'), Token(text='"{print $1}"', kind='DOUBLE_QUOTE_STRING')]
    """
    */
}
