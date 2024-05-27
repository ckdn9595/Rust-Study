pub fn hello(){
    panic!("에러 발생");
}

pub fn get_value(num : usize) -> i32{
    let v = vec![1,2,3,4];
    v[num]
}