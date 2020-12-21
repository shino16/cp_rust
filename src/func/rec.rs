pub struct Recurse<F>(F);

impl<F> Recurse<F> {
	pub fn call<Arg, Ret>(&self, arg: Arg) -> Ret
	where
		F: Fn(&dyn Fn(Arg) -> Ret, Arg) -> Ret,
	{
		self.0(&|arg| self.call(arg), arg)
	}
}

pub fn recurse<Arg, Ret, F: Fn(&dyn Fn(Arg) -> Ret, Arg) -> Ret>(f: F) -> Recurse<F> {
	Recurse(f)
}
