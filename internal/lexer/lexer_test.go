package lexer

import "testing"

func tokensOf(sts []SpannedToken) []TokenType {
	out := make([]TokenType, 0, len(sts))
	for _, st := range sts {
		out = append(out, st.Token.Type)
	}
	return out
}

func TestNumberTokenize(t *testing.T) {
	code := []rune("123456.7890")
	l := NewLexer(code)
	got, err := l.Tokenize()
	if err != nil {
		t.Fatalf("tokenize error: %v", err)
	}
	// простите
	if len(got) != 1 ||
		got[0].Token.Type != TokFloat ||
		got[0].Token.F64 != 123456.7890 {

		t.Fatalf("unexpected: %#v", got)
	}
}

func TestBinaryNumberTokenize(t *testing.T) {
	code := []rune("0b1010101")
	l := NewLexer(code)
	got, err := l.Tokenize()
	if err != nil {
		t.Fatalf("tokenize error: %v", err)
	}
	if len(got) != 1 || got[0].Token.Type != TokInteger || got[0].Token.I64 != 85 {
		t.Fatalf("unexpected: %#v", got)
	}
}

func TestOctalNumberTokenize(t *testing.T) {
	code := []rune("0o7777")
	l := NewLexer(code)
	got, err := l.Tokenize()
	if err != nil {
		t.Fatalf("tokenize error: %v", err)
	}
	if len(got) != 1 || got[0].Token.Type != TokInteger || got[0].Token.I64 != 4095 {
		t.Fatalf("unexpected: %#v", got)
	}
}

func TestStringTokenize(t *testing.T) {
	code := []rune("\"hello, world!\"")
	l := NewLexer(code)
	sts, err := l.Tokenize()
	if err != nil {
		t.Fatalf("tokenize error: %v", err)
	}
	if len(sts) != 1 {
		t.Fatalf("unexpected len: %d", len(sts))
	}
	if sts[0].Token.Type != TokString || sts[0].Token.Text != "hello, world!" {
		t.Fatalf("unexpected: %#v", sts[0])
	}
}

func TestKeywordsTokenize(t *testing.T) {
	code := []rune("match a _ for")
	l := NewLexer(code)
	sts, err := l.Tokenize()
	if err != nil {
		t.Fatalf("tokenize error: %v", err)
	}
	if len(sts) != 4 {
		t.Fatalf("unexpected len: %d", len(sts))
	}
	if sts[0].Token.Type != TokMatch {
		t.Fatalf("expected match, got %#v", sts[0])
	}
	if sts[1].Token.Type != TokIdentifier || sts[1].Token.Text != "a" {
		t.Fatalf("id a failed: %#v", sts[1])
	}
	if sts[2].Token.Type != TokIdentifier || sts[2].Token.Text != "_" {
		t.Fatalf("id _ failed: %#v", sts[2])
	}
	if sts[3].Token.Type != TokFor {
		t.Fatalf("expected for, got %#v", sts[3])
	}
}

func TestKeywordsTokenize2(t *testing.T) {
	code := []rune("fun a(): int {}")
	l := NewLexer(code)
	sts, err := l.Tokenize()
	if err != nil {
		t.Fatalf("tokenize error: %v", err)
	}
	wantTypes := []TokenType{TokFun, TokIdentifier, TokLeftParen, TokRightParen, TokColon, TokIdentifier, TokLeftBrace, TokRightBrace}
	if len(sts) != len(wantTypes) {
		t.Fatalf("unexpected len: %d", len(sts))
	}
	for i, wt := range wantTypes {
		if sts[i].Token.Type != wt {
			t.Fatalf("idx %d: want %v got %v", i, wt, sts[i].Token.Type)
		}
	}
}
