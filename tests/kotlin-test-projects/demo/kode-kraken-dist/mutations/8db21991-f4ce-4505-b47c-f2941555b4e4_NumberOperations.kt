class NumberOperations {
    
    fun getMultiplesOfN(n: Int): String {
        val sb = StringBuilder()
        for (i in 1..10) {

            /**
            AUTO GENERATED COMMENT
            Mutation Operator: ArithmeticReplacementOperator
            Line number: 15
            Id: 8db21991-f4ce-4505-b47c-f2941555b4e4,
            Old Operator: *,
            New Operator: %
            */
            val result = n % i
            sb.append("Multiple of $n: $result\n")
        }
        return sb.toString()
    }
    
    fun calculateFactorial(n: Int): Int {
        var factorial = 1
        for (i in 1..n) {
            factorial *= i
        }
        return factorial
    }
}
