Set-Location C:\Users\matth\repo\web\

if(($item = Get-Item -Path "C:\Users\matth\Downloads\html.htm" -ErrorAction SilentlyContinue)) {
    Remove-Item $item
}
if(($item = Get-Item -Path "C:\Users\matth\Downloads\html_files" -ErrorAction SilentlyContinue)) {
    Remove-item $item -Recurse
}

python web.py
Copy-Item "C:\Users\matth\Downloads\html.htm" -Destination "C:\Users\matth\repo\web\html"
Set-Location C:\Users\matth\repo\web\parse\
cargo run