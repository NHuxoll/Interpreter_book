package token

type TokenType string

type Token struct {
		Type TokenType
		Literal string
}

const (
		ILLEGAL = "ILLEGAL"
		EOF = "EOF"

		IDENT = "IDENT"
		INT = "INT"

		ASSIGN = "="
		PLUS = "+"
		MINUS = "-"
		BANG = "!"
		ASTERISK = "*"
		SLASH = "/"

		EQ = "=="
		NOT_EQ ="!="

		LT = "<"
		GT = ">"

		COMMA = ","
		SEMICOLON = ";"

		LPAREN ="("
		RPAREN = ")"
		LSQUIRLY = "{"
		RSQUIRLY = "}"
        // Keywords
		FUNCTION = "FUNCTION"
		LET = "LET"
		TRUE = "true"
		FALSE = "false"
		IF = "if"
		ELSE = "else"
		RETURN = "return"
)

var keywords = map[string]TokenType{
	"fn": FUNCTION,
	"let": LET,
	"true": TRUE,
	"false": FALSE,
	"if": IF,
	"else": ELSE,
	"return": RETURN,
}

func LookupIdent(ident string) TokenType {
	if tok, ok := keywords[ident]; ok {
		return tok
	}
	return IDENT
}
