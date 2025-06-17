@echo off
echo Installing FlowLang for Windows...

if exist "bin\flowlang-windows-x86_64.exe" (
    echo Found Windows binary
    copy "bin\flowlang-windows-x86_64.exe" "%USERPROFILE%\flowlang.exe"
    echo FlowLang installed to %USERPROFILE%\flowlang.exe
    echo.
    echo Add %USERPROFILE% to your PATH to use 'flowlang' command globally
    echo Or run: %USERPROFILE%\flowlang.exe
) else (
    echo Error: Windows binary not found
    echo Available files:
    dir bin
)

pause
