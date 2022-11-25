use snake_helper::{into_none_if, unwrap_or_print_err};

#[test]
fn test() {
	let a = 5;
	assert_eq!(into_none_if!(a == 0, a), Some(5));
	let b = 0;
	assert_eq!(into_none_if!(b == 0, b), None)
}

#[test]
fn unwrap_print() {
	unwrap_or_print_err!(Ok::<i32, &str>(3), println, {
		panic!();
	});
	unwrap_or_print_err!(Ok::<i32, &str>(3), {
		panic!();
	});
}
