pub trait Conv: Sized {
	fn conv(mut lhs: Vec<Self>, mut rhs: Vec<Self>) -> Vec<Self> {
		Conv::conv_in_place(&mut lhs, &mut rhs);
		lhs
	}
	fn conv_in_place(lhs: &mut Vec<Self>, rhs: &mut Vec<Self>);
}
