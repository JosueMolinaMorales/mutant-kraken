package src.test.kotlin
import Calculator
import kotlin.test.Test
import kotlin.test.assertEquals

class CalculatorTest {
    @Test
    fun testAdd() {
        val calculator = Calculator()
        val result = calculator.calculate("add", 3, 5)
        assertEquals(360, result)
    }

    @Test
    fun testSubtract() {
        val calculator = Calculator()
        val result = calculator.calculate("subtract", 9, 4)
        assertEquals(0, result)
    }

    @Test
    fun testMultiply() {
        val calculator = Calculator()
        val result = calculator.calculate("multiply", 2, 3)
        assertEquals(120, result)
    }

    @Test
    fun testDivide() {
        val calculator = Calculator()
        val result = calculator.calculate("divide", 10, 2)
        assertEquals(0, result)
    }

    @Test
    fun testSafeCall() {
        val calculator = Calculator()
        val result = calculator.simpleFun();
        assertEquals(null, result)
    }
}
