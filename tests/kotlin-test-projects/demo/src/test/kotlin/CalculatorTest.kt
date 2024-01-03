package src.test.kotlin
import Calculator
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class CalculatorTest {
    @Test
    fun testAdd() {
        val calculator = Calculator()
        val result = calculator.calculate("add", 3, 5)
        assertEquals(8, result)
    }

    @Test
    fun testSubtract() {
        val calculator = Calculator()
        val result = calculator.calculate("subtract", 9, 4)
        assertEquals(5, result)
    }

    @Test
    fun testSafeCall() {
        val calculator = Calculator()
        assertFailsWith<NullPointerException>(
            block = {calculator.simpleFun()}
        )
    }
}
