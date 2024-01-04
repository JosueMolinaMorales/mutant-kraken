mod builder;
mod mutation;
mod operators;
mod tool;

pub use builder::*;
pub use mutation::*;
pub use operators::*;
pub use tool::*;

pub fn debug_print_ast(ast: &tree_sitter::Node, spaces: usize) {
    // Print the node
    println!("{}{:?}", " ".repeat(spaces), ast.kind());
    // Go through the children
    for child in ast.children(&mut ast.walk()) {
        debug_print_ast(&child, spaces + 2);
    }
}

#[cfg(test)]
pub mod test_util {
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

    pub const KOTLIN_ELVIS_TEST_CODE: &str = r#"
fun main() {
    val a = 10
    val b = 3
    val c = a ?: b
}
"#;

    pub const KOTLIN_ELVIS_LITERAL_TEST_CODE: &str = r#"
fun main() {
    val a: String? = null
    val b = a ?: "b"
    val c: Int? = null
    val d = c ?: 1
    val e = c ?: -10
    val f: Boolean? = null
    val g = e ?: true
    val h: Double? = null
    val i = h ?: 2.0
    val j = h ?: -3.0
    val k: Float? = null
    val l = k ?: 4.0f
    val m = k ?: -5.0f
    val n: Long? = null
    val o = n ?: 6L
    val p = n ?: -7L
    val q: Char? = null
    val r = q ?: 'a'
    val s = q ?: 'b'
}
"#;

    pub const KOTLIN_LITERAL_TEST_CODE: &str = r#"
fun main() {
    val a = 1
    val b = -2
    val c = 4.0
    val d = -6.0
    val e = 12L
    val f = -13L
    val g = 14.0f
    val h = -16.0f
    val i = true
    val j = false
    val k = 'a'
    val l = "a"
}
"#;

    pub const KOTLIN_TEST_NULL_ASSERTION_CODE: &str = r#"
fun main() {
    val a: String? = null
    val b = a!!
    val c = a?.length
}
"#;

    pub const KOTLIN_EXCEPTION_TEST_CODE: &str = r#"
fun main() {
    var result = 0;
        if (operation == "add") {
            result = a + b
        } else if (operation == "subtract") {
            result = a - b
        } else if (operation == "multiply") {
            throw UnsupportedOperationException("Multiplication is not supported")
        } else if (operation == "divide") {
            throw DivideByZeroException("Division by zero is not allowed")
        } else {
            throw IllegalArgumentException("Unknown operation")
        }
        
        return result
}
"#;

    pub const KOTLIN_WHEN_EXPRESSION_TEST_CODE: &str = r#"
fun main() {
    val a = 10
    val b = 3
    val c = when (a) {
        1 -> 1
        2 -> 2
        3 -> 3
        else -> 0
    }
    val d = when (a) {
        1 -> 1
        2 -> 2
        3 -> 3
        else -> 0
    }
"#;

    pub const KOTLIN_LABEL_REMOVING_TEST_CODE: &str = r#"
    fun main() {
        listOf(1, 2, 3, 4, 5).forEach lit@{
            if (it == 3) return@lit // local return to the caller of the lambda - the forEach loop
            print(it)
        }
        
        print(" done with explicit label")

        outerLoop@ for (i in 1..5) {
            innerLoop@ for (j in 1..3) {
                println("i: $i, j: $j")

                when (j) {
                    2 -> {
                        println("Breaking inner loop")
                        break@innerLoop
                    }
                    3 -> {
                        println("Continuing to the next iteration of outer loop")
                        continue@outerLoop
                    }
                }
            }
        }
    }
"#;
}
