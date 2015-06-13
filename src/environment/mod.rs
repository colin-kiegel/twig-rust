/*
 * This file is part of Twig (ported to Rust).
 *
 * For the copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/**
 * Stores the Twig configuration.
 *
 * @author Colin Kiegel <kiegel@gmx.de>
 */

#[allow(dead_code)]
const VERSION : &'static str = "1.18.1";

#[derive(Default)]
pub struct Environment;

#[allow(dead_code)]
impl Environment {
    /**
     * Gets the registered unary Operators.
     *
     * @return array An array of unary operators
     */
    pub fn get_unary_operators(&mut self) -> Option<String> {
        /*if !self.extension_initialized {
            self.init_extensions();
        }

        self.unary_operators*/
        None
    }

    /**
     * Gets the registered binary Operators.
     *
     * @return array An array of binary operators
     */
    pub fn get_binary_operators(&mut self) -> Option<String> {
        /*if !self.extension_initialized {
            self.init_extensions();
        }

        self.binary_operators*/
        None
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test {
    use super::*;

    //#[test]
    pub fn get_unary_operators() {
        let mut e = Environment;
        e.get_unary_operators();
        unimplemented!();
    }

    //#[test]
    pub fn get_binary_operators() {
        let mut e = Environment;
        e.get_binary_operators();
        unimplemented!();
    }
}
