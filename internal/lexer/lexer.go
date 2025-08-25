package lexer

import (
	"strconv"
)

type Lexer struct {
	line     int
	column   int
	cursor   Cursor
	tokens   []SpannedToken
	keywords map[string]TokenType
}

func NewLexer(code []rune) *Lexer {
	return &Lexer{
		line:   0,
		column: 0,
		cursor: NewCursor(code),
		tokens: make([]SpannedToken, 0, 64),
		keywords: map[string]TokenType{
			"fun":    TokFun,
			"for":    TokFor,
			"match":  TokMatch,
			"use":    TokUse,
			"struct": TokStruct,
			"enum":   TokEnum,
			"let":    TokLet,
			"pub":    TokPub,
			"priv":   TokPriv,
			"module": TokModule,
		},
	}
}

func (l *Lexer) Tokenize() ([]SpannedToken, error) {
	for !l.cursor.IsAtEnd() {
		ch, ok := l.advance()
		if !ok {
			return nil, UnexpectedEOF()
		}
		switch ch {
		case ':':
			l.addTk(TokColon)
		case ';':
			l.addTk(TokSemicolon)
		case '+':
			if r, ok := l.cursor.Peek(); ok && r == '=' {
				l.addTk(TokCompoundPlus)
			} else {
				l.addTk(TokPlus)
			}
		case '-':
			if r, ok := l.cursor.Peek(); ok && r == '=' {
				l.addTk(TokCompoundMinus)
			} else {
				l.addTk(TokMinus)
			}
		case '*':
			if r, ok := l.cursor.Peek(); ok && r == '=' {
				l.addTk(TokCompoundStar)
			} else {
				l.addTk(TokStar)
			}
		case '/':
			if r, ok := l.cursor.Peek(); ok && r == '=' {
				l.addTk(TokCompoundSlash)
			} else if ok && r == '/' { /* TODO comments */
			} else {
				l.addTk(TokSlash)
			}
		case '&':
			if r, ok := l.cursor.Peek(); ok && r == '&' {
				l.addTk(TokAnd)
			} else {
				return nil, LexError{Kind: ErrIndexOutOfRange, Message: "unsupported single &"}
			}
		case '|':
			if r, ok := l.cursor.Peek(); ok && r == '|' {
				l.addTk(TokOr)
			} else {
				return nil, LexError{Kind: ErrIndexOutOfRange, Message: "unsupported single |"}
			}
		case '^':
			l.addTk(TokXor)
		case '[':
			l.addTk(TokLeftSquareBracket)
		case ']':
			l.addTk(TokRightSquareBracket)
		case '(':
			l.addTk(TokLeftParen)
		case ')':
			l.addTk(TokRightParen)
		case '{':
			l.addTk(TokLeftBrace)
		case '}':
			l.addTk(TokRightBrace)
		case '.':
			l.addTk(TokDot)
		case '!':
			if r, ok := l.cursor.Peek(); ok && r == '=' {
				l.addTk(TokNotEquals)
			} else {
				l.addTk(TokNot)
			}
		case '=':
			if r, ok := l.cursor.Peek(); ok && r == '=' {
				l.addTk(TokEquals)
			} else {
				l.addTk(TokAssign)
			}
		case '>':
			if r, ok := l.cursor.Peek(); ok && r == '=' {
				l.addTk(TokGreaterEquals)
			} else {
				l.addTk(TokGreater)
			}
		case '<':
			if r, ok := l.cursor.Peek(); ok && r == '=' {
				l.addTk(TokLessEquals)
			} else {
				l.addTk(TokLess)
			}
		case ',':
			l.addTk(TokComma)
		case '"':
			strTok, err := l.tokenizeString()
			if err != nil {
				return nil, err
			}
			l.addSpannedTk(NewSpannedToken(strTok, Span{Start: l.cursor.current, End: l.cursor.current + 1}))
		case ' ':
		case '\t':
		case '\n':
			l.line += 1
		case '\x00':
		default:
			if ch >= '0' && ch <= '9' {
				var st SpannedToken
				var err error
				if ch == '0' {
					if r, e := l.cursor.OffsetResult(0); e == nil && r == 'x' {
						st, err = l.tokenizeHexNumber()
					}
					if err == nil {
						if r, e := l.cursor.OffsetResult(0); e == nil && r == 'b' {
							st, err = l.tokenizeBinNumber()
						}
					}
					if err == nil {
						if r, e := l.cursor.OffsetResult(0); e == nil && r == 'o' {
							st, err = l.tokenizeOctNumber()
						}
					}
				}
				if st.Token.Type == 0 && err == nil {
					st, err = l.tokenizeNumber(ch)
				}
				if err != nil {
					return nil, err
				}
				l.addSpannedTk(st)
			} else if (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_' {
				st, err := l.tokenizeId(ch)
				if err != nil {
					return nil, err
				}
				l.addSpannedTk(st)
			}
		}
	}
	out := make([]SpannedToken, len(l.tokens))
	copy(out, l.tokens)
	l.tokens = l.tokens[:0]
	return out, nil
}

func (l *Lexer) Cleanup() {
	l.cursor.current = 0
	l.tokens = l.tokens[:0]
}

func (l *Lexer) advance() (rune, bool) {
	r, ok := l.cursor.Peek()
	if !ok {
		return 0, false
	}
	l.cursor.current += 1
	l.column += 1
	return r, true
}

func (l *Lexer) addTk(t TokenType) {
	l.tokens = append(l.tokens, NewSingleCharToken(t, l.cursor.current))
}

