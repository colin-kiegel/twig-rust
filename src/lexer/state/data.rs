/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Data state of the lexer.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */
use super::*;

#[allow(dead_code)]
struct Data;

impl State for Data {
    fn lex(&self) -> Result<Option<Box<State>>,SyntaxError> {
    /*
            // if no matches are left we return the rest of the template as simple text token
        if (self.position == self.token_iter.count() - 1) {
            tokens.push(Token::new(
                token::Type::Eof,
                token::Value("".to_string()),
                cursor.get_position() 
            ));
        
            self.pushToken(Twig_Token::TEXT_TYPE, substr($this->code, $this->cursor));
            $this->cursor = $this->end;

            return;
        }

        // Find the first token after the current cursor
        $position = $this->positions[0][++$this->position];
        while ($position[1] < $this->cursor) {
            if ($this->position == count($this->positions[0]) - 1) {
                return;
            }
            $position = $this->positions[0][++$this->position];
        }

        // push the template text first
        $text = $textContent = substr($this->code, $this->cursor, $position[1] - $this->cursor);
        if (isset($this->positions[2][$this->position][0])) {
            $text = rtrim($text);
        }
        $this->pushToken(Twig_Token::TEXT_TYPE, $text);
        $this->moveCursor($textContent.$position[0]);

        switch ($this->positions[1][$this->position][0]) {
            case $this->options['tag_comment'][0]:
                $this->lexComment();
                break;

            case $this->options['tag_block'][0]:
                // raw data?
                if (preg_match($this->regexes['lex_block_raw'], $this->code, $match, null, $this->cursor)) {
                    $this->moveCursor($match[0]);
                    $this->lexRawData($match[1]);
                // {% line \d+ %}
                } elseif (preg_match($this->regexes['lex_block_line'], $this->code, $match, null, $this->cursor)) {
                    $this->moveCursor($match[0]);
                    $this->lineno = (int) $match[1];
                } else {
                    $this->pushToken(Twig_Token::BLOCK_START_TYPE);
                    $this->pushState(self::STATE_BLOCK);
                    $this->currentVarBlockLine = $this->lineno;
                }
                break;

            case $this->options['tag_variable'][0]:
                $this->pushToken(Twig_Token::VAR_START_TYPE);
                $this->pushState(self::STATE_VAR);
                $this->currentVarBlockLine = $this->lineno;
                break;
        }
        */
        unimplemented!();
    }
}
