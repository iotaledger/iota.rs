package org.iota.client.build;

import java.io.File;
import java.io.IOException;
import java.nio.file.FileVisitResult;
import java.nio.file.FileVisitor;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;
import java.nio.file.attribute.BasicFileAttributes;
import static java.util.Arrays.asList;

import java.util.Arrays;
import java.util.Date;
import java.util.LinkedList;
import java.util.List;
import java.util.Locale;

/**
 * Provides the functionality to compile Rust crates
 * as a maven action.
 */
public class CompileRustCrates {

    private static final Date EPOCH = new Date(0);
    private static final Path RUST_OUTPUT_DIR = Paths.get("..", "c", "target", "release");

    public static void main(String[] args) throws Exception {
        if (changesDetected()) {
            System.out.println("Changes detected. Compiling all Rust crates!");
            CompileRustCrates.compile(cratePath());
        } else {
            System.out.println("No changes detected. Not recompiling Rust crates.");
        }
    }

    private static boolean changesDetected() throws IOException {
        Date lastSourceChange = newestChange(cratePath().resolve("src/lib.rs"));
        Path compiled = compiledRustLibraryPath();
        if (compiled == null) {
            return true;
        }
        Date lastCompilation = newestChange(compiled);
        return lastSourceChange.getTime() > lastCompilation.getTime();
    }

    private static Path cratePath() {
        return Paths.get("..", "c");
    }

    private static void compile(Path cratePath) {
        System.out.format("Compiling crate %s%n", cratePath);
        try {
            Process process = cargoProcess(cratePath).inheritIO().start();
            process.waitFor();
            if (process.exitValue() != 0) {
                throw new RuntimeException(String.format("cargo exited nonzero (status code = %s)", process.exitValue()));
            }
            moveLibIntoClasspath(compiledRustLibraryPath());
        } catch (IOException | InterruptedException ex) {
            throw new RuntimeException(ex);
        }
    }

    private static ProcessBuilder cargoProcess(Path cratePath) {
        List<String> commandParts = asList("cargo", "build", "--release");
        System.out.format("Running command: %s%n", commandParts);
        return new ProcessBuilder(commandParts)
            .directory(cratePath.toFile());
    }

    private static void moveLibIntoClasspath(Path library) {
        try {
            Path outputDir = outputDir();
            outputDir.toFile().mkdirs();
            System.out.format("Installing %s into %s%n", library, outputDir);
            Files.copy(library, outputDir.resolve(library.getFileName()), StandardCopyOption.REPLACE_EXISTING);
        } catch (IOException ex) {
            throw new RuntimeException(ex);
        }
    }

    private static Path outputDir() {
        return Paths.get("target", "classes", osArchName());
    }

    private static String osArchName() {
        return Os.getCurrent().jnaArchString();
    }

    private static Path compiledRustLibraryPath() throws IOException {
        if (!RUST_OUTPUT_DIR.toFile().exists()) {
            return null;
        }
        return RUST_OUTPUT_DIR.resolve(
            osArchName().equals("darwin") ? "libiota.dylib" : "libiota.so"
        );
    }

    private static Date newestChange(Path path) {
        return CompileRustCrates.mtime(path);
    }

    @SuppressWarnings("CallToPrintStackTrace")
    private static Date mtime(Path path) {
        try {
            return new Date(Files.getLastModifiedTime(path).toMillis());
        } catch (IOException ex) {
            ex.printStackTrace();
            return EPOCH;
        }
    }

    private enum Os {
        MAC_OS("mac", "darwin") {
            @Override
            public String jnaArchString() {
                return "darwin";
            }
        },
        WINDOWS("win") {
            @Override
            public String jnaArchString() {
                return currentIs64Bit() ? "win32-x86-64" : "win32-x86";
            }
        },
        GNU_SLASH_LINUX("nux") {
            @Override
            public String jnaArchString() {
                return currentIs64Bit() ? "linux-x86-64" : "linux-x86";
            }
        },
        UNKNOWN() {
            @Override
            public String jnaArchString()  {
                throw new RuntimeException("Unknown platform. Can't tell what platform we're running on!");
            }
        };
        private final String[] substrings;

        Os(String... substrings) {
            this.substrings = substrings;
        }

        public abstract String jnaArchString();

        public static Os getCurrent() {
            return Arrays.stream(values())
                .filter(Os::isCurrent)
                .findFirst()
                .orElse(UNKNOWN);
        }

        public boolean isCurrent() {
            return Arrays.stream(substrings)
                .anyMatch(substring -> currentOsString().contains(substring));
        }

        private static boolean currentIs64Bit() {
            return System.getProperty("os.arch").contains("64");
        }

        private static String currentOsString() {
            return System.getProperty("os.name", "unknown").toLowerCase(Locale.ENGLISH);
        }
    }
}
