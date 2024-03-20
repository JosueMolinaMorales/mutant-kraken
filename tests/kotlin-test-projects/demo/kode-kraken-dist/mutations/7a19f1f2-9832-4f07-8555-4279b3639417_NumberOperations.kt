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
            Id: 7a19f1f2-9832-4f07-8555-4279b3639417,
            Old Operator: *=,
            New Operator: /=
            */
            factorial /= i
        }
        return factorial
    }
}
