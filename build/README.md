# Icon Instructions

## On macOS

- Change `macos/icon_1024x1024.png` to be the icon image you want it to be
- Run `macos/create_icns.sh` to generate all the mac icon versions
  - `cd macos && ./create_icns.sh`
- Open `macos/AppIcon.iconset/icon_256x256.png` in `Preview.app`
- `Preview > Export` menu item
- Hold `Option` while clicking on the `Format` drop-down so that the `Microsoft Icon` option appears
- Save as `windows/icon.ico`
