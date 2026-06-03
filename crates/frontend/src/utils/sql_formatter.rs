#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    StringLiteral(String),
    QuotedIdent(String),
    LineComment(String),
    BlockComment(String),
    LParen,
    RParen,
    Comma,
    Semicolon,
    Operator(String),
}

/// Tokenizes a SQL string into a sequence of [`Token`] values.
///
/// Handles string literals (`'...'`), quoted identifiers (`"..."`),
/// line comments (`--`), block comments (`/* ... */`), parentheses,
/// commas, semicolons, operators, and word tokens.
///
/// # Arguments
///
/// * `sql` - The raw SQL string to tokenize.
///
/// # Returns
///
/// A `Vec<Token>` representing the lexical tokens of the input SQL.
///
/// # Example
///
/// ```
/// use sql_admin_frontend::utils::sql_formatter::tokenize_sql;
/// let tokens = tokenize_sql("SELECT * FROM users");
/// assert!(matches!(tokens[0], Token::Word(w) if w == "SELECT"));
/// ```
pub fn tokenize_sql(sql: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = sql.char_indices().peekable();

    while let Some((idx, ch)) = chars.next() {
        match ch {
            '\'' => {
                let mut s = String::from("'");
                while let Some((_, c)) = chars.next() {
                    s.push(c);
                    if c == '\'' {
                        if chars.peek().map(|(_, c)| *c) == Some('\'') {
                            chars.next();
                            s.push('\'');
                        } else {
                            break;
                        }
                    }
                }
                tokens.push(Token::StringLiteral(s));
            }
            '"' => {
                let mut s = String::from("\"");
                for (_, c) in chars.by_ref() {
                    s.push(c);
                    if c == '"' {
                        break;
                    }
                }
                tokens.push(Token::QuotedIdent(s));
            }
            '-' if sql.as_bytes().get(idx + 1) == Some(&b'-') => {
                let mut s = String::from("--");
                chars.next();
                for (_, c) in chars.by_ref() {
                    s.push(c);
                    if c == '\n' {
                        break;
                    }
                }
                tokens.push(Token::LineComment(s.trim_end().to_string()));
            }
            '/' if sql.as_bytes().get(idx + 1) == Some(&b'*') => {
                let mut s = String::from("/*");
                chars.next();
                let mut prev_star = false;
                for (_, c) in chars.by_ref() {
                    s.push(c);
                    if prev_star && c == '/' {
                        break;
                    }
                    prev_star = c == '*';
                }
                tokens.push(Token::BlockComment(s));
            }
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ',' => tokens.push(Token::Comma),
            ';' => tokens.push(Token::Semicolon),
            ' ' | '\t' | '\n' | '\r' => {}
            '=' | '!' | '<' | '>' | '|' | '+' | '-' | '/' | '%' | '*' => {
                let mut op = String::new();
                op.push(ch);
                if ch == '|' && chars.peek().map(|(_, c)| *c) == Some('|') {
                    chars.next();
                    op.push('|');
                } else if ch == '-' {
                    // already handled -- above
                } else if (ch == '<' || ch == '>' || ch == '!') && chars.peek().map(|(_, c)| *c) == Some('=') {
                    chars.next();
                    op.push('=');
                } else if ch == '<' && chars.peek().map(|(_, c)| *c) == Some('>') {
                    chars.next();
                    op.push('>');
                }
                tokens.push(Token::Operator(op));
            }
            _ if ch.is_alphanumeric() || ch == '_' || ch == '$' || ch == '.' => {
                let mut word = String::new();
                word.push(ch);
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' || c == '$' || c == '.' {
                        word.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Word(word));
            }
            _ => {
                tokens.push(Token::Operator(ch.to_string()));
            }
        }
    }

    tokens
}

static UPPERCASE_KEYWORDS: &[&str] = &[
    "SELECT",
    "FROM",
    "WHERE",
    "JOIN",
    "INNER",
    "LEFT",
    "RIGHT",
    "OUTER",
    "CROSS",
    "NATURAL",
    "FULL",
    "ON",
    "AND",
    "OR",
    "NOT",
    "IN",
    "LIKE",
    "BETWEEN",
    "EXISTS",
    "IS",
    "NULL",
    "AS",
    "ASC",
    "DESC",
    "ORDER",
    "BY",
    "GROUP",
    "HAVING",
    "LIMIT",
    "OFFSET",
    "UNION",
    "ALL",
    "INTERSECT",
    "EXCEPT",
    "INSERT",
    "INTO",
    "VALUES",
    "UPDATE",
    "SET",
    "DELETE",
    "CREATE",
    "TABLE",
    "DROP",
    "ALTER",
    "ADD",
    "COLUMN",
    "INDEX",
    "VIEW",
    "IF",
    "REPLACE",
    "DEFAULT",
    "PRIMARY",
    "KEY",
    "FOREIGN",
    "REFERENCES",
    "UNIQUE",
    "CHECK",
    "CONSTRAINT",
    "AUTOINCREMENT",
    "BEGIN",
    "COMMIT",
    "ROLLBACK",
    "TRANSACTION",
    "CASE",
    "WHEN",
    "THEN",
    "ELSE",
    "END",
    "RETURNING",
    "WITH",
    "RECURSIVE",
    "DISTINCT",
    "OVER",
    "PARTITION",
    "WINDOW",
    "ROWS",
    "RANGE",
    "UNBOUNDED",
    "PRECEDING",
    "FOLLOWING",
    "CURRENT",
    "ROW",
    "LATERAL",
    "MATERIALIZED",
    "TEMP",
    "TEMPORARY",
    "CASCADE",
    "RESTRICT",
    "NO",
    "ACTION",
    "VACUUM",
    "ANALYZE",
    "REINDEX",
    "PRAGMA",
    "EXPLAIN",
    "QUERY",
    "PLAN",
    "TRUE",
    "FALSE",
    "UNKNOWN",
    "INTEGER",
    "TEXT",
    "BLOB",
    "REAL",
    "NUMERIC",
    "BOOLEAN",
    "DATE",
    "TIMESTAMP",
    "VARCHAR",
    "CHAR",
    "BIGINT",
    "SMALLINT",
    "FLOAT",
    "DOUBLE",
    "DECIMAL",
    "SERIAL",
    "BIGSERIAL",
    "UUID",
    "COUNT",
    "SUM",
    "AVG",
    "MIN",
    "MAX",
    "COALESCE",
    "CAST",
    "CONVERT",
    "EXTRACT",
    "SUBSTRING",
    "POSITION",
    "TRIM",
    "UPPER",
    "LOWER",
    "LENGTH",
    "CONCAT",
    "ABS",
    "ROUND",
    "CEIL",
    "FLOOR",
    "POWER",
    "SQRT",
    "MOD",
    "NOW",
    "NULLS",
    "FIRST",
    "LAST",
    "USING",
    "SCHEMA",
    "TO",
    "GRANT",
    "REVOKE",
    "TRUNCATE",
    "RENAME",
    "ONLY",
    "DEFERRABLE",
    "IMMEDIATE",
    "CONCURRENTLY",
];

