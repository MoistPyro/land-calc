cargo build
md %~dp0\windows
copy %~dp0\target\release\land-calc.exe %~dp0\windows\land-calc.exe
copy %~dp0\list.txt %~dp0\windows\list.txt
copy %~dp0\LICENSE %~dp0\windows\LICENCE