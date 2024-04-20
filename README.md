# `brainfuck_image`
Ever wanted to embed turing complete programs within ascii art? Well now you can thanks to brainfuck

This is a Rust program that can take in any arbitrary image, convert it into ascii art and then embed any arbitrary brainfuck program within said image.

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

                                                           }  +h:ZlZ&Ct|ff
                                           8+ }YIICJ }//Yk/&Qqqh{/LJn)+0*1n{ {
                                         '+;k^%h  o :U+C"nC/ |c; Z/@t+v)v ZYx0'/z"Q
                                       ZQW/  'M&1axZ}L%/Cn"&}+@Z+x+Wk:CfvzZv/1 Q" 0
                                    a +0/'&[1B>{:LL0%0ZC:l +ZY/&QLklv+/Lk}@"/|
                                    pp\xC^ &{+0COXOh%kd'fd'pdn dnf"0x&*00cLflv
                                          %}|dB0{ZpapaX"Z1QxkvZd"1x%zxX'{rJYnt
                                           } %:/**pOM1O+v*n\lxL  vXxQY/0YhOY/vB
                                          C@anxl{ dOv{ +O"'1^lC%CaX+Y^CCX"Y1q|
                                          Mo+UlL Z+/X++p+*Zk>ad+/+d^d'^ aQ0r1
                                        I}1tIk+^n/nd+aQd UdntCCC&+X%p/0Cp'|MY
                                       |/%'hkBr1bfzxa%+k+"&r"aqlQl0C/*C@Z/rZ:+
                                       m+@h  r&Q8@":L1r"t|+Z/Z \<<*-|lk#/|] d )>
                                      tMZvZ" x1 1Z++aacar.h'"+h;1Umkqp+mWYf@88)Jo
                                     \rq/ClCJ||/1ra&Zr|\1|/ +LB qLYYJ)/rZYYZ/c)z:
                                    XYdI%tCCM#1||C;/z Z}\& 1W&@""vp:@)Y+/M/wcnd@x
                                    r'YL /aka|kJJ+W.tttd;Mr|&&J h>zJ+: CdJwC:rC:1
                                   rJ1U:%;%{+UUYl;)r) v)@pmdmZk#YrMMLCz:wl1+)/wlv.
                                  pZ:x&akzIWlq ;oaq dkq/ C #/Jk/ ) p_::1Zf:YiZ rm
                                 \l/|1UarrrrkW):i~YC~r/h1_fkrZCddqJzCMJ@vn/Zw rZ
                                  k}; kUa&/ Z|CqW|WW)"&/M1p &pd8M_|@dC r~8jv8Yi
                              L/rZr%Wq"{/1k))1i@orWqddqWIppC/CUt%xmtr  t/@_/fl
                           _m%%UzYraq|rUrw f "th|qt;dv@qrq &)1//fZW)Cx"vZw"ldvd
                         tWom /r ;LYk|rpq#zd)i~kaWU|  )CCmCpL :ZCCw Bad_  J&pnM^
                         MMMMtrr;%'ZUtola ~c &~Ca/@Wd l| @ )__ZvhldthBaWk:~ncwYf
                      xJ ZW@ @WooZ: ZrUMqLCY8rc/mtpqZ"/lZ /@Z|h LZ_/m dY_vfv/@iZ
                      q)JJL"plo}ZCI"n %Zavwfcpp&&  Zq 1Z"w@ &kp_lkdWdw)MJ"Y~&i#^
                     h/W/f#rZdWaZd?rzfhza@ZJ~/@)|W"WZwW ZW|kv |BW/"aCCd|MWz~ri#
                      1aq)r w @da""ZZ"akk1ttvzY@%Y&dUkarz&8n/?L&k C"|iY/ZC~Yc#q
                       q/Cwq_|kmdkz://t1a}%aCJ#CZO1"jU/'L~B1Zmh@kw""/r#Z@hvzwZd
                        %{ /M qprrUU\||nb/k)qhMYCLb ?~d'Laathq"LpZ q@_Wk d1CYr/
                        {8CZZ"1Ira1/C "Yf;IvCllJCbB%1L;z&/a/hWdlCmZaq_J# d"wwZ#@
                    `1}o/aUZOb&{:} Y{;dib&::#lCMCMt C@l r@dk@|*h%Urtr%MU81WdWdC/#CX
                  @rm@/^~/"fki rMbhZZ"/xlqCMB CdvCmZ rUU;r ;    Y%z OriL\Y|&C{ #q)_Lhr
                                      nq&d&UOW?@0_d                         qc;UfzW#C&zI
$ brainfuck_image -p '++++++++++[>+++++++++++>++++++++++<<-]>++.+++++.>+++.' tests/image.png | bff
pug
```
