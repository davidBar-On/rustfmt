// Tests for original #3943 issue
use ::foo;
use ::foo::Bar1;
use ::foo::{Bar2, Baz2};
use ::Foo;
use ::{Bar3, Baz3};

use ::foo;
use ::foo::Bar;
use ::foo::{Bar, Baz};
use ::Foo;
use ::{Bar, Baz};

use ::foo;
use ::foo::Bar;
use ::foo::{Bar, Baz};
use ::Foo;
use ::{Bar, Baz};

// Additional tests for signle item `{}` handling
use super::auxvec;
use crate::detect::{cache, Feature};
use ::AAAA;
use ::BBBB;
use aaaa::BBBB;
use bbbbb::AAAA;

// Tests with comments and "as"
use a::{/* pre-comment */ item};
use a::{item /* post-comment */};
use a::{/* pre-comment */ item /* post-comment */};

// Misc
use self::std::fs as self_fs;
use ::foo;
use ::foo as bar;
use ::foo::{foo, Foo};
use ::Foo;
use ::Foo as baz;
use ::Foo1;
use ::*;
use dummy;
use std;
use Super::foo;