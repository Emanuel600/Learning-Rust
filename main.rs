mod source1;

fn main(){
	let new_comp= source1::Complex::new;

	let comp1= new_comp(1.0, 2.0);
	let comp2= new_comp(0.0, 1.0);

	let op= source1::COps::Mul;

	let comp3= op.run(comp1, comp2);

	println!("{}", comp3);
}
