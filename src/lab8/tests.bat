@echo off
REM --- Settings ---
SET BASE_URL=http://127.0.0.1:8080
SET CONCURRENCY=100
SET REQUESTS_FAST=50000
SET REQUESTS_NORMAL=10000
SET REQUESTS_SLOW=1000
REM !!! ВИКОРИСТОВУЄМО ПОВНИЙ ШЛЯХ, ЯКИЙ ВИ ВКАЗАЛИ !!!
SET AB_PATH=D:\RustRover 2024.3.6\projects\src\lab8\Apache24\bin\ab.exe

echo === Starting performance test (ab) ===
echo.

REM --- Scenario 1 ---
echo --- Clearing cache ---
curl -s %BASE_URL%/api/clear-cache
echo. & timeout /T 1 > NUL
echo --- Running: Scenario 1 (Cache miss, no delay) ---
"%AB_PATH%" -n %REQUESTS_NORMAL% -c %CONCURRENCY% -k "%BASE_URL%/api/data?id=1&query=test_no_delay"
IF ERRORLEVEL 1 ( echo *AB FAILED* & pause & exit /b %ERRORLEVEL% ) ELSE ( echo --- Finished: Scenario 1 --- )
echo. & timeout /T 2 > NUL

REM --- Scenario 2 (Do not clear cache!) ---
echo --- Running: Scenario 2 (Cache hit, no delay) ---
"%AB_PATH%" -n %REQUESTS_FAST% -c %CONCURRENCY% -k "%BASE_URL%/api/data?id=1&query=test_no_delay"
IF ERRORLEVEL 1 ( echo *AB FAILED* & pause & exit /b %ERRORLEVEL% ) ELSE ( echo --- Finished: Scenario 2 --- )
echo. & timeout /T 2 > NUL

REM --- Scenario 3 ---
echo --- Clearing cache ---
curl -s %BASE_URL%/api/clear-cache
echo. & timeout /T 1 > NUL
echo --- Running: Scenario 3 (Cache miss, with 50ms delay) ---
"%AB_PATH%" -n %REQUESTS_SLOW% -c %CONCURRENCY% -k "%BASE_URL%/api/data?id=2&query=test_with_delay&delay=50"
IF ERRORLEVEL 1 ( echo *AB FAILED* & pause & exit /b %ERRORLEVEL% ) ELSE ( echo --- Finished: Scenario 3 --- )
echo. & timeout /T 2 > NUL

REM --- Scenario 4 (Do not clear cache!) ---
echo --- Running: Scenario 4 (Cache hit, after delay) ---
"%AB_PATH%" -n %REQUESTS_FAST% -c %CONCURRENCY% -k "%BASE_URL%/api/data?id=2&query=test_with_delay&delay=50"
IF ERRORLEVEL 1 ( echo *AB FAILED* & pause & exit /b %ERRORLEVEL% ) ELSE ( echo --- Finished: Scenario 4 --- )
echo. & timeout /T 2 > NUL

echo === Testing finished ===
pause