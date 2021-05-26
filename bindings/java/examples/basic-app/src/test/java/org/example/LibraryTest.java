package org.example;

import org.junit.Test;

public class LibraryTest {
    @Test
    public void testSomeLibraryMethod() {
        try {
            ExampleApp.customPayload();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
