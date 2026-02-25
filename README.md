# BarChart

This displays a BarChart in a Ratatui application

This widget is directly lifted from Ratatui, refactored as its own standalone widget, and augmented with additional features (see `Extra Features`)

### Usage:

<sub>cargo:</sub>
```
cargo add tui-barchart-ext
```
<sub>Import:</sub>
```
use tui_barchart_ext::barchart::{Bar, BarChart};
```
<sub>Instantiate:</sub>
```
let chart = BarChart::horizontal(bars).bar_width(3);
```

### Extra Features:
 - Inverted bars (top to bottom, right to left)
 - Nine level horizontal bars

details forthcoming
