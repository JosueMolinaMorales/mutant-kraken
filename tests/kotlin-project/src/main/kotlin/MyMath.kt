class MyMath {
    fun isEven(number: Int): Boolean {
        return number % 2 == 0
    }

    fun isOdd(number: Int): Boolean {
        return number % 2 != 0
    }

    fun factorial(n: Int): Int {
        return if (n == 0) {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    fun gcd(a: Int, b: Int): Int {
        return if (b == 0) {
            a
        } else {
            gcd(b, a % b)
        }
    }
}