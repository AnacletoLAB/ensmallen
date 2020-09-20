
#[macro_use]
extern crate afl;
extern crate mycrate; //the crate we want to fuzz

fn main() {
    fuzz!(|data: &[u8]| {
	mycrate::myfunction(&data); //call the function you want to fuzz with the input from afl
    });

}
