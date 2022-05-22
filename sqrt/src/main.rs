fn main() {
    let target = 100f64;
    let ans = find_sqrt(target);
    println!("{:.1}", ans);
}


fn find_sqrt(target:f64) -> f64{
    let mut ans = 50.0;
    let mut interval = 100f64;

    for _ in 0..30000 {
        if ans * ans < target {
            ans += interval;
        } else if  ans * ans > target {
            ans -= interval;
        } else {
            return ans;
        }

        interval -= 1f64;
    }

    return ans
}
