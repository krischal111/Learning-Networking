pub fn popcount<T>(mut a:T) -> usize 
where T : Copy + std::cmp::PartialEq + From<u8> + std::ops::Sub 
+ std::ops::BitAndAssign<<T as std::ops::Sub>::Output>
{
    let mut i = 0;
    while a!= 0.into()  {
        a &= a-1.into();
        i += 1;
    }
    i
}
#[test]
fn test_count(){
    let a = 7;
    let count = popcount(a);
    println!("Bit count of {a:b} is {count}");
    assert_eq!(4, popcount(15));
}