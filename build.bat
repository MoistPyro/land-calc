rem build the program with release optimisation (remove -r for faster build, the bottleneck is always responses from api.scryfall anyway)
cargo build
rem creates a folder called windows
md %~dp0\windows
rem copies the executable into the new folder
copy %~dp0\target\release\land-calc.exe %~dp0\windows\land-calc.exe
rem makes a default list.txt with 99 mountains
echo 99 mountain (SLD)>%~dp0\windows\list.txt
rem copies over the license
copy %~dp0\LICENSE %~dp0\windows\LICENCE
rem copies over the readme
copy %~dp0\README.md %~dp0\windows\README.md