# SQLCipher (Windows)

## Prerequisites

- [OpenSSL](https://slproweb.com/products/Win32OpenSSL.html)

- [ActiveTCL](https://www.activestate.com/products/tcl/)

- [Visual Studio](https://visualstudio.microsoft.com/downloads/)

  Choose `Desktop development with C++`

- [SQLCipher Source Code](https://github.com/sqlcipher/sqlcipher/archive/refs/heads/master.zip)

## Steps

- `Makefile.msc` in folder `sqlcipher-master`

  - 1

    From

    `TCC = $(TCC) -DSQLITE_TEMP_STORE=1`

    To

    `TCC = $(TCC) -DSQLITE_TEMP_STORE=2 -DSQLITE_HAS_CODEC -I"C:\Program Files\OpenSSL-Win64\include"`

  - 2

    From

    ```
    # If ICU support is enabled, add the linker options for it.
    #
    !IF $(USE_ICU)!=0
    LTLIBPATHS = $(LTLIBPATHS) /LIBPATH:$(ICULIBDIR)
    LTLIBS = $(LTLIBS) $(LIBICU)
    !ENDIF
    # <</mark>>
    ```

    To

    ```
    # If ICU support is enabled, add the linker options for it.
    #
    !IF $(USE_ICU)!=0
    LTLIBPATHS = $(LTLIBPATHS) /LIBPATH:$(ICULIBDIR)
    LTLIBS = $(LTLIBS) $(LIBICU)
    !ENDIF
    # <</mark>>
    
    LTLIBPATHS = $(LTLIBPATHS) /LIBPATH:"C:\Program Files\OpenSSL-Win64\lib\VC\x64\MT"
    LTLIBS = $(LTLIBS) libcrypto.lib libssl.lib ws2_32.lib shell32.lib advapi32.lib gdi32.lib user32.lib crypt32.lib
    ```

- Open `x64 Native Tools Command Prompt for VS 2022`

  `cd` to `sqlcipher-master`

  execute `nmake /f Makefile.msc`

- Result:

  `sqlite3.exe` includes encryption feature

## Test

### Init

`sqlite3 test.db`

- `PRAGMA key='test123';`

- `create table demo (id text);`
- `.quit`

`test.db` needs key `test123`

### Reset Key

`PRAGMA rekey='test123';`

After using old key, execute upper statement, reset key to `test123`

