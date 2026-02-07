### BarChart

This displays a BarChart in a Ratatui application

This widget is directly lifted from Ratatui, made to be its own standalone widget, and with some of my own additions.

Usage:

Cargo.toml
```
tui-barchart-ext = { git = "https://github.com/notkaj/tui-barchart-ext.git", version = "0.1.0" }
```
```
use tui_barchart_ext::barchart::{Bar, BarChart};
```
```
let chart = BarChart::horizontal(bars).bar_width(3);
```
details forthcoming


