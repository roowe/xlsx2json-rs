@echo off
setlocal EnableDelayedExpansion

%~dp00-xlsx2json.exe -i . -c 0-config.toml --pretty


pause
exit

 
