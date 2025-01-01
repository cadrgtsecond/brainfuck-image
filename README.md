# `brainfuck_image`
Ever wanted to embed turing complete programs within ascii art? Well now you can thanks to brainfuck

This is a Rust program that can take in any arbitrary image, convert it into ascii art and then embed any arbitrary brainfuck program within said image.

## Installation
Install `cargo` and the Rust toolchain.
then
```
git clone https://github.com/cadrgtsecond/brainfuck_image
cd brainfuck_image
cargo install --path .
```

## Usage
```
brainfuck_image [OPTIONS] --program <PROGRAM> <IMAGE>

Arguments:
  <IMAGE>  The image to use

Options:
  -p, --program <PROGRAM>      The program to embed within the image
  -w, --width <WIDTH>          The width of the generated ascii art [default: 100]
  -t, --threshold <THRESHOLD>  The threshold for replacing characters. Higher means more characters will be replaced [default: 4]
  -h, --help                   Print help
```

## Example
![Cute image of a pug](tests/Pug_portrait.jpg.png)

```
$ brainfuck_image -p '++++++++++[>+++++++++++>++++++++++<<-]>++.+++++.>+++.' tests/Pug_portrait.jpg
+lllIlIl;lllllI;"^`'             `'^^"""^`^^`^"^^^"^^^"^^^"^^`^^``^````````^^``^^\0XcvcznruUCQJzvnjf
;;:;;I;;""":::;"^```'              ```^^":I+h#WM&%%%BB%%WMWB%%8oZ|++))}})+}++;`^+JJcnrxnunnxxxnxnrnn
^""^"";"""""::"^^^'` '     '  '' +a##WM#MW#O*#*pkoapo#WMh#Y(f/Ybh/|fj()1))tr\1\ruUzXvuvccnrjt\/trrf(
"";:;;;II;;;;":""^`'''   {aW%88WoodkkCcQpdpx/YYJYOLkkdOwbokdU1`+I[1Uq/)|>1{\ft/1+tjnzvcuvvzzr)}+++++
:";:;;II;l;I;::"""`'''')k#wmdaqowCOwLUxcmOL(uccYmowpmppmbkhhM##dv1l;})(X(1\||)){{(xXznuvzYXYcnxxjftt
^^"^""""^^""^^`"+\cXYYkkU{rCmbbhaohOxxvXnU)rXC0pLCpmuZYphpbaZ*W&&#Wa\I+1}1r)1|1l+/0Ynujxf\//trnnunvx
`'`'`^''' ''`+\uvxrxLLl;IU0OZhapmqQqzUvYuXXvXmXLq\LUq0bo#*M8&*a#&W&&##kj+>+r(+II+xufrxrjjjt\)\|tftfn
 '' '     +nn)1|\/xz+(+{ZJXYZnwZCLr/Ju/nv|xc\tv||pQLv;   ^++jjr0dMoqpOnj{+l:{|+1)(/(tfffvUUYzvxjnuun
` '````  ^|1<<{{1/f-])h#oCj\Zu>++fpvC{)\x|\}{}t?vx}}.  "/{  +  ":+l\)lYtt{+:"+f\(\|)|rrunczYUJLJUYzY
 ' ' ''   +\i:i~}n{}u#ohJkYn)lI.   >|+{:+I+.``;'`    'i( )_?l"; ^`)~~~rjunjt::"{f)/(_|fjfjncczYXzzXY
    '      :{?^"(1~~rQCu\l(l   `?;` ^ I?l^;}_fx1"';~Jn))c_:1^` \iilli1nuzXjrr;:I1)()_1tf/tjxuvvczJUc
    ' '   ' '`_I{?~~"i?`} '' `^l^ I1t/t`)~rz\}C\?|}`  ?;{f~?}i^`iI""_vrvOUXLzrrjt(11?_/f///jjrjnrjjf
 '''`'^``'`''''`l(_l'~'I:;? ^?~ /{()l'';;(``{iL   YLiiI'I'I_\` |i:?i))|rYqwQZkU?ll__{l}|((|())|)(\\/
```````````^```^^)i:`' :__^'`   }}?II'`I}txt`fl'^t)j)i{ `'l|I;1I"::~?fzvurxQ*#Mh|II??__{))))(1})(1?~
^"^""^"^^^"`^^^^`iI'''':\|} ''`' l_~^~)"|)}zrYl~/v?t;?|;_"i?//(i|CJxfC\?\OdbMW&&ozI_~ii(){{{{__~iiii
"""^"""^^^^```^`:^^^`'`"|f/(l^`~'//|"?i_l}l'^: :"^";1 ~_":}1?}Ii_i"~rInwpo#MW&&MdOI;iil1){}{}}?~~iii
:^"""""::^^^`^^^`"^^``'  i/(t{` `?~_I"I"l^ '`:l'`:`'  ';l\li?_t;"" 'luk*M#M&WMa#MWMW&&&oO/}}?_?_~_~~
:"":""":"^^^````^^`^`' ' ;tun)I{):ll^^:^ '"`^l:l ^^:":`''  `;''   "lCbho##*#*W##&8&W&&WW&ot?~~_?__~~
:":::""""^^^`^^`^^^`^'`  "|zXvji`"` '"" '  `iI;l' '' `"''': ''l}jcwkhaa*dkkkak*WMMWMWW&&8&&aC{~__~~_
:"":"""""^^^````````'' '  ~xUYujf^       ' '  '       ^?1\/jnnYv0ULdw0Umhkodo**0#k*W&8%%8&W#hjl_i~~{
"""""":""^"^"``"`^``''     ;tJUnnvvj\)}(_}_{{}jj\j/tufj{r\jrnxr1r\|j\wa#M&#oko#kkoMW&8%%%%8&W##k01}|
"^"""^""^""^^^^^``````'      `~/XX/(unnuufjtrjvcrxx/f)_)(l}~\nLdbhohkb#Mb**MM8&ak#W&8&W&W&WoM##**o#M
^`^`^^^^^^^`"``^^`^^`""`;"^`^?CCzcvccczLQ0QXQUczUCXcQcmmwwdaa*#o##Mo#hwW&#W88M8**WW&&W&W&&&WW&WW&W&W
$ brainfuck_image -p '++++++++++[>+++++++++++>++++++++++<<-]>++.+++++.>+++.' tests/image.png | bff
pug
```

## TODO
- [ ] Improve quality of generated images
- [ ] Support generating brainfuck programs with messages
