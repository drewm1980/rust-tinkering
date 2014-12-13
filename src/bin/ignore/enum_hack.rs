
/// This is a macro hack for an enum that you can iterate over
#![feature(macro_rules)]
macro_rules! create_enum(
		( $enum_name : ident , $a_field_name : ident ) => 
		( 
		enum $enum_name { $a_field_name };
		static foovec: [$enum_name,..1] = [ $a_field_name ] ; 
		)
;)

create_enum! (Direction , NORTH)
//create_enum! (Direction , NORTH, SOUTH, EAST, WEST);

//enum Direction { NORTH, SOUTH, EAST, WEST }
//static DIRECTIONS: [Direction, ..4] = [NORTH, SOUTH, EAST, WEST];

fn main() {
	let foo:Direction = NORTH;
    println!("foo = {}" , foo);
}

#[cfg(test)]
mod test {
    extern crate test;

    #[test]
    fn use_constants () {
        let mut foo = super::FOO;
        let mut bar = super::BAR;
    }

}
