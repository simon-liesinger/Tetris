this is tetris. it is currently fucntional, but not pretty.
to compile to a mac executable, run these commands (in the Tetris directory):
```
cargo build --release

mkdir -p Tetris.app/Contents/MacOS

cp target/release/Tetris Tetris.app/Contents/MacOS/

cat <<EOF > Tetris.app/Contents/Info.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Tetris</string>
    <key>CFBundleDisplayName</key>
    <string>Tetris</string>
    <key>CFBundleExecutable</key>
    <string>Tetris</string>
    <key>CFBundleIdentifier</key>
    <string>com.liesinger.simon.Tetris</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
</dict>
</plist>
EOF

hdiutil create -volname Tetris -srcfolder Tetris.app -ov -format UDZO Tetris.dmg
```
then open the .dmg to get the app, move it to applications, and just CMD-Space Tetris
