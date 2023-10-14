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
}