/// Converts a word to its uppercase form if it is a recognized SQL keyword.
///
/// If the word (case-insensitive) matches an entry in [`UPPERCASE_KEYWORDS`],
/// returns the uppercase version; otherwise returns the original word unchanged.
///
/// # Arguments
///
/// * `word` - Any type that implements `AsRef<str>`, such as `&str` or `String`.
///
/// # Example
///
/// ```
/// use sql_admin_frontend::utils::sql_formatter::uppercase_keyword;
/// assert_eq!(uppercase_keyword("select"), "SELECT");
/// assert_eq!(uppercase_keyword("my_table"), "my_table");
/// ```
pub fn uppercase_keyword<S: AsRef<str>>(word: S) -> String {
    let upper = word.as_ref().to_uppercase();
    if UPPERCASE_KEYWORDS.contains(&upper.as_str()) {
        upper
    } else {
        word.as_ref().to_string()
    }
}

/// Checks whether a word matches a specific SQL keyword (case-insensitive).
///
/// # Arguments
///
/// * `word` - The word to check.
/// * `kw`  - The keyword to compare against (must be uppercase).
///
/// # Example
///
/// ```
/// use sql_admin_frontend::utils::sql_formatter::is_keyword_eq;
/// assert!(is_keyword_eq("select", "SELECT"));
/// assert!(!is_keyword_eq("my_table", "SELECT"));
/// ```
pub fn is_keyword_eq<S: AsRef<str>>(word: S, kw: &str) -> bool {
    word.as_ref().to_uppercase() == kw
}

/// Checks whether a word matches any of the given SQL keywords (case-insensitive).
///
/// # Arguments
///
/// * `word`     - The word to check.
/// * `keywords` - A slice of keyword strings to compare against (must be uppercase).
///
/// # Example
///
/// ```
/// use sql_admin_frontend::utils::sql_formatter::is_keyword_in;
/// assert!(is_keyword_in("left", &["LEFT", "RIGHT"]));
/// assert!(!is_keyword_in("my_table", &["LEFT", "RIGHT"]));
/// ```
pub fn is_keyword_in<S: AsRef<str>>(word: S, keywords: &[&str]) -> bool {
    let upper = word.as_ref().to_uppercase();
    keywords.iter().any(|k| upper == *k)
}

/// A streaming SQL formatter that transforms raw SQL into a standardized,
/// human-readable layout following the project's SQL formatting specification.
///
/// The formatter applies the following rules:
///
/// - SQL keywords are uppercased (e.g. `SELECT`, `FROM`, `WHERE`).
/// - Major clauses (`SELECT`, `FROM`, `WHERE`, `JOIN`, `GROUP BY`, `ORDER BY`,
///   `LIMIT`, etc.) each start on a new line.
/// - `SELECT` fields are placed one per line with 2-space indentation.
/// - `AND` / `OR` conditions are indented under `WHERE` with 2-space indentation.
/// - `ON` conditions are indented under their `JOIN` clause.
/// - `CASE` / `WHEN` / `ELSE` / `END` blocks are properly nested.
/// - CTEs (`WITH` clauses) are formatted with `AS (` on the same line as the
///   CTE name, and the body is indented.
/// - Statements are terminated with a semicolon.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the borrowed token slice.
///
/// # Example
///
/// ```
/// use sql_admin_frontend::utils::sql_formatter::{format_sql, SqlFormatter, tokenize_sql};
/// let formatted = format_sql("select id,name from users where id=1");
/// assert!(formatted.contains("SELECT\n  id"));
/// assert!(formatted.contains("FROM"));
/// assert!(formatted.contains("WHERE"));
/// ```
pub struct SqlFormatter<'a> {
    tokens: &'a [Token],
    pos: usize,
    output: String,
    indent: usize,
}

impl<'a> SqlFormatter<'a> {
    /// Creates a new `SqlFormatter` that will format the given token slice.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A slice of [`Token`] values produced by [`tokenize_sql`].
    pub fn new(tokens: &'a [Token]) -> Self {
        SqlFormatter {
            tokens,
            pos: 0,
            output: String::new(),
            indent: 0,
        }
    }

    /// Consumes the formatter and returns the formatted SQL string.
    ///
    /// Processes all tokens sequentially, applying formatting rules for each
    /// SQL statement type. The result is trimmed and guaranteed to end with
    /// a semicolon (unless the input is empty).
    pub fn format(mut self) -> String {
        while self.pos < self.tokens.len() {
            self.format_statement();
        }
        let result = self.output.trim().to_string();
        if result.is_empty() || result.ends_with(';') {
            result
        } else {
            format!("{};", result)
        }
    }

    fn peek_word(&self) -> Option<&str> {
        if self.pos < self.tokens.len()
            && let Token::Word(w) = &self.tokens[self.pos] {
                return Some(w);
            }
        None
    }

