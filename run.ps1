Set-Location C:\Users\matth\repo\web\

if(($item = Get-Item -Path "C:\Users\matth\Downloads\html.htm" -ErrorAction SilentlyContinue)) {
    Remove-Item $item
}
if(($item = Get-Item -Path "C:\Users\matth\Downloads\html_files" -ErrorAction SilentlyContinue)) {
    Remove-item $item -Recurse
}
if(($item = Get-Item -Path "C:\Users\matth\Downloads\overview.htm" -ErrorAction SilentlyContinue)) {
    Remove-Item $item
}
if(($item = Get-Item -Path "C:\Users\matth\Downloads\overview_files" -ErrorAction SilentlyContinue)) {
    Remove-item $item -Recurse
}

python web.py
Copy-Item "C:\Users\matth\Downloads\html.htm" -Destination "C:\Users\matth\repo\web\parse\html"
Copy-Item "C:\Users\matth\Downloads\overview.htm" -Destination "C:\Users\matth\repo\web\parse\overview"
Set-Location C:\Users\matth\repo\web\parse\
cargo run
cd ..
python spread.py