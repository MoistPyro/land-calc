rem build the program with release optimisation (remove -r for faster build, the bottleneck is always responses from api.scryfall anyway)
cargo build
rem creates a folder called windows, and copies all the files recuired to run the app into it.
md %~dp0\windows
copy %~dp0\target\release\land-calc.exe %~dp0\windows\land-calc.exe
echo 99 mountain (SLD)>%~dp0\windows\list.txt
copy %~dp0\LICENSE %~dp0\windows\LICENCE
copy %~dp0\README.md %~dp0\windows\README.md