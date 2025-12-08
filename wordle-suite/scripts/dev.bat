@echo off
REM Quick reference script for common development tasks (Windows)

if "%1%"=="" goto help
if "%1%"=="help" goto help
if "%1%"=="init" goto init
if "%1%"=="dev" goto dev
if "%1%"=="test" goto test
if "%1%"=="build" goto build
if "%1%"=="lint" goto lint
if "%1%"=="check" goto check
if "%1%"=="clean" goto clean

:help
echo Wordle Suite - Development Commands (Windows)
echo.
echo Usage: dev.bat [command]
echo.
echo Commands:
echo   init    - Install dependencies
echo   dev     - Start development server
echo   test    - Run tests
echo   build   - Build for production
echo   lint    - Run linters
echo   check   - Run all checks
echo   clean   - Clean build artifacts
echo   help    - Show this help message
echo.
goto end

:init
echo Installing dependencies...
call npm install
echo ✓ Dependencies installed
goto end

:dev
echo Starting development server...
call npm run tauri-dev
goto end

:test
echo Running tests...
call npm test
goto end

:build
echo Building for production...
call npm run tauri-build
goto end

:lint
echo Running linters...
call npm run lint
cd src-tauri && cargo clippy && cd ..
goto end

:check
echo Running all checks...
call npm run type-check
call npm run lint
call npm test
cd src-tauri && cargo test && cd ..
goto end

:clean
echo Cleaning build artifacts...
rmdir /s /q dist
rmdir /s /q src-tauri\target
rmdir /s /q node_modules
goto end

:end
