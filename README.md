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
![Cute image of a pug](tests/image.png)

```
$ brainfuck_image -p '++++++++++[>+++++++++++>++++++++++<<-]>++.+++++.>+++.' tests/image.png
                                                                                                         
                                                           ^  ;++l/zv/\U00                               
                                           tYuxxfX+ItxjvqQOmdaaLuvc+;:{qhb#*Y0                           
                                         JZCrk#L1ufYnuOqQUq*0U0CYLJ)\l:::+LbkhpOmQU                      
                                       wdwda0h|)rjZ\xcjJqUCvxXm\OkOt1+++:ll:(C CC0Z                      
                                    L0OqdhZarv/unpph#Zz)n/YawbdvdpMk*#*pQx){+)                           
                                    Zpbkdk ruzhbooo+#Map0mhpmwamw0hqkohqq0[>++                           
                                          jxUcwZ*kp#*LohMbW\r*L#hbZ#Qkohu:Il;\                           
                                          /xYjnOhh*oWboa*h#bkkppa*okdqOqqLXx(}:                          
                                          ;f/Ukk*aao**aaohhbkkd#b#oabkbbohqn(}                           
                                          |fznkpaka*oaaoahMMM#W#*#akahkp#dZf}+                           
                                        fxn\fQXkw*##M#WW0n|U\)))m#o#pdqbpu}+l                            
                                       ((juLQvfnJ0Qk###MXn)Un/(kdkhqdhX)l;:l+I                           
                                       |{)(|xUOdcfnncCQQUUzwJz1baahY(+1++({+I":l                         
                                      \|+}+{1\rxnzXOLL0LQCLpQz(tru/1)){|{>+;"":I"                        
                                     )f(+++++}((rfjvzUULCUJYzcvx)+llI:;+}lll;;:lI                        
                                    )xcXj\++<<-]()tjruxxcvxrt)){{})>;:lI++;I;;I;l.                       
                                    /uxcYvjr/(1++{{|\\\|t|/())+++IlIIi~_IIIii_~i}.                       
                                   f?rrnjtjuYnnx/t{>{1}{))|1|}1+l++i?ili}?}{{?}?}.                       
                                  |xn\rjrrft/)ttf/)||1)?~i~~??1(|{1){ii}}iill_~_I                        
                                 l/v(}njf/f/1{{ilIl_I_;~}{i1_})||(?liiI;:;;lI~__                         
                                 |rxtx1n/)?~}}~){}{{{{)(|})|v)I"i{()1_~_I"`^"ll                          
                              }Jt\tjt(nuvr1{{nXffft(||)tf)))()r\j\|\f||\(f{?i?                           
                           {/jjuQxf/(}frf}~i~{\(()\t|})(_(1){}??il{{/\{}}}{?I:_                          
                         _tf/xjfxCcxr(/))~lI:lI1/vu(|1{))|/)?1ili_}1vj|{~~I:";il                         
                        1||||\//tjuzr\f//~I;":I~jv){11?}1)|{{{_}~?1\(v/{1iI;;Ili                         
                      \?1}{)1)tffxnxxfr|)?il":;;|\)(\{??}1()}}~~?_{(|t1l{}i:;;l_                         
                     1){???{)/fxzXXCUYjx/}}i;"":)|1})1}}{})1)1){?1|t|}{iI{lI:l~;                         
                     (({?i~_}1tjxcYUQ0Lrj)_?I;;:({{{}}{1}{}1}1}vvvn/))|}i{lI:l~                          
                      \/({_~}1)|/nnzzCLQrr\\}ll;jxvcrrj/rrcUvYcrr|){}ll(})Il;~?                          
                       :()}){}1||rrnvJUCLxjj)?~izXCCYnjuczvnx/()1}{{?_~_f(}lI_}                          
                        _ut(|1))/frucUUUJjr{)~ilicJYYzcucj/\(){?)}1(){{1 |}_l_?                          
                        :cXzznnf//rvXYCQ0tf}___?~Jvjnctrrj/(({|///\/){?~ 1{}}}~i                         
                    1rxfjjrzXJvuJxxxuvcXJ)ii~__i~i\|/f/t/)11)})ljrt\/j|ucr{1{1_?~i}                      
                  f//fJkzjQ0rX0QxJLzLQOZ_)_iw1~|}_|\1/nut/1l    }jruXUXccx(r/u~~Y{{?(_                   
                                      _(OarnXtOmh{c                         Y0Cu0mw~)vrX                 
$ brainfuck_image -p '++++++++++[>+++++++++++>++++++++++<<-]>++.+++++.>+++.' tests/image.png | bff
pug
```

## TODO
- [ ] Improve quality of generated images
- [ ] Support generating brainfuck programs with messages
