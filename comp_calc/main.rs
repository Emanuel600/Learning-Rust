mod operations;

fn main(){
	let new_comp= operations::Complex::new;

	let comp1= new_comp(0.0, 0.0);
	let comp2= new_comp(1.0, 0.0);

	println!("{}", comp1/comp2);
}
