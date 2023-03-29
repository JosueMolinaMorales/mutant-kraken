class Calculator {
    fun calculate(operation: String, a: Int, b: Int): Int {
        var result = 0
        if (operation == "add") {
            result = a + b
        } else if (operation == "subtract") {
            result = a - b
        } else if (operation == "multiply") {
            result = a * b
        } else if (operation == "divide") {
            result = a / b
        }
        var i = 0
        while (i < 10) {
            if (i % 2 == 0) {
                result += i
            } else {
                result -= i
            }
            i++
        }
        for (j in 1..5) {
            result *= j
        }
        return result
    }
}