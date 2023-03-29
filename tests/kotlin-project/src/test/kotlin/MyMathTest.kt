package src.test.kotlin
import MyMath
import kotlin.test.Test
import kotlin.test.assertEquals

class MyMathTest {
    
    @Test
    fun testIsEven() {
        val myMath = MyMath()
        assertTrue(myMath.isEven(2))
        assertFalse(myMath.isEven(3))
    }
    
    @Test
    fun testIsOdd() {
        val myMath = MyMath()
        assertTrue(myMath.isOdd(3))
        assertFalse(myMath.isOdd(2))
    }
    
    @Test
    fun testFactorial() {
        val myMath = MyMath()
        assertEquals(1, myMath.factorial(0))
        assertEquals(1, myMath.factorial(1))
        assertEquals(120, myMath.factorial(5))
    }
    
    @Test
    fun testGcd() {
        val myMath = MyMath()
        assertEquals(6, myMath.gcd(30, 18))
        assertEquals(4, myMath.gcd(12, 8))
    }
}
