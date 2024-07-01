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

            /**
            AUTO GENERATED COMMENT
            Mutation Operator: AssignmentReplacementOperator
            Line number: 32
            Id: e4f91eaa-6011-44cd-ae0b-0668eda62769,
            Old Operator: *=,
            New Operator: /=
            */
            result /= j
        }
        return result
    }

    fun simpleFun(): Int? {
        var test:String? = null
        var result:Int? = test!!.length
        return result
    }
}