func (l *Lexer) addSpannedTk(st SpannedToken) {
	l.tokens = append(l.tokens, st)
}

func (l *Lexer) tokenizeString() (Token, error) {
	var text []rune
	for !l.cursor.IsAtEnd() {
		r, err := l.cursor.OffsetResult(0)
		if err != nil {
			return Token{}, err
		}
		if r == '"' {
			break
		}
		rr, ok := l.advance()
		if !ok {
			return Token{}, UnexpectedEOF()
		}
		text = append(text, rr)
	}
	if _, ok := l.advance(); !ok {
		return Token{}, UnexpectedEOF()
	}
	return Token{Type: TokString, Text: string(text)}, nil
}

func (l *Lexer) tokenizeHexNumber() (SpannedToken, error) {
	start := l.cursor.current - 1
	if _, ok := l.advance(); !ok {
		return SpannedToken{}, UnexpectedEOF()
	}
	var digits []rune
	for !l.cursor.IsAtEnd() {
		r, err := l.cursor.OffsetResult(0)
		if err != nil {
			return SpannedToken{}, err
		}
		if (r >= '0' && r <= '9') || (r >= 'a' && r <= 'f') || (r >= 'A' && r <= 'F') {
			rr, _ := l.advance()
			digits = append(digits, rr)
		} else {
			break
		}
	}
	end := l.cursor.current
	v, err := strconv.ParseInt(string(digits), 16, 64)
	if err != nil {
		return SpannedToken{}, LexError{Kind: ErrFailedToParseNumber, Message: "failed to parse hex"}
	}
	return NewSpannedToken(Token{Type: TokInteger, I64: v}, Span{Start: start, End: end}), nil
}

func (l *Lexer) tokenizeBinNumber() (SpannedToken, error) {
	start := l.cursor.current - 1
	if _, ok := l.advance(); !ok {
		return SpannedToken{}, UnexpectedEOF()
	} // b
	var digits []rune
	for !l.cursor.IsAtEnd() {
		r, err := l.cursor.OffsetResult(0)
		if err != nil {
			return SpannedToken{}, err
		}
		if r == '0' || r == '1' {
			rr, _ := l.advance()
			digits = append(digits, rr)
		} else {
			break
		}
	}
	end := l.cursor.current
	v, err := strconv.ParseInt(string(digits), 2, 64)
	if err != nil {
		return SpannedToken{}, LexError{Kind: ErrFailedToParseNumber, Message: "failed to parse bin"}
	}
	return NewSpannedToken(Token{Type: TokInteger, I64: v}, Span{Start: start, End: end}), nil
}

func (l *Lexer) tokenizeOctNumber() (SpannedToken, error) {
	start := l.cursor.current - 1
	if _, ok := l.advance(); !ok {
		return SpannedToken{}, UnexpectedEOF()
	}
	var digits []rune
	for !l.cursor.IsAtEnd() {
		r, err := l.cursor.OffsetResult(0)
		if err != nil {
			return SpannedToken{}, err
		}
		if r >= '0' && r <= '7' {
			rr, _ := l.advance()
			digits = append(digits, rr)
		} else {
			break
		}
	}
	end := l.cursor.current
	v, err := strconv.ParseInt(string(digits), 8, 64)
	if err != nil {
		return SpannedToken{}, LexError{Kind: ErrFailedToParseNumber, Message: "failed to parse oct"}
	}
	return NewSpannedToken(Token{Type: TokInteger, I64: v}, Span{Start: start, End: end}), nil
}

func (l *Lexer) tokenizeNumber(first rune) (SpannedToken, error) {
	start := len(string(first))
	number := []rune{first}
	isDecimal := false
	for !l.cursor.IsAtEnd() {
		r, _ := l.advance()
		if (r >= '0' && r <= '9') || r == '.' {
			if r == '.' {
				if isDecimal {
					return SpannedToken{}, LexError{Kind: ErrFailedToParseNumber, Message: "multiple dots"}
				}
				isDecimal = true
			}
			number = append(number, r)
		} else {
			break
		}
	}
	end := l.cursor.current
	s := string(number)
	if isDecimal {
		f, err := strconv.ParseFloat(s, 64)
		if err != nil {
			return SpannedToken{}, LexError{Kind: ErrFailedToParseNumber, Message: "failed to parse float"}
		}
		return NewSpannedToken(Token{Type: TokFloat, F64: f}, Span{Start: start, End: end}), nil
	}
	i, err := strconv.ParseInt(s, 10, 64)
	if err != nil {
		return SpannedToken{}, LexError{Kind: ErrFailedToParseNumber, Message: "failed to parse int"}
	}
	return NewSpannedToken(Token{Type: TokInteger, I64: i}, Span{Start: start, End: end}), nil
}

func (l *Lexer) tokenizeId(first rune) (SpannedToken, error) {
	start := len(string(first))
	id := []rune{first}
	for !l.cursor.IsAtEnd() {
		r, ok := l.cursor.Peek()
		if !ok {
			return SpannedToken{}, UnexpectedEOF()
		}
		if (r >= 'a' && r <= 'z') || (r >= 'A' && r <= 'Z') || r == '_' {
			rr, _ := l.advance()
			id = append(id, rr)
		} else {
			break
		}
	}
	end := l.cursor.current
	s := string(id)
	if t, ok := l.keywords[s]; ok {
		return NewSpannedToken(Token{Type: t}, Span{Start: start, End: end}), nil
	}
	return NewSpannedToken(Token{Type: TokIdentifier, Text: s}, Span{Start: start, End: end}), nil
}
