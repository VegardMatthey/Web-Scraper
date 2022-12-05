Set-Location C:\Users\matth\repo\web\
python web.py
rm C:\Users\matth\repo\web\parse\data\*
mv C:\Users\matth\Downloads\html* C:\Users\matth\repo\web\parse\data\
mv C:\Users\matth\Downloads\overview* C:\Users\matth\repo\web\parse\data\
cd C:\Users\matth\repo\web\parse\
cargo run -q
cd ..
python spread.py
rm C:\Users\matth\repo\web\parse\data\*
