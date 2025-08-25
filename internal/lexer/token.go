package lexer

type TokenType int

const (
	TokFun TokenType = iota
	TokModule
	TokUse
	TokStruct
	TokPriv
	TokPub
	TokLet
	TokMatch
	TokEnum
	TokFor

	TokLeftParen
	TokRightParen
	TokLeftBrace
	TokRightBrace
	TokLeftSquareBracket
	TokRightSquareBracket
	TokColon
	TokSemicolon
	TokComma
	TokDot

	TokAssign
	TokCompoundPlus
	TokCompoundMinus
	TokCompoundStar
	TokCompoundSlash
	TokCompoundMod
	TokPlus
	TokMinus
	TokStar
	TokSlash
	TokMod
	TokAnd
	TokOr
	TokXor
	TokLeftArrow
	TokRightArrow
	TokGreater
	TokLess
	TokEquals
	TokNotEquals
	TokLessEquals
	TokGreaterEquals
	TokNot

	TokInteger
	TokFloat
	TokIdentifier
	TokString
	TokBoolean
)

type Token struct {
	Type TokenType
	I64  int64
	F64  float64
	Text string
	Bool bool
}

type Span struct {
	Start int
	End   int
}

type SpannedToken struct {
	Token Token
	Span  Span
}

func NewSpannedToken(t Token, s Span) SpannedToken {
	return SpannedToken{Token: t, Span: s}
}

func NewSingleCharToken(t TokenType, start int) SpannedToken {
	return SpannedToken{Token: Token{Type: t}, Span: Span{Start: start, End: start + 1}}
}
