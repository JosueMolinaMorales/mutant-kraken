/*
fun main() {
	// Given a string containing brackets of 4 types: (), [], {}, <>
	// Check whether brackets are in the correct sequence, ignore all other characters
	var input = """
    (a+[b*c]-{d/3})
    (a + [b * c) - 17]
    (((a * x) + [b] * y) + c
    auf(zlo)men [gy<psy>] four{s}
    """.trimIndent()

    // split the string on \n
    var lines = input.split('\n')
    var res = mutableListOf<Int>()
    for (line in lines) {
        res.add(isBracketSequenceCorrect(line)) 
    }
    println(res.joinToString(" "))
}

fun isBracketSequenceCorrect(line: String): Int {
    var stack = mutableListOf<Char>()
    val bracketPairs = mapOf<Char, Char>('{' to '}', '(' to ')', '{' to '}', '<' to '>', '[' to ']')
    for (char in line) {
        if (!(char in bracketPairs.keys) && !(char in bracketPairs.values)) continue
        if (stack.isEmpty()) {
            stack.add(char)
            continue
        }
        when (char) {
            in bracketPairs.keys -> {
                // keys is the opening
                stack.add(char)
            }
            in bracketPairs.values -> {
                // get the top of the stack & get its closing pair
                val pair = bracketPairs.getOrDefault(stack.last(), ' ')
                if (char != pair) {
                    return 0
                }
                stack.removeAt(stack.lastIndex)
            }
        }
        
    }
    return if (stack.isEmpty()) 1 else 0
}
*/
/*
fun main() {
    // Arithmetic expressions
    val a = 10
    val b = 3
    val c = a + b
    val d = a - b
    val e = a * b
    val f = a / b
    val g = a % b

    // Assignment operators
    var h = 5
    h += 3
    h -= 1
    h *= 2
    h /= 4
    h %= 2

    // Conditional operators
    val i = 7
    val j = 8
    val k = if (i < j) i else j
    val l = if (i > j) i else j
    val m = if (i <= j) i else j
    val n = if (i >= j) i else j
    val o = if (i == j) i else j
    val p = if (i != j) i else j

    // Loops
    for (q in 1..5) {
        println(q)
    }

    var r = 0
    while (r < 5) {
        r++
        println(r)
    }

    var s = 0
    do {
        s++
        println(s)
    } while (s < 5)

    // Logical expressions
    val t = true
    val u = false
    val v = t && u
    val w = t || u
    val x = !t

    println(c)
    println(d)
    println(e)
    println(f)
    println(g)
    println(h)
    println(k)
    println(l)
    println(m)
    println(n)
    println(o)
    println(p)
    println(v)
    println(w)
    println(x)
}
*/

fun main() {
    var x = 10
    val y = 5
    println("x = $x, y = $y")
    
    if (x > y) {
        println("x is greater than y")
    } else {
        println("x is not greater than y")
    }
    
    while (x > 0) {
        x--
    }
    println("x is now $x")
    
    for (i in 0 until 5) {
        println("i = $i")
    }
    
    val z = x + y
    val w = x * y
    println("z = $z, w = $w")
    
    val s1 = "hello"
    val s2 = "world"
    val s3 = s1 + s2
    println("s3 = $s3")
    
    val b1 = true
    val b2 = false
    val b3 = !b1
    val b4 = b1 && b2
    val b5 = b1 || b2
    println("b3 = $b3, b4 = $b4, b5 = $b5")
    
    val a = arrayOf(1, 2, 3, 4, 5)
    for (i in a) {
        println("i = $i")
    }
    
    val map = mapOf("key1" to "value1", "key2" to "value2")
    for ((k, v) in map) {
        println("$k = $v")
    }
    
    val nullableInt: Int? = null
    val nonNullableInt: Int = nullableInt ?: 0
    println("nonNullableInt = $nonNullableInt")
    
    val string: String? = "hello"
    val length: Int? = string?.length
    println("length = $length")
    
    val range = 1..5
    for (i in range step 2) {
        println("i = $i")
    }
    
    val numList = listOf(1, 2, 3, 4, 5)
    val filteredList = numList.filter { it % 2 == 0 }
    println("filteredList = $filteredList")
    
    val person = Person("John", 25)
    println("person = $person")
}

data class Person(val name: String, val age: Int)