
pub struct Vector<const N: usize>{
    pub data : [f32; N]
}

impl<const N: usize> Vector<N>{
    pub fn new(data : [f32; N])-> Self{
        return Vector {data};
    }
}