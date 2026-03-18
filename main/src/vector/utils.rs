use crate::vector::core::Vector;

impl <const N : usize> Vector<N>{
    pub fn display(&self){
        for component in self.data{
            println!("{}",component);
        }
    }
}