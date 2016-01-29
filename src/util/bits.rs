use std::ops::BitXor;

pub fn xor_bits(buf1: &[u8], buf2: &[u8]) -> Vec<u8> {
    xor(buf1.iter(), buf2.iter())
}

/// A generic method which XORs everything from the given iterators and collects the result into a
/// Vec.
// Yes, this function is unnecessarily generic as we only work with u8s, but it was an interesting
// learning experience. Credit to nayru25
pub fn xor<'a, 'b, T, U, I, J>(itr1:I, itr2: J) -> Vec<<T as BitXor<U>>::Output>
    where T: 'a + BitXor<U>,
          U: 'b,
          I: Iterator<Item=T>,
          J: Iterator<Item=U>
{
    itr1.zip(itr2).map(|(a,b)| a ^ b).collect()
}
