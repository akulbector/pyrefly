/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::testcase;

testcase!(
    test_typeof_basic_int_literal,
    r#"
from typing import assert_type, cast, Literal
from pyrefly_extensions import typeof

# typeof extracts the precise inferred type (Literal[42] in this case)
x = 42
y = cast(typeof[x], "not 42")
assert_type(y, Literal[42])
"#,
);

testcase!(
    test_typeof_basic_string_literal,
    r#"
from typing import assert_type, cast, Literal
from pyrefly_extensions import typeof

# typeof extracts the precise inferred type (Literal["hello"] in this case)
s = "hello"
t = cast(typeof[s], 42)
assert_type(t, Literal["hello"])
"#,
);

testcase!(
    test_typeof_tuple_literals,
    r#"
from typing import assert_type, cast, Literal
from pyrefly_extensions import typeof

mytuple = (1, "b", False)
similartuple = cast(typeof[mytuple], "wrong type")
assert_type(similartuple, tuple[Literal[1], Literal["b"], Literal[False]])
"#,
);

testcase!(
    test_typeof_annotated_type,
    r#"
from typing import assert_type, cast
from pyrefly_extensions import typeof

# When a variable has an explicit annotation, typeof uses that annotation
mydict: dict[str, list[int]] = {"a": [1, 2]}
similar = cast(typeof[mydict], {"b": [3, 4]})
assert_type(similar, dict[str, list[int]])
"#,
);

testcase!(
    test_typeof_list,
    r#"
from typing import assert_type, cast
from pyrefly_extensions import typeof

# When a variable has an explicit annotation, typeof uses that annotation
mylist: list[int] = [1, 2, 3]
similar_list = cast(typeof[mylist], "not a list")
assert_type(similar_list, list[int])
"#,
);

testcase!(
    test_typeof_undefined_variable,
    r#"
from typing import cast
from pyrefly_extensions import typeof

y = cast(typeof[undefined_var], 42)  # E: Could not find name `undefined_var`
"#,
);

testcase!(
    test_typeof_no_argument,
    r#"
from pyrefly_extensions import typeof

x: typeof = 42  # E: Expected a type argument for `typeof`  # E: is not assignable
"#,
);

testcase!(
    test_typeof_too_many_args,
    r#"
from typing import cast
from pyrefly_extensions import typeof

x = 42
z = 100
y = cast(typeof[x, z], 42)  # E: `typeof` requires exactly one argument but got 2
"#,
);

testcase!(
    test_typeof_complex_expression_not_supported,
    r#"
from typing import cast
from pyrefly_extensions import typeof

class Point:
    x: int
    y: int

p = Point()
y = cast(typeof[p.x], 10)  # E: `typeof` requires a simple variable name
"#,
);

testcase!(
    test_typeof_scope_resolution,
    r#"
from typing import assert_type, cast, Literal
from pyrefly_extensions import typeof

def outer():
    x = 42
    def inner():
        # typeof resolves x from outer scope
        y = cast(typeof[x], "wrong")
        assert_type(y, Literal[42])
        return y
    return inner()
"#,
);

testcase!(
    test_typeof_function_parameter,
    r#"
from pyrefly_extensions import typeof

mytuple = (1, "b", False)

def process_data(data: typeof[mytuple]) -> int:
    return len(data)
"#,
);

testcase!(
    test_typeof_return_type,
    r#"
from pyrefly_extensions import typeof

mytuple = (1, "b", False)

def create_similar() -> typeof[mytuple]:
    return (1, "b", False)
"#,
);

testcase!(
    test_typeof_union_type,
    r#"
from typing import assert_type, cast
from pyrefly_extensions import typeof

# When a variable has an explicit annotation, typeof uses that annotation
x: int | str = 42
y = cast(typeof[x], "hello")
assert_type(y, int | str)
"#,
);

testcase!(
    test_typeof_none_type,
    r#"
from typing import assert_type, cast
from pyrefly_extensions import typeof

x = None
y = cast(typeof[x], 42)
assert_type(y, None)
"#,
);

testcase!(
    test_typeof_class_instance,
    r#"
from typing import assert_type, cast
from pyrefly_extensions import typeof

class Foo:
    pass

x = Foo()
y = cast(typeof[x], "not a Foo")
assert_type(y, Foo)
"#,
);

testcase!(
    test_typeof_widened_int,
    r#"
from typing import assert_type, cast
from pyrefly_extensions import typeof

# When the variable is annotated with int, typeof gives int (not Literal)
x: int = 42
y = cast(typeof[x], 100)
assert_type(y, int)
"#,
);