    fn peek_token(&self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            Some(self.tokens[self.pos].clone())
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn write_indent(&mut self) {
        self.output.push_str(&"  ".repeat(self.indent));
    }

    fn write_raw(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn trim_trailing_space(&mut self) {
        let trimmed = self.output.trim_end_matches(' ');
        self.output.truncate(trimmed.len());
    }

    fn format_statement(&mut self) {
        let word = match self.peek_word() {
            Some(w) => w.to_uppercase(),
            None => {
                if let Some(tok) = self.advance() {
                    self.format_token_inline(tok);
                }
                return;
            }
        };

        match word.as_str() {
            "SELECT" => self.format_select(),
            "WITH" => self.format_with(),
            "INSERT" => self.format_insert(),
            "UPDATE" => self.format_update(),
            "DELETE" => self.format_delete(),
            "CREATE" | "DROP" | "ALTER" | "VACUUM" | "ANALYZE" | "REINDEX" | "PRAGMA"
            | "EXPLAIN" => self.format_ddl(),
            "BEGIN" | "COMMIT" | "ROLLBACK" => self.format_transaction(),
            _ => {
                if let Some(tok) = self.advance() {
                    self.format_token_inline(tok);
                }
            }
        }
    }

    fn format_token_inline(&mut self, token: Token) {
        match token {
            Token::Word(w) => self.write_raw(&uppercase_keyword(w)),
            Token::StringLiteral(s) => self.write_raw(&s),
            Token::QuotedIdent(s) => self.write_raw(&s),
            Token::LineComment(s) => {
                self.write_raw(&s);
                self.write_raw("\n");
            }
            Token::BlockComment(s) => {
                self.write_indent();
                self.write_raw(&s);
                self.write_raw("\n");
            }
            Token::LParen => self.write_raw("("),
            Token::RParen => self.write_raw(")"),
            Token::Comma => self.write_raw(", "),
            Token::Semicolon => {
                self.write_raw(";\n");
            }
            Token::Operator(op) => {
                self.write_raw(" ");
                self.write_raw(&op);
                self.write_raw(" ");
            }
        }
    }

    fn format_select(&mut self) {
        self.advance();
        self.write_indent();
        self.write_raw("SELECT");

        if let Some(Token::Word(w)) = self.peek_token()
            && is_keyword_eq(w, "DISTINCT") {
                self.advance();
                self.write_raw(" DISTINCT");
            }

        self.write_raw("\n");
        self.indent += 1;
        self.format_select_list();
        self.indent -= 1;

        self.format_from_and_rest();
    }

    fn format_select_list(&mut self) {
        let mut first = true;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }

            match self.peek_token() {
                Some(Token::Word(ref w))
                    if is_keyword_in(
                        w,
                        &[
                            "FROM",
                            "WHERE",
                            "GROUP",
                            "HAVING",
                            "ORDER",
                            "LIMIT",
                            "OFFSET",
                            "UNION",
                            "INTERSECT",
                            "EXCEPT",
                            "INTO",
                            "SET",
                            "VALUES",
                            "RETURNING",
                            "JOIN",
                            "INNER",
                            "LEFT",
                            "RIGHT",
                            "CROSS",
                            "FULL",
                            "NATURAL",
                            "ON",
                            "AND",
                            "OR",
                        ],
                    ) =>
                {
                    break;
                }
                Some(Token::Semicolon) | Some(Token::RParen) => break,
                _ => {}
            }

            if !first {
                if let Some(Token::Comma) = self.peek_token() {
                    self.advance();
                    self.write_raw(",");
                    self.write_raw("\n");
                    self.write_indent();
                } else {
                    break;
                }
            } else {
                self.write_indent();
                first = false;
            }

            self.format_select_item();
        }

        if !first {
            self.write_raw("\n");
        }
    }

    fn format_select_item(&mut self) {
        let mut depth = 0usize;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }

