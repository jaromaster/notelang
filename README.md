# Notelang
## simple note taking language that compiles to html

## Usage
---
`notelang myfile.ntlang` <br>
(generates `out.html`)

## Basic tags
---

### normal text
`.p This is a normal line of text.`

### headlines
`.h Some headline`

### bold text
`some * bold * text`

### italic text
`some # italic # text`

### marked text
`some _ marked _ text`

### images
`.img Image.png`

### hyperlinks
`.link https://some-other/page.html`

### tables
```
.tab
.hrow ID,NAME,AGE
.row 1,John,10
.row 2,Peter,35
.row 3,Toni,39
.tab
```