package lexer

type Cursor struct {
    code   []rune
    current int
}

func NewCursor(code []rune) Cursor {
    return Cursor{code: code, current: 0}
}

func (c *Cursor) Peek() (rune, bool) {
    if c.IsAtEnd() { return 0, false }
    r, ok := c.Offset(0)
    if !ok { return 0, false }
    return r, true
}

func (c *Cursor) IsAtEnd() bool {
    return c.current >= len(c.code)
}

func (c *Cursor) Offset(offset int) (rune, bool) {
    index := c.current + offset
    if index < len(c.code) {
        return c.code[index], true
    }
    return 0, false
}

func (c *Cursor) OffsetResult(offset int) (rune, error) {
    index := c.current + offset
    if index < len(c.code) {
        return c.code[index], nil
    }
    return 0, UnexpectedEOF()
}


