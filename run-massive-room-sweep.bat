@echo off
setlocal
cd /d "C:\asolaria-acer\navier-stokes-regularity-spectral"

echo ==============================================================================
echo  ASOLARIA 3-ADIC NAVIER-STOKES MASSIVE ROOM SWEEP (HOST8 100G ROTOR)
echo  Runtime: Rust 1.81 (GNU/MSVC Parity) ^| 0 JSON ^| 0 Node ^| Integer Only
echo ==============================================================================

:: Default to full sector (100,000,000 rooms). Can pass custom args:
:: e.g. run-massive-room-sweep.bat --rooms 10000000
:: e.g. run-massive-room-sweep.bat --all-sectors
if "%~1"=="" (
    set ARGS=--full-sector
) else (
    set ARGS=%*
)

cargo +1.81.0-x86_64-pc-windows-gnu run --release --bin sweep_rooms -- %ARGS%