            match self.peek_token() {
                Some(Token::Comma) if depth == 0 => break,
                Some(Token::Word(ref w))
                    if depth == 0
                        && is_keyword_in(
                            w,
                            &[
                                "FROM",
                                "WHERE",
                                "GROUP",
                                "HAVING",
                                "ORDER",
                                "LIMIT",
                                "OFFSET",
                                "UNION",
                                "INTERSECT",
                                "EXCEPT",
                                "RETURNING",
                                "JOIN",
                                "INNER",
                                "LEFT",
                                "RIGHT",
                                "CROSS",
                                "FULL",
                                "NATURAL",
                                "ON",
                            ],
                        ) =>
                {
                    break;
                }
                Some(Token::Semicolon) | Some(Token::RParen) if depth == 0 => break,
                Some(Token::LParen) => {
                    self.advance();
                    self.write_raw("(");
                    depth += 1;
                }
                Some(Token::RParen) => {
                    self.advance();
                    self.write_raw(")");
                    depth = depth.saturating_sub(1);
                }
                Some(Token::Word(ref w)) if is_keyword_eq(w, "CASE") && depth == 0 => {
                    self.format_case();
                }
                Some(Token::Word(ref w)) if is_keyword_eq(w, "AS") && depth == 0 => {
                    self.advance();
                    self.write_raw(" AS ");
                    if let Some(Token::Word(alias)) = self.advance() {
                        self.write_raw(&uppercase_keyword(alias));
                    }
                }
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        match &tok {
                            Token::Word(w) => {
                                self.write_raw(&uppercase_keyword(w));
                            }
                            Token::StringLiteral(s) => {
                                self.write_raw(s);
                            }
                            Token::QuotedIdent(s) => {
                                self.write_raw(s);
                            }
                            Token::Operator(op) => {
                                self.write_raw(op);
                            }
                            _ => self.format_token_inline(tok),
                        }
                        if !matches!(
                            self.peek_token(),
                            Some(Token::LParen)
                                | Some(Token::RParen)
                                | Some(Token::Comma)
                                | Some(Token::Semicolon)
                                | None
                        ) {
                            self.write_raw(" ");
                        }
                    }
                }
                None => break,
            }
        }
        self.trim_trailing_space();
    }

    fn format_case(&mut self) {
        self.advance();
        self.write_raw("CASE\n");
        self.indent += 1;

        loop {
            if self.pos >= self.tokens.len() {
                break;
            }

            if let Some(Token::Word(w)) = self.peek_token() {
                match w.to_uppercase().as_str() {
                    "WHEN" => {
                        self.advance();
                        self.write_indent();
                        self.write_raw("WHEN ");
                        self.format_case_condition();
                    }
                    "ELSE" => {
                        self.advance();
                        self.write_indent();
                        self.write_raw("ELSE ");
                        self.format_case_expression();
                    }
                    "END" => {
                        self.advance();
                        self.indent -= 1;
                        self.write_indent();
                        self.write_raw("END");
                        break;
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
    }

    fn format_case_condition(&mut self) {
        let mut depth = 0usize;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            match self.peek_token() {
                Some(Token::Word(ref w)) if depth == 0 && is_keyword_eq(w, "THEN") => {
                    self.advance();
                    self.trim_trailing_space();
                    self.write_raw(" THEN ");
                    self.format_case_expression();
                    return;
                }
                Some(Token::LParen) => {
                    self.advance();
                    self.write_raw("(");
                    depth += 1;
                }
                Some(Token::RParen) => {
                    self.advance();
                    self.write_raw(")");
                    depth = depth.saturating_sub(1);
                }
                Some(Token::Word(ref w))
                    if is_keyword_in(w, &["WHEN", "ELSE", "END"]) && depth == 0 =>
                {
                    break;
                }
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        match &tok {
                            Token::Word(w) => {
                                self.write_raw(&uppercase_keyword(w));
                            }
                            Token::StringLiteral(s) => {
                                self.write_raw(s);
                            }
                            Token::Operator(op) => {
                                self.write_raw(op);
                            }
                            _ => self.format_token_inline(tok),
                        }
                        if !matches!(
                            self.peek_token(),
                            Some(Token::LParen)
                                | Some(Token::RParen)
                                | Some(Token::Comma)
                                | Some(Token::Semicolon)
                                | None
                        ) {
                            self.write_raw(" ");
                        }
                    }
                }
                None => break,
            }
        }
        self.trim_trailing_space();
    }

    fn format_case_expression(&mut self) {
        let mut depth = 0usize;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            match self.peek_token() {
                Some(Token::Word(ref w))
                    if depth == 0 && is_keyword_in(w, &["WHEN", "ELSE", "END"]) =>
                {
                    self.trim_trailing_space();
                    self.write_raw("\n");
                    break;
                }
                Some(Token::LParen) => {
                    self.advance();
                    self.write_raw("(");
                    depth += 1;
                }
                Some(Token::RParen) => {
                    self.advance();
                    self.write_raw(")");
                    depth = depth.saturating_sub(1);
                }
                Some(Token::Comma) if depth == 0 => {
                    self.write_raw("\n");
                    break;
                }
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        match &tok {
                            Token::Word(w) => {
                                self.write_raw(&uppercase_keyword(w));
                            }
                            Token::StringLiteral(s) => {
                                self.write_raw(s);
                            }
                            Token::Operator(op) => {
                                self.write_raw(op);
                            }
                            _ => self.format_token_inline(tok),
                        }
                        if !matches!(
                            self.peek_token(),
                            Some(Token::LParen)
                                | Some(Token::RParen)
                                | Some(Token::Comma)
                                | Some(Token::Semicolon)
                                | None
                        ) {
                            self.write_raw(" ");
                        }
                    }
                }
                None => break,
            }
        }
        self.trim_trailing_space();
    }

    fn format_from_and_rest(&mut self) {
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }

            let word = match self.peek_word() {
                Some(w) => w.to_uppercase(),
                None => match self.peek_token() {
                    Some(Token::Semicolon) => {
                        self.advance();
                        self.write_raw(";\n");
                        break;
                    }
                    Some(Token::RParen) => break,
                    Some(_) => {
                        if let Some(tok) = self.advance() {
                            self.format_token_inline(tok);
                        }
                        continue;
                    }
                    None => break,
                },
            };

            match word.as_str() {
                "FROM" => {
                    self.advance();
                    self.write_indent();
                    self.write_raw("FROM ");
                    self.format_table_ref_with_alias();
                }
                "JOIN" | "INNER" | "LEFT" | "RIGHT" | "CROSS" | "FULL" | "NATURAL" => {
                    self.format_join();
                }
                "ON" => {
                    self.advance();
                    self.indent += 1;
                    self.write_indent();
                    self.write_raw("ON ");
                    self.format_on_condition();
                    self.indent -= 1;
                }
                "WHERE" => {
                    self.advance();
                    self.write_indent();
                    self.write_raw("WHERE ");
                    self.format_where_condition();
                }
                "AND" => {
                    self.advance();
                    self.indent += 1;
                    self.write_indent();
                    self.write_raw("AND ");
                    self.format_condition_expr();
                    self.indent -= 1;
                }
                "OR" => {
                    self.advance();
                    self.indent += 1;
                    self.write_indent();
                    self.write_raw("OR ");
                    self.format_condition_expr();
                    self.indent -= 1;
                }
                "GROUP" => {
                    self.advance();
                    if let Some(Token::Word(ref w)) = self.peek_token()
                        && is_keyword_eq(w, "BY") {
                            self.advance();
                        }
                    self.write_indent();
                    self.write_raw("GROUP BY ");
                    self.format_comma_list();
                }
                "HAVING" => {
                    self.advance();
                    self.write_indent();
                    self.write_raw("HAVING ");
                    self.format_condition_expr();
                }
                "ORDER" => {
                    self.advance();
                    if let Some(Token::Word(ref w)) = self.peek_token()
                        && is_keyword_eq(w, "BY") {
                            self.advance();
                        }
                    self.write_indent();
                    self.write_raw("ORDER BY ");
                    self.format_comma_list();
                }
                "LIMIT" => {
                    self.advance();
                    self.write_indent();
                    self.write_raw("LIMIT ");
                    self.format_single_expr();
                }
                "OFFSET" => {
                    self.advance();
                    self.write_indent();
                    self.write_raw("OFFSET ");
                    self.format_single_expr();
                }
                "UNION" => {
                    self.advance();
                    self.write_raw("\n");
                    self.write_indent();
                    self.write_raw("UNION");
                    if let Some(Token::Word(w)) = self.peek_token()
                        && is_keyword_eq(&w, "ALL") {
                            self.advance();
                            self.write_raw(" ALL");
                        }
                    self.write_raw("\n");
                }
                "INTERSECT" => {
                    self.advance();
                    self.write_raw("\n");
                    self.write_indent();
                    self.write_raw("INTERSECT\n");
                }
                "EXCEPT" => {
                    self.advance();
                    self.write_raw("\n");
                    self.write_indent();
                    self.write_raw("EXCEPT\n");
                }
                "RETURNING" => {
                    self.advance();
                    self.write_indent();
                    self.write_raw("RETURNING ");
                    self.format_comma_list();
                }
                "SELECT" => {
                    self.format_select();
                }
                "WITH" => {
                    self.format_with();
                }
                _ => {
                    if let Some(tok) = self.advance() {
                        self.format_token_inline(tok);
                    }
                }
            }
        }
    }

    fn format_table_ref_with_alias(&mut self) {
        let mut got_alias = false;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            match self.peek_token() {
                Some(Token::Word(ref w))
                    if is_keyword_in(
                        w,
                        &[
                            "WHERE",
                            "JOIN",
                            "INNER",
                            "LEFT",
                            "RIGHT",
                            "CROSS",
                            "FULL",
                            "NATURAL",
                            "ON",
                            "GROUP",
                            "HAVING",
                            "ORDER",
                            "LIMIT",
                            "OFFSET",
                            "UNION",
                            "INTERSECT",
                            "EXCEPT",
                            "RETURNING",
                            "SET",
                            "VALUES",
                            "AND",
                            "OR",
                            "SELECT",
                            "WITH",
                            "INSERT",
                            "UPDATE",
                            "DELETE",
                            "CREATE",
                            "DROP",
                            "ALTER",
                        ],
                    ) =>
                {
                    break;
                }
                Some(Token::Semicolon) | Some(Token::RParen) => break,
                Some(Token::Word(ref w)) if is_keyword_eq(w, "AS") => {
                    self.advance();
                    self.write_raw(" AS ");
                    if let Some(Token::Word(alias)) = self.advance() {
                        self.write_raw(&uppercase_keyword(alias));
                    }
                    break;
                }
                Some(Token::Word(ref w)) if !got_alias => {
                    self.advance();
                    self.write_raw(&uppercase_keyword(w));
                    got_alias = true;
                }
                Some(Token::LParen) => {
                    self.advance();
                    self.write_raw("(");
                    self.format_subquery_or_list();
                    self.write_raw(")");
                }
                Some(Token::Operator(op)) if op == "." => {
                    self.advance();
                    self.write_raw(".");
                    if let Some(Token::Word(w)) = self.advance() {
                        self.write_raw(&uppercase_keyword(w));
                    }
                }
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        match &tok {
                            Token::Word(w) => {
                                if got_alias {
                                    self.write_raw(" ");
                                    self.write_raw(&uppercase_keyword(w));
                                    break;
                                } else {
                                    self.write_raw(&uppercase_keyword(w));
                                    got_alias = true;
                                }
                            }
                            _ => self.format_token_inline(tok),
                        }
                    }
                }
                None => break,
            }
        }
        self.trim_trailing_space();
        self.write_raw("\n");
    }

    fn format_subquery_or_list(&mut self) {
        let mut depth = 1usize;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            match self.peek_token() {
                Some(Token::LParen) => {
                    self.advance();
                    self.write_raw("(");
                    depth += 1;
                }
                Some(Token::RParen) => {
                    if depth == 1 {
                        break;
                    }
                    self.advance();
                    self.write_raw(")");
                    depth -= 1;
                }
                Some(Token::Comma) => {
                    self.advance();
                    self.write_raw(", ");
                }
                Some(Token::Word(w)) => {
                    self.advance();
                    self.write_raw(&uppercase_keyword(w));
                    self.write_raw(" ");
                }
                Some(Token::StringLiteral(s)) => {
                    self.advance();
                    self.write_raw(&s);
                    self.write_raw(" ");
                }
                Some(Token::Operator(op)) => {
                    self.advance();
                    self.write_raw(" ");
                    self.write_raw(&op);
                    self.write_raw(" ");
                }
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        self.format_token_inline(tok);
                    }
                }
                None => break,
            }
        }
    }

    fn format_join(&mut self) {
        let mut join_str = String::new();
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            match self.peek_token() {
                Some(Token::Word(ref w)) if is_keyword_eq(w, "JOIN") => {
                    self.advance();
                    join_str.push_str("JOIN ");
                    break;
                }
                Some(Token::Word(ref w))
                    if is_keyword_in(
                        w,
                        &[
                            "INNER", "LEFT", "RIGHT", "CROSS", "FULL", "NATURAL", "OUTER",
                        ],
                    ) =>
                {
                    self.advance();
                    if !join_str.is_empty() {
                        join_str.push(' ');
                    }
                    join_str.push_str(&uppercase_keyword(w));
                    join_str.push(' ');
                }
                Some(_) => break,
                None => break,
            }
        }
        self.write_indent();
        self.write_raw(join_str.trim_end());
        self.write_raw(" ");
        self.format_table_ref_with_alias();
    }

    fn format_on_condition(&mut self) {
        let mut depth = 0usize;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            match self.peek_token() {
                Some(Token::Word(ref w))
                    if depth == 0
                        && is_keyword_in(
                            w,
                            &[
                                "WHERE",
                                "JOIN",
                                "INNER",
                                "LEFT",
                                "RIGHT",
                                "CROSS",
                                "FULL",
                                "NATURAL",
                                "GROUP",
                                "HAVING",
                                "ORDER",
                                "LIMIT",
                                "OFFSET",
                                "UNION",
                                "INTERSECT",
                                "EXCEPT",
                                "RETURNING",
                                "SET",
                                "VALUES",
                                "AND",
                                "OR",
                                "SELECT",
                                "WITH",
                            ],
                        ) =>
                {
                    break;
                }
                Some(Token::Semicolon) | Some(Token::RParen) if depth == 0 => break,
                Some(Token::LParen) => {
                    self.advance();
                    self.write_raw("(");
                    depth += 1;
                }
                Some(Token::RParen) => {
                    self.advance();
                    self.write_raw(")");
                    depth = depth.saturating_sub(1);
                }
                Some(Token::Operator(op)) if op == "." => {
                    self.advance();
                    self.write_raw(".");
                    if let Some(Token::Word(w)) = self.advance() {
                        self.write_raw(&uppercase_keyword(w));
                    }
                }
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        match &tok {
                            Token::Word(w) => {
                                self.write_raw(&uppercase_keyword(w));
                            }
                            Token::StringLiteral(s) => {
                                self.write_raw(s);
                            }
                            Token::Operator(op) => {
                                self.write_raw(op);
                            }
                            _ => self.format_token_inline(tok),
                        }
                        if !matches!(
                            self.peek_token(),
                            Some(Token::LParen)
                                | Some(Token::RParen)
                                | Some(Token::Comma)
                                | Some(Token::Semicolon)
                                | None
                        ) {
                            self.write_raw(" ");
                        }
                    }
                }
                None => break,
            }
        }
        self.trim_trailing_space();
        self.write_raw("\n");
    }

    fn format_where_condition(&mut self) {
        self.format_condition_expr();
    }

    fn format_condition_expr(&mut self) {
        let mut depth = 0usize;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            match self.peek_token() {
                Some(Token::Word(ref w))
                    if depth == 0
                        && is_keyword_in(
                            w,
                            &[
                                "GROUP",
                                "HAVING",
                                "ORDER",
                                "LIMIT",
                                "OFFSET",
                                "JOIN",
                                "INNER",
                                "LEFT",
                                "RIGHT",
                                "CROSS",
                                "FULL",
                                "NATURAL",
                                "UNION",
                                "INTERSECT",
                                "EXCEPT",
                                "RETURNING",
                                "SET",
                                "VALUES",
                                "SELECT",
                                "WITH",
                                "ON",
                            ],
                        ) =>
                {
                    break;
                }
                Some(Token::Word(ref w)) if depth == 0 && is_keyword_eq(w, "AND") => break,
                Some(Token::Word(ref w)) if depth == 0 && is_keyword_eq(w, "OR") => break,
                Some(Token::Semicolon) | Some(Token::RParen) if depth == 0 => break,
                Some(Token::LParen) => {
                    self.advance();
                    self.write_raw("(");
                    depth += 1;
                }
                Some(Token::RParen) => {
                    self.advance();
                    self.write_raw(")");
                    depth = depth.saturating_sub(1);
                }
                Some(Token::Operator(op)) if op == "." => {
                    self.advance();
                    self.write_raw(".");
                    if let Some(Token::Word(w)) = self.advance() {
                        self.write_raw(&uppercase_keyword(w));
                    }
                }
                Some(Token::Operator(op)) if op == "," && depth == 0 => break,
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        match &tok {
                            Token::Word(w) => {
                                self.write_raw(&uppercase_keyword(w));
                            }
                            Token::StringLiteral(s) => {
                                self.write_raw(s);
                            }
                            Token::Operator(op) => {
                                self.write_raw(op);
                            }
                            _ => self.format_token_inline(tok),
                        }
                        if !matches!(
                            self.peek_token(),
                            Some(Token::LParen)
                                | Some(Token::RParen)
                                | Some(Token::Comma)
                                | Some(Token::Semicolon)
                                | None
                        ) {
                            self.write_raw(" ");
                        }
                    }
                }
                None => break,
            }
        }
        self.trim_trailing_space();
        self.write_raw("\n");
    }

    fn format_comma_list(&mut self) {
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            match self.peek_token() {
                Some(Token::Word(ref w))
                    if is_keyword_in(
                        w,
                        &[
                            "WHERE",
                            "GROUP",
                            "HAVING",
                            "ORDER",
                            "LIMIT",
                            "OFFSET",
                            "JOIN",
                            "INNER",
                            "LEFT",
                            "RIGHT",
                            "CROSS",
                            "FULL",
                            "NATURAL",
                            "ON",
                            "UNION",
                            "INTERSECT",
                            "EXCEPT",
                            "RETURNING",
                            "SET",
                            "VALUES",
                            "AND",
                            "OR",
                            "SELECT",
                            "WITH",
                        ],
                    ) =>
                {
                    break;
                }
                Some(Token::Semicolon) | Some(Token::RParen) => break,
                Some(Token::Comma) => {
                    self.advance();
                    self.write_raw(", ");
                }
                Some(Token::Word(ref w))
                    if is_keyword_in(w, &["ASC", "DESC", "NULLS", "FIRST", "LAST"]) =>
                {
                    self.advance();
                    self.write_raw(&uppercase_keyword(w));
                }
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        match &tok {
                            Token::Word(w) => {
                                self.write_raw(&uppercase_keyword(w));
                            }
                            Token::StringLiteral(s) => {
                                self.write_raw(s);
                            }
                            Token::Operator(op) => {
                                self.write_raw(op);
                            }
                            _ => self.format_token_inline(tok),
                        }
                        if !matches!(
                            self.peek_token(),
                            Some(Token::LParen)
                                | Some(Token::RParen)
                                | Some(Token::Comma)
                                | Some(Token::Semicolon)
                                | None
                        ) {
                            self.write_raw(" ");
                        }
                    }
                }
                None => break,
            }
        }
        self.trim_trailing_space();
        self.write_raw("\n");
    }

    fn format_single_expr(&mut self) {
        let mut depth = 0usize;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            match self.peek_token() {
                Some(Token::Word(ref w))
                    if depth == 0
                        && is_keyword_in(
                            w,
                            &[
                                "WHERE",
                                "GROUP",
                                "HAVING",
                                "ORDER",
                                "LIMIT",
                                "OFFSET",
                                "JOIN",
                                "INNER",
                                "LEFT",
                                "RIGHT",
                                "CROSS",
                                "FULL",
                                "NATURAL",
                                "ON",
                                "UNION",
                                "INTERSECT",
                                "EXCEPT",
                                "RETURNING",
                                "SET",
                                "VALUES",
                                "AND",
                                "OR",
                                "SELECT",
                                "WITH",
                            ],
                        ) =>
                {
                    break;
                }
                Some(Token::Semicolon) | Some(Token::RParen) if depth == 0 => break,
                Some(Token::LParen) => {
                    self.advance();
                    self.write_raw("(");
                    depth += 1;
                }
                Some(Token::RParen) => {
                    self.advance();
                    self.write_raw(")");
                    depth = depth.saturating_sub(1);
                }
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        match &tok {
                            Token::Word(w) => {
                                self.write_raw(&uppercase_keyword(w));
                            }
                            Token::StringLiteral(s) => {
                                self.write_raw(s);
                            }
                            Token::Operator(op) => {
                                self.write_raw(op);
                            }
                            _ => self.format_token_inline(tok),
                        }
                        if !matches!(
                            self.peek_token(),
                            Some(Token::LParen)
                                | Some(Token::RParen)
                                | Some(Token::Comma)
                                | Some(Token::Semicolon)
                                | None
                        ) {
                            self.write_raw(" ");
                        }
                    }
                }
                None => break,
            }
        }
        self.trim_trailing_space();
        self.write_raw("\n");
    }

    fn format_with(&mut self) {
        self.advance();
        self.write_indent();
        self.write_raw("WITH");
        if let Some(Token::Word(ref w)) = self.peek_token()
            && is_keyword_eq(w, "RECURSIVE") {
                self.advance();
                self.write_raw(" RECURSIVE");
            }

        let mut first_cte = true;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }

            let cte_name = match self.peek_token() {
                Some(Token::Word(w)) => {
                    self.advance();
                    uppercase_keyword(w)
                }
                Some(Token::Semicolon) => {
                    self.advance();
                    self.write_raw(";\n");
                    break;
                }
                _ => break,
            };

            if first_cte {
                self.write_raw(" ");
            }
            self.write_raw(&cte_name);
            first_cte = false;

            if let Some(Token::Word(ref w)) = self.peek_token()
                && is_keyword_eq(w, "AS") {
                    self.advance();
                    self.write_raw(" AS");
                }

            if let Some(Token::LParen) = self.peek_token() {
                self.advance();
                self.write_raw(" (\n");
                self.indent += 1;

                self.format_statement();

                self.indent -= 1;
                if let Some(Token::RParen) = self.peek_token() {
                    self.advance();
                }
                self.write_indent();
                self.write_raw(")");
            }

            if let Some(Token::Comma) = self.peek_token() {
                self.advance();
                self.write_raw(",\n");
            } else {
                self.write_raw("\n");
                break;
            }
        }

        self.format_statement();
    }

    fn format_insert(&mut self) {
        self.advance();
        self.write_indent();
        self.write_raw("INSERT INTO ");

        if let Some(Token::Word(w)) = self.advance() {
            self.write_raw(&uppercase_keyword(w));
        }

        if let Some(Token::LParen) = self.peek_token() {
            self.advance();
            self.write_raw(" (");
            self.format_comma_list_inline();
            self.write_raw(")\n");
        }

        self.format_from_and_rest();
    }

    fn format_comma_list_inline(&mut self) {
        let mut depth = 1usize;
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            match self.peek_token() {
                Some(Token::RParen) => {
                    if depth == 1 {
                        break;
                    }
                    self.advance();
                    self.write_raw(")");
                    depth -= 1;
                }
                Some(Token::LParen) => {
                    self.advance();
                    self.write_raw("(");
                    depth += 1;
                }
                Some(Token::Comma) => {
                    self.advance();
                    self.write_raw(", ");
                }
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        match &tok {
                            Token::Word(w) => {
                                self.write_raw(&uppercase_keyword(w));
                            }
                            Token::StringLiteral(s) => {
                                self.write_raw(s);
                            }
                            Token::Operator(op) => {
                                self.write_raw(op);
                            }
                            _ => self.format_token_inline(tok),
                        }
                        if !matches!(
                            self.peek_token(),
                            Some(Token::LParen)
                                | Some(Token::RParen)
                                | Some(Token::Comma)
                                | Some(Token::Semicolon)
                                | None
                        ) {
                            self.write_raw(" ");
                        }
                    }
                }
                None => break,
            }
        }
        self.trim_trailing_space();
    }

    fn format_update(&mut self) {
        self.advance();
        self.write_indent();
        self.write_raw("UPDATE ");

        if let Some(Token::Word(w)) = self.advance() {
            self.write_raw(&uppercase_keyword(w));
        }
        self.write_raw("\n");

        self.format_from_and_rest();
    }

    fn format_delete(&mut self) {
        self.advance();
        self.write_indent();
        self.write_raw("DELETE FROM ");

        if let Some(Token::Word(w)) = self.advance() {
            self.write_raw(&uppercase_keyword(w));
        }
        self.write_raw("\n");

        self.format_from_and_rest();
    }

    fn format_ddl(&mut self) {
        self.write_indent();
        let mut line = String::new();
        loop {
            if self.pos >= self.tokens.len() {
                break;
            }
            let tok = self.peek_token();
            match tok {
                Some(Token::Semicolon) => {
                    self.advance();
                    line.push(';');
                    break;
                }
                Some(Token::LParen) => {
                    self.advance();
                    line.push('(');
                    let mut depth = 1usize;
                    loop {
                        if self.pos >= self.tokens.len() {
                            break;
                        }
                        let inner = self.peek_token();
                        match inner {
                            Some(Token::RParen) => {
                                self.advance();
                                line.push(')');
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            Some(Token::LParen) => {
                                self.advance();
                                line.push('(');
                                depth += 1;
                            }
                            Some(Token::Comma) => {
                                self.advance();
                                line.push_str(", ");
                            }
                            Some(Token::Word(w)) => {
                                self.advance();
                                line.push_str(&uppercase_keyword(w));
                                line.push(' ');
                            }
                            Some(Token::StringLiteral(s)) => {
                                self.advance();
                                line.push_str(&s);
                                line.push(' ');
                            }
                            Some(Token::Operator(op)) => {
                                self.advance();
                                line.push(' ');
                                line.push_str(&op);
                                line.push(' ');
                            }
                            Some(_) => {
                                if let Some(tok) = self.advance() {
                                    match tok {
                                        Token::Word(w) => line.push_str(&uppercase_keyword(w)),
                                        Token::StringLiteral(s) => line.push_str(&s),
                                        _ => {}
                                    }
                                }
                            }
                            None => break,
                        }
                    }
                }
                Some(Token::Word(w)) => {
                    self.advance();
                    line.push_str(&uppercase_keyword(w));
                    line.push(' ');
                }
                Some(Token::StringLiteral(s)) => {
                    self.advance();
                    line.push_str(&s);
                    line.push(' ');
                }
                Some(Token::Operator(op)) => {
                    self.advance();
                    line.push(' ');
                    line.push_str(&op);
                    line.push(' ');
                }
                Some(_) => {
                    if let Some(tok) = self.advance() {
                        match tok {
                            Token::Word(w) => line.push_str(&uppercase_keyword(w)),
                            Token::StringLiteral(s) => line.push_str(&s),
                            _ => {}
                        }
                    }
                }
                None => break,
            }
        }
        self.write_raw(line.trim());
        self.write_raw("\n");
    }

    fn format_transaction(&mut self) {
        let word = match self.peek_word() {
            Some(w) => uppercase_keyword(w),
            None => return,
        };
        self.advance();
        self.write_indent();
        self.write_raw(&word);

        if is_keyword_eq(&word, "BEGIN")
            && let Some(Token::Word(w)) = self.peek_token()
                && is_keyword_eq(&w, "TRANSACTION") {
                    self.advance();
                    self.write_raw(" TRANSACTION");
                }

        self.write_raw(";\n");
    }
}

