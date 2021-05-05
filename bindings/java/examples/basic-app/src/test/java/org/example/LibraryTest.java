package org.example;

import org.junit.Test;

public class LibraryTest {
    @Test
    public void testSomeLibraryMethod() {
        try {
            new ExampleApp();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
