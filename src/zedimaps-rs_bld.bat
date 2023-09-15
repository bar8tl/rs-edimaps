@echo off
xcopy _config.json ..\target\debug\ /D /C /Y
cd ..\
cargo build
pause
