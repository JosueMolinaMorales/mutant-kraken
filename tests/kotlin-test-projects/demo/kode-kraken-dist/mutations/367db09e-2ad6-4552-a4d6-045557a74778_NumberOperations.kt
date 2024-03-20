class NumberOperations {
    
    fun getMultiplesOfN(n: Int): String {
        val sb = StringBuilder()
        for (i in 1..10) {

            /**
            AUTO GENERATED COMMENT
            Mutation Operator: ArithmeticReplacementOperator
            Line number: 15
            Id: 367db09e-2ad6-4552-a4d6-045557a74778,
            Old Operator: *,
            New Operator: -
            */
            val result = n - i
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
