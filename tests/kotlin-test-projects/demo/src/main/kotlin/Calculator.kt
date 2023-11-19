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
        // x = b?.length ?: c
        var z = b?.length ?: 1.0
        var a = b?.length ?: 1.0f
        var c = b?.length ?: 1L
        // x = b?.length ?: 'a'
        // x = b?.length ?: 1.toShort()
        // x = b?.length ?: 1.toChar()
    
        // if (x > 1) {
        //     println("Len is greater than 1!")
        // }
    
        println(x)
    }
}
