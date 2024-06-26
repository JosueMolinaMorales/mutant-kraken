class Calculator {
    fun calculate(operation: String, a: Int, b: Int): Int {
        var result = 0;
        if (operation == "add") {
            result = a + b
        } else if (operation == "subtract") {
            result = a - b
        } else {
            throw IllegalArgumentException("Unknown operation")
        }
        return result
    }

    fun simpleFun(): Int? {
        var test:String? = null
        var result:Int? = test!!.length
        return result
    }

    fun ElvisLiteralChangeOperator() {
        var b: String? = null
        var x = b?.length ?: -11
        var y = b?.length ?: "Hello There"
        var z = b?.length ?: 1.0
        var a = b?.length ?: 1.0f
        var c = b?.length ?: 1L

        if (x > 1) {
            println("Len is greater than 1!")
        }
    
        println(x)
    }
    fun WhenExpression() {
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
        val box: Box<Int> = Box<Int>(1)
    }


}

class Box<T>(t: T) {
    var value = t
}

fun copy(from: Array<Any>, to: Array<Any>) {
    assert(from.size == to.size)
    for (i in from.indices)
        to[i] = from[i]
}