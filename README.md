# TermRect &emsp; [![Latest Version]][crates.io] [![Docs Badge]][docs.rs]

[Latest Version]: https://img.shields.io/crates/v/termrect.svg
[crates.io]: https://crates.io/crates/termrect
[Docs Badge]: https://docs.rs/termrect/badge.svg
[docs.rs]: https://docs.rs/termrect/

**TermRect is a cross-platform representation of a rectangle of characters on a terminal.**

---

# About

TermRect is a lightweight representation of a styled 2d character grid. You can
draw styled text into a TermRect, you can draw other TermRects into a TermRect.
TermRect keeps track of the regions that have changed in each line, so you can
render only what changed to the screen.


