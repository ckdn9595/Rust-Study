fn main(){
    let t = (32, true, 3.14);
    let x = t.0;
    let y = t.1;
    let z = t.2;
    let tuple = t;

    let arr:[i32; 5] = [1,2,3,4,5]; //배열은 동일한 타입만 // 러스트는 배열의 길이가 고정

    println(x[0]);
    println(x[4]);

    let threes = ["헬로", 10];
    println(threes[9])
}