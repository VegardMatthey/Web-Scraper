cd ~\repo\web\
python web.py
rm ~\repo\web\parse\data\*
mv ~\Downloads\html* ~\repo\web\parse\data\
mv ~\Downloads\overview* ~\repo\web\parse\data\
cd ~\repo\web\parse\
cargo run --release -q
cd ..
python spread.py
rm ~\repo\web\parse\data\*
