@echo off
setlocal EnableDelayedExpansion

set "BUILD_TYPE=debug"
set "BIN_PATH="
set "CARGO_ARGS="

:parse_loop
if "%~1"=="" goto :done_parsing

if /i "%~1"=="--release" (
    set "BUILD_TYPE=release"
    shift
    goto :parse_loop
)

if /i "%~1"=="--debug" (
    set "BUILD_TYPE=debug"
    shift
    goto :parse_loop
)

if /i "%~1"=="-bin" (
    shift
    if "%~1"=="" (
        echo Error: -bin requires an executable path
        exit /b 1
    )
    call set "BIN_PATH=%%~1"
    shift
    goto :parse_loop
)

:: Parse args="..." for cargo build
set "CURRENT_ARG=%~1"
if /i "!CURRENT_ARG:~0,5!"=="args=" (
    set "ARG_VALUE=!CURRENT_ARG:~5!"

    :: Handle quoted values
    if "!ARG_VALUE:~0,1!"=="""" (
        set "ARG_VALUE=!ARG_VALUE:~1!"
        if "!ARG_VALUE:~-1!"=="""" (
            set "CARGO_ARGS=!ARG_VALUE:~0,-1!"
        ) else (
            shift
            :collect_args
            if "%~1"=="" goto :done_parsing
            set "PART=%~1"
            if "!PART:~-1!"=="""" (
                set "PART=!PART:~0,-1!"
                set "CARGO_ARGS=!ARG_VALUE! !PART!"
                shift
                goto :done_parsing
            ) else (
                set "ARG_VALUE=!ARG_VALUE! !PART!"
                shift
                goto :collect_args
            )
        )
    ) else (
        set "CARGO_ARGS=!ARG_VALUE!"
    )
    shift
    goto :parse_loop
)

shift
goto :parse_loop

:done_parsing

if "%BIN_PATH%"=="" (
    echo Usage: run.bat [--release^|--debug] -bin ^<path^> [args="^<cargo build args^>"]
    exit /b 1
)

:: Build command construction
set "BUILD_CMD=cargo build"
if "!BUILD_TYPE!"=="release" set "BUILD_CMD=!BUILD_CMD! --release"
if not "!CARGO_ARGS!"=="" set "BUILD_CMD=!BUILD_CMD! !CARGO_ARGS!"

echo Running: !BUILD_CMD!
!BUILD_CMD!

if !ERRORLEVEL! neq 0 (
    echo Build failed!
    exit /b 1
)

:: Resolve binary path
echo %BIN_PATH% | findstr /C:"\" >nul || echo %BIN_PATH% | findstr /C:"/" >nul || (
    set "BIN_PATH=target\!BUILD_TYPE!\%BIN_PATH%"
)

for %%F in ("%BIN_PATH%") do set "BIN_NAME=%%~nxF"

if not exist "%BIN_PATH%" (
    echo Error: Executable not found: %BIN_PATH%
    exit /b 1
)

echo.
echo Starting: %BIN_NAME%
start "Running: %BIN_NAME%" cmd /k "%BIN_PATH%"

endlocal