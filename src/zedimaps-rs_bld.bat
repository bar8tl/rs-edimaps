@echo off
xcopy edimaps-cfg.json ..\target\debug\ /D /C /Y
cd ..\
cargo build
pause
