package main

import (
	"fmt"
	"waffle/internal/lexer"
)

func main() {

	code2 := []rune("fun test(): int { return 42 }")
	l2 := lexer.NewLexer(code2)
	sts2, err2 := l2.Tokenize()
	if err2 != nil {
		fmt.Printf("Error: %v\n", err2)
		return
	}

	fmt.Printf("%+v\n", sts2)
}
