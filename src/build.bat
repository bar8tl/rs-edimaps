echo off
rem build.bat - Script to start compiling of program EDIMAPS (2021-07-01 bar8tl)
cd c:\rbhome\rust\edimaps-rs
xcopy c:\cprogs\edimaps-rs\src        src                      /D /S /C /I /F /Y
xcopy c:\cprogs\edimaps-rs\Cargo.toml .                        /D /C /Y
cargo build
xcopy .\target\debug\edimaps.exe      c:\cprogs\edimaps-rs\bin /D /C /Y
pause
