package src.test.kotlin
import kotlin.test.Test
import kotlin.test.assertEquals
import NumberOperations

class NumberOperationsTest {
    
    @Test
    fun testGetMultiplesOfN() {
        val numberOps = NumberOperations()
        val result = numberOps.getMultiplesOfN(3)
        val expected = "Multiple of 3: 3\n" +
                       "Multiple of 3: 6\n" +
                       "Multiple of 3: 9\n" +
                       "Multiple of 3: 12\n" +
                       "Multiple of 3: 15\n" +
                       "Multiple of 3: 18\n" +
                       "Multiple of 3: 21\n" +
                       "Multiple of 3: 24\n" +
                       "Multiple of 3: 27\n" +
                       "Multiple of 3: 30\n"
        assertEquals(expected, result)
    }
    
    @Test
    fun testCalculateFactorial() {
        val numberOps = NumberOperations()
        assertEquals(1, numberOps.calculateFactorial(0))
        assertEquals(1, numberOps.calculateFactorial(1))
        assertEquals(120, numberOps.calculateFactorial(5))
    }
}
