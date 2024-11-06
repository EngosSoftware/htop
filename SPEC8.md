# Specification for [\#8](https://github.com/EngosSoftware/htop/issues/8)

The `Tab` has a function named `capture_screenshot` that is supposed to return the requested image type of the page.

The supported image formats are: PNG, JPG, WebP.

New options (for generating an image):

- `   --jpg` exports image to JPG format,
- `   --png` exports image to PNG format,
- `   --webp` exports image to WebP format,
- `   --window-size=int,int` sets the window size of the headless Chrome (optional),

Shared options (with PDF printing):

- `-v --verbose` same behavior as for PDF printing,
- `   --no-crash-reports` same behavior as for PDF printing,
- `   --log-level` same behavior as for PDF printing,
- `-t --timeout` same behavior as for PDF printing,

Conflicting options (from PDF printing):

- `-b, --print-background`
- `    --print-header-footer`
- `    --header`
- `    --header-file`
- `    --footer`
- `    --footer-file`
- `-l, --landscape`
- `-m, --margin`
- `-p, --paper-format`
- `    --paper-width`
- `    --paper-height`
- `    --paper-size`
- `-r, --ranges`
- `-s, --scale`
