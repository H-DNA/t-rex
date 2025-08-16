# t-rex
A rusty text editor that works decently with Unicode.

## Learning points
Here's a list of learning points I have obtained from doing this projects & reading the references.

### Unicode
- Character: Quite an overloaded term. It's the most basic unit of text that has a predefined semantic and usage. A character is not bound to any shape, color, size, form but can have a typical presentation of those.
- Character code: A way to index all the characters.
- Code point: An index of a character
- Character property: Unicode assigns properties to every characters.
  - Character width property
- Grapheme cluster: A unit of text that is visually perceived as one character by the user. For example: `á`, `ê`, `🧑🏿‍🤝‍🧑🏿`.
- Character encoding (charset): A way to map a character to a sequence of octets.
- Character repertoire: A set of character.
- Glyph: A visual image of a character.
- Font: A set of glyphs.
- Typeface: The general style of a font.

### Terminal
- ANSI/VT100
- Alternate screen
- History of bell, carriage return and newline

### Color models
- Additive color model & RGB & Why screens use RGB
- Subtractive color model & CYMK & Why prints use CYMK

## References
- [hecto](https://github.com/pflenker/hecto-tutorial): Binge-reading resource & Redesign and redo from scratch. Great resource on the nitpicks of handling Unicode texts.
- [Unicode explained](https://books.google.com.vn/books/about/Unicode_Explained.html?id=lxndiWaFMvMC&source=kp_cover&redir_esc=y): Binge-reading stuffs on some foundational terms related to Unicode.
