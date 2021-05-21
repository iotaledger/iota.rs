package org.example;

import org.junit.Test;

public class LibraryTest {
    @Test
    public void testSomeLibraryMethod() {
        try {
            ExampleApp.getDataMessage();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
