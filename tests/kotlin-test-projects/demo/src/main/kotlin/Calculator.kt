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

    fun ExceptionThrowing() {
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
    }

    fun LabelRemoval() {
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

    fun FunctionalFun() {
        val numbers = listOf(1, 2, 3, 4, 5)

        // Using any() to check if there is at least one even number
        val hasEvenNumber = numbers.any { it % 2 == 0 }
        println("Has even number: $hasEvenNumber")

        // Using all() to check if all numbers are even
        val allEvenNumbers = numbers.all { it % 2 == 0 }
        println("All numbers are even: $allEvenNumbers")

        // Using none() to check if there are no even numbers
        val noEvenNumbers = numbers.none { it % 2 == 0 }
        println("No even numbers: $noEvenNumbers")

        // Using Filter() to filter the list
        val evenNumbers = numbers.filter { it % 2 == 0 }
        println("Even numbers: $evenNumbers")

        // Using Map() to map the numbers to their squares
        val squaredNumbers = numbers.map { it * it }
        println("Squared numbers: $squaredNumbers")

        // Using ForEach() to print each number
        numbers.forEach { println(it) }

    }

}
