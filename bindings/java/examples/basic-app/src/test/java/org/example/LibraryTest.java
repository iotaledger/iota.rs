package org.example;

import org.junit.Test;

public class LibraryTest {
    @Test
    public void testSomeLibraryMethod() {
        try {
            ExampleApp.generateSeed();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
