pub const KOTLIN_TEST_CODE: &str = r#"
fun main() {
    // Arithmetic expressions
    val a = 10
    val b = 3
    val c = a + b
    val d = a - b
    val e = a * b
    val f = a / b
    val g = a % b
}
"#;

pub const KOTLIN_RELATIONAL_TEST_CODE: &str = r#"
fun main() {
    // Relational expressions
    val a = 10
    val b = 3
    val c = a > b
    val d = a < b
    val e = a >= b
    val f = a <= b
    val g = a == b
    val h = a != b
}
"#;

pub const KOTLIN_LOGICAL_TEST_CODE: &str = r#"
fun main() {
    // Logical expressions
    val a = true
    val b = false
    val c = a && b
    val d = a || b
}
"#;

pub const KOTLIN_UNARY_TEST_CODE: &str = r#"
var h = 5
h++
h--
++h
--h
"#;

pub const KOTLIN_UNARY_REMOVAL_TEST_CODE: &str = r#"
var h = 5
h++
h--
++h
--h
val a = !h
val b = -h
val c = +h
"#;

pub const KOTLIN_ASSIGNMENT_TEST_CODE: &str = r#"
var h = 5
h += 3
h -= 1
h *= 2
h /= 4
h %= 2
"#;