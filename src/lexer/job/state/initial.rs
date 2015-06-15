/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Initial state of the lexer job.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */
use super::*;
use lexer::SyntaxError;
use lexer::job::Job;
use super::_final::Final;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Initial;//<'a>(Box<InnerData<'a>>);

impl Initial {
/*    fn test<T>(self: Box<Self>) -> Result<Box<Tokenize<'a>>,SyntaxError>
    where T: Tokenize<'a> {
        let Initial(data) = *self;
        let state = Final::new(data);// as &mut Tokenize;
        //Ok(state)
        unimplemented!();
        
        //let state = *Final::new(data) as Tokenize;
        //Ok(Box::new(state))
    }*/
}

impl Tokenize for Initial {

    /*fn new(data: Box<InnerData<'a>>) -> Box<Self> {
        Box::new(Initial(data))
    }*/
    
    fn new() -> Box<Self> {
        Box::new(Initial)
    }
    
    fn is_finished(&self) -> bool {
        false
    }
    
    // this one requires a read o
    // https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
    // http://stackoverflow.com/questions/29985153/trait-object-is-not-object-safe-error
    //fn lex<T>(self: Box<Self>) -> Result<Box<Tokenize<'a>>,SyntaxError>;
    
    fn step(&self, job: &mut Job) -> Result<Box<Tokenize>,SyntaxError> {
        Ok(Final::new())
        //let Initial(data) = *self;
        //unimplemented!()
        //let state = _final::Final::new(data);
        //Ok(Initial::new(data))
    }
}
