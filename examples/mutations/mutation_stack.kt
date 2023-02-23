
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

fun comparisons() {
    val x = 10
    val y = 11
    if (x < y) {
        println("X is less than Y")
    }
    
    if (x != y) {
        println("X equals Y")
    }
}