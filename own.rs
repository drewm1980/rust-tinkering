fn swallow(x:i64){
	println!("swallowing x, which has value {}",x)
}
fn main() {
	let a:i64 = 10;
	swallow(a);
	println!("{}",a)
}
