Below is a list of know divergences from the behavior of CPython

## Tokenizer

- Rye returns exact numerical tokens. So 12 returns NUMBER:INTEGER while
  .12 returns NUMBER:FLOAT while CPython returns NUMBER:NUMBER for both

- CPython at all versions will return an `error: EOF in multi-line statement`
  for unbalanced RPAR ")", RSQB "]", and RBRACE "}". Rye will only return an
  error for unbalanced LPAR "(", LSQB "[", and LBRACE "{"

- CPython will always follow a comment with NL or NEWLINE, even if there is
  no newline in the source
