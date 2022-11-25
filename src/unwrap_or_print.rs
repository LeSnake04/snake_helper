#[macro_export]
/**
# Unwrap or print
## Arguments
1. Result
2. Optional: println-like macro to print error with (log::error! by default)
3. Action: Code to run. Must exit the current context
*/
macro_rules! unwrap_or_print_err {
	($res: expr, $op: stmt) => {
		match $res {
			Ok(r) => r,
			Err(e) => {
				::log::error!("{}", e);
				$op
			}
		}
	};
	($res: expr, $printfn: ident, $op: stmt) => {
		match $res {
			Ok(r) => r,
			Err(e) => {
				$printfn!("{}", e);
				$op
			}
		}
	};
}
