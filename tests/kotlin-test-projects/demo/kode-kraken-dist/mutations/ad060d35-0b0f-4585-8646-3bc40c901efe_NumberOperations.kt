class NumberOperations {
    
    fun getMultiplesOfN(n: Int): String {
        val sb = StringBuilder()
        for (i in 1..10) {
            val result = n * i
            sb.append("Multiple of $n: $result\n")
        }
        return sb.toString()
    }
    
    fun calculateFactorial(n: Int): Int {
        var factorial = 1
        for (i in 1..n) {

            /**
            AUTO GENERATED COMMENT
            Mutation Operator: AssignmentReplacementOperator
            Line number: 24
            Id: ad060d35-0b0f-4585-8646-3bc40c901efe,
            Old Operator: *=,
            New Operator: +=
            */
            factorial += i
        }
        return factorial
    }
}
