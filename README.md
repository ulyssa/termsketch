# termsketch

[![Build Status](https://github.com/ulyssa/termsketch/workflows/CI/badge.svg)](https://github.com/ulyssa/termsketch/actions?query=workflow%3ACI+)
[![License: Apache 2.0](https://img.shields.io/crates/l/termsketch.svg?logo=apache)](https://crates.io/crates/termsketch)
[![Latest Version](https://img.shields.io/crates/v/termsketch.svg?logo=rust)](https://crates.io/crates/termsketch)
[![Docs Status](https://docs.rs/termsketch/badge.svg)](https://docs.rs/crate/termsketch/)

## About

This is a tool for converting images to text. It supports two different conversion modes:

- `grayscale`, a typical conversion onto a map of light to dark characters to produce a grayscale image.
- `outline`, a conversion onto visually similar characters done region-by-region, to help produce output that makes better use of negative space than the grayscale conversion.

## Examples

You can see the visual difference between the two modes on this [image][witch-hat]:

```
% termsketch outline ./witch-hat-2.png






                                           ⠃⠁⠁⠁⠇
                                          ⠃      ⠁
                                         ⠃    ⡀⡄⡄⡄⡀⡁
                                        ⠃    ⡂
                                       ⠃     ⠅
                                      ⠇      ⠁
                                     ⡇        ⡃
                                    ⡃         ⠁
                                   ⡇           ⡃
                                   ⠃
                                  ⡇             ⠁
                                 ⡇               ⠃
                                 ⠁
                                ⡇                 ⠁
                               ⡇                   ⠁
                               ⠃                    ⡃
                              ⡇                                ⡇⠇⠃⡇⠃⠃⠃⠇
                             ⡇                      ⡀⡁    ⠇⠃⠁⠃
                     ⠇⠃⠃⠇⠁⠁  ⠇                   ⡀⡄⡅  ⠃⠁⠁               ⡅
                ⠇⠃⠁⠇      ⡂ ⡇                 ⡀⡄⡅     ⡇⡆⡄⡀⡀           ⡀⡆
             ⠃⠁⠁            ⠁               ⡀⡆              ⡆⡆⡄⡇⡄⡇⡄⡆⡆
           ⠃⠁            ⡂ ⡇             ⡀⡄
         ⠃⠁             ⡀  ⡄⡀⡆⡀⡀⡀⡀⡀⡀⡀⡀⡄⡆
        ⡃                ⠁⠇⠃⠃⠇⠇⠅
       ⡇                         ⡀⡆
       ⡇                     ⡀⡄⡆
        ⡆               ⡀⡀⡄⡆
          ⡆⡄⡀⡆⡆⡆⡆⡀⡄⡄⡄⡆







% termsketch outline --charset="/\\|" ./witch-hat-2.png






                                           /|||/
                                          \      \
                                         \    ||||\|
                                        \    \
                                       \     /
                                      /      /
                                     \        |
                                    /         /
                                   /           \
                                   \
                                  \             /
                                 /               |
                                 \
                                \                 /
                               /                   |
                               \                    \
                              \                                ||||||||
                             /                      \|    |/|\
                     /|||||  /                   \||  //\               /
                /||\      \ /                 \||     ||||            \|
             ||          \                  |\              \|||||||\
           |             | \             ||
         /              \  \||||    |||\
        \               /|||||||
       /                         ||
       |                     |||
        |               |||\
          |||||||||||||






% termsketch grayscale ./witch-hat-2.png






                                           .cl;.
                                          :XMMMXx'
                                         oMMM0:..'.
                                        dMMMW.
                                       oMMMMW.
                                      cMMMMMM:
                                     ,WMMMMMM0
                                    .XMMMMMMMM:
                                    OMMMMMMMMMX.
                                   lMMMMMMMMMMMx
                                  'WMMMMMMMMMMMW;
                                  KMMMMMMMMMMMMMX.
                                 dMMMMMMMMMMMMMMMk
                                ,WMMMMMMMMMMMMMMMMc
                                KMMMMMMMMMMMMMMMMMN'
                               oMMMMMMMMMMMMMMMMMMM0
                              'WMMMMMMMMMMMMMMMMMMMMd           ...'...
                              OMMMMMMMMMMMMMMMMMMMMXd     .,lk0XNWMMMWNd
                      .';c'  cMMMMMMMMMMMMMMMMMMXd,    ,xKWMMMMMMMMMMMM0
                .'cx0NWMMN. .NMMMMMMMMMMMMMMMNx,       .;oOKNWMMMMWWN0o.
             .cONMMMMMMMMl  xMMMMMMMMMMMMMWO:.              ...'''..
           ,kNMMMMMMMMMMK  ,WMMMMMMMMMMW0l.
         .kMMMMMMMMMMMMM;  'loxkkOOkdl;.
        'XMMMMMMMMMMMMMMk:,'....
        0MMMMMMMMMMMMMMMMMMMWWX0o'
        XMMMMMMMMMMMMMMMMMMNOo,.
        ;KWMMMMMMMMMMWX0d:'.
          .,cloolc:,..
```

## License

`termsketch` is released under the [Apache License, Version 2.0][APLV2].

[APLV2]: https://github.com/ulyssa/iamb/blob/master/LICENSE
[witch-hat]: https://openclipart.org/detail/245968/wizards-white-hat
