package lexer

import "fmt"

type LexErrorKind int

const (
	ErrIo LexErrorKind = iota
	ErrUtf8
	ErrUnexpectedEof
	ErrIndexOutOfRange
	ErrFailedToParseNumber
)

type LexError struct {
	Kind    LexErrorKind
	Message string
}

func (e LexError) Error() string {
	if e.Message == "" {
		return fmt.Sprintf("lexer error: %v", e.Kind)
	}
	return e.Message
}

func UnexpectedEOF() error { return LexError{Kind: ErrUnexpectedEof, Message: "unexpected EOF"} }