/// Formats a raw SQL string into a standardized, human-readable layout.
///
/// This is the primary entry point for SQL formatting. It tokenizes the input
/// SQL and applies the project's formatting rules, including:
///
/// - Keyword uppercasing
/// - Clause-level line breaks (`SELECT`, `FROM`, `WHERE`, etc.)
/// - One field per line in `SELECT` lists
/// - `AND`/`OR` indentation under `WHERE`
/// - `ON` indentation under `JOIN`
/// - `CASE`/`WHEN`/`ELSE`/`END` nesting
/// - CTE (`WITH`) formatting
/// - Semicolon termination
///
/// # Arguments
///
/// * `sql` - The raw SQL string to format.
///
/// # Returns
///
/// A formatted SQL string. If the input is empty, returns an empty string.
///
/// # Example
///
/// ```
/// use sql_admin_frontend::utils::sql_formatter::format_sql;
/// let result = format_sql("select id, name from users where id = 1");
/// assert!(result.contains("SELECT\n  id"));
/// assert!(result.contains("FROM"));
/// assert!(result.contains("WHERE"));
/// assert!(result.ends_with(';'));
/// ```
pub fn format_sql(sql: &str) -> String {
    let tokens = tokenize_sql(sql);
    let formatter = SqlFormatter::new(&tokens);
    formatter.format()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_select() {
        let sql = "select * from users where id = 1";
        let result = format_sql(sql);
        assert!(result.contains("SELECT"));
        assert!(result.contains("FROM"));
        assert!(result.contains("WHERE"));
        assert!(!result.contains("select"));
    }

    #[test]
    fn test_select_fields_per_line() {
        let sql = "select id, name, email from users";
        let result = format_sql(sql);
        assert!(result.contains("SELECT\n"));
        let lines: Vec<&str> = result.lines().collect();
        let select_idx = lines.iter().position(|l| l.starts_with("SELECT")).unwrap();
        assert!(lines[select_idx + 1].contains("id"));
    }

    #[test]
    fn test_and_or_at_line_start() {
        let sql = "select * from users where status = 'active' and city is not null and create_time >= '2026-01-01' order by create_time desc limit 100";
        let result = format_sql(sql);
        assert!(result.contains("\n  AND "));
        assert!(result.contains("ORDER BY"));
        assert!(result.contains("LIMIT"));
    }

    #[test]
    fn test_join_on_separate() {
        let sql = "select u.id, u.name from users u left join orders o on u.id = o.user_id where o.amount > 100";
        let result = format_sql(sql);
        assert!(result.contains("LEFT JOIN"));
        assert!(result.contains("\n  ON "));
    }

    #[test]
    fn test_case_when() {
        let sql = "select id, case when status = 'active' then '有效' when status = 'disabled' then '禁用' else '未知' end as status_desc from users";
        let result = format_sql(sql);
        assert!(result.contains("CASE"));
        assert!(result.contains("WHEN"));
        assert!(result.contains("THEN"));
        assert!(result.contains("ELSE"));
        assert!(result.contains("END"));
    }

    #[test]
    fn test_with_cte() {
        let sql = "with user_base as (select id, name, city from user where status = 'active'), order_stat as (select user_id, count(*) as order_count from orders group by user_id) select ub.id, ub.name, os.order_count from user_base ub left join order_stat os on ub.id = os.user_id where ub.city is not null order by ub.id desc";
        let result = format_sql(sql);
        assert!(result.contains("WITH"));
        assert!(result.contains("AS"));
        assert!(result.starts_with("WITH"));
    }

    #[test]
    fn test_keywords_uppercase() {
        let sql = "select id from users where name is not null and status in ('active', 'pending') order by id desc limit 10";
        let result = format_sql(sql);
        assert!(!result.contains("select"));
        assert!(!result.contains("from"));
        assert!(!result.contains("where"));
        assert!(result.contains("SELECT"));
        assert!(result.contains("FROM"));
        assert!(result.contains("WHERE"));
        assert!(result.contains("IS NOT NULL"));
        assert!(result.contains("IN"));
        assert!(result.contains("ORDER BY"));
        assert!(result.contains("DESC"));
        assert!(result.contains("LIMIT"));
    }

    #[test]
    fn test_string_preserved() {
        let sql = "select * from users where name = 'John Doe'";
        let result = format_sql(sql);
        assert!(result.contains("'John Doe'"));
    }

    #[test]
    fn test_semicolon_appended() {
        let sql = "select * from users";
        let result = format_sql(sql);
        assert!(result.ends_with(';'));
    }

    #[test]
    fn test_insert_statement() {
        let sql = "insert into users (id, name) values (1, 'Alice')";
        let result = format_sql(sql);
        assert!(result.contains("INSERT INTO"));
        assert!(result.contains("VALUES"));
    }

    #[test]
    fn test_update_statement() {
        let sql = "update users set name = 'Bob' where id = 1";
        let result = format_sql(sql);
        assert!(result.contains("UPDATE"));
        assert!(result.contains("SET"));
        assert!(result.contains("WHERE"));
    }

    #[test]
    fn test_delete_statement() {
        let sql = "delete from users where id = 1";
        let result = format_sql(sql);
        assert!(result.contains("DELETE FROM"));
        assert!(result.contains("WHERE"));
    }

    #[test]
    fn test_two_space_indent() {
        let sql = "select id, name from users where status = 'active' and city = 'NYC' order by id";
        let result = format_sql(sql);
        for line in result.lines() {
            if line.starts_with("  ") {
                let indent = line.len() - line.trim_start().len();
                assert_eq!(indent % 2, 0, "Indentation should be multiples of 2 spaces");
            }
        }
    }

    #[test]
    fn test_spec_example_standard() {
        let sql = "with user_base as (select u.id, u.name, u.city from user u where u.status = 'active'), order_stat as (select o.user_id, count(*) as order_count from orders o group by o.user_id) select ub.id, ub.name, ub.city, os.order_count from user_base ub left join order_stat os on ub.id = os.user_id where ub.city is not null order by ub.id desc";
        let result = format_sql(sql);
        let expected = "WITH user_base AS (\n  SELECT\n    u.id,\n    u.name,\n    u.city\n  FROM user u\n  WHERE u.status = 'active'\n),\norder_stat AS (\n  SELECT\n    o.user_id,\n    COUNT(*) AS order_count\n  FROM orders o\n  GROUP BY o.user_id\n)\nSELECT\n  ub.id,\n  ub.name,\n  ub.city,\n  os.order_count\nFROM user_base ub\nLEFT JOIN order_stat os\n  ON ub.id = os.user_id\nWHERE ub.city IS NOT NULL\nORDER BY ub.id DESC;";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_spec_example_conditions() {
        let sql = "select u.id, u.name, u.status from user u where u.status = 'active' and u.city is not null and u.create_time >= '2026-01-01' order by u.create_time desc limit 100";
        let result = format_sql(sql);
        assert!(result.contains("SELECT\n  u.id"));
        assert!(result.contains("WHERE u.status = 'active'"));
        assert!(result.contains("\n  AND u.city IS NOT NULL"));
        assert!(result.contains("\n  AND u.create_time >= '2026-01-01'"));
        assert!(result.contains("ORDER BY u.create_time DESC"));
        assert!(result.contains("LIMIT 100"));
    }

    #[test]
    fn test_spec_example_case_when() {
        let sql = "select u.id, u.name, case when u.status = 'active' then '有效' when u.status = 'disabled' then '禁用' else '未知' end as status_desc from user u";
        let result = format_sql(sql);
        assert!(result.contains("CASE"));
        assert!(result.contains("WHEN u.status = 'active' THEN '有效'"));
        assert!(result.contains("WHEN u.status = 'disabled' THEN '禁用'"));
        assert!(result.contains("ELSE '未知'"));
        assert!(result.contains("END AS status_desc"));
    }

    #[test]
    fn test_format_output_no_trailing_whitespace() {
        let sql = "select id from users";
        let result = format_sql(sql);
        for line in result.lines() {
            assert_eq!(
                line,
                line.trim_end(),
                "Line has trailing whitespace: {:?}",
                line
            );
        }
    }

    #[test]
    fn test_spec_example_conditions_exact() {
        let sql = "select u.id, u.name, u.status from user u where u.status = 'active' and u.city is not null and u.create_time >= '2026-01-01' order by u.create_time desc limit 100";
        let result = format_sql(sql);
        let expected = "SELECT\n  u.id,\n  u.name,\n  u.status\nFROM user u\nWHERE u.status = 'active'\n  AND u.city IS NOT NULL\n  AND u.create_time >= '2026-01-01'\nORDER BY u.create_time DESC\nLIMIT 100;";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_spec_example_case_when_exact() {
        let sql = "select u.id, u.name, case when u.status = 'active' then '有效' when u.status = 'disabled' then '禁用' else '未知' end as status_desc from user u";
        let result = format_sql(sql);
        let expected = "SELECT\n  u.id,\n  u.name,\n  CASE\n    WHEN u.status = 'active' THEN '有效'\n    WHEN u.status = 'disabled' THEN '禁用'\n    ELSE '未知'\n  END AS status_desc\nFROM user u;";
        assert_eq!(result, expected);
    }
}
