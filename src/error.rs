#[macro_export]
macro_rules! composite_error {
	($e:ident, $($n:ident($t:ty)),*) => {
		#[derive(Debug)]
		enum $e {
			$($n($t),)*
		}
		$(
			impl From<$t> for $e {
				fn from(error: $t) -> Self {
					$e::$n(error)
				}
			}
		)*
		impl ::std::fmt::Display for $e {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				match *self {
					$(
						$e::$n(ref error) => error.fmt(f),
					)*
				}
			}
		}
	};
}
