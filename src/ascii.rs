use colored::*;

pub fn grape() {
    println!("{}", r#"
      |
    \|/|/
  \|\\|//|/
   \|\|/|/
    \\|//
     \|/
     \|/
      |
_\|/__|_\|/____\|/_"#.yellow());
}

pub fn land() {
    println!("{}", r#"          |
        \ _ /
      -= (_) =-
        /   \         _\/_
          |           //o\  _\/_
   _____ _ __ __ ____ _ | __/o\\ _
 =-=-_-__=_-= _=_=-=_,-'|"'""-|-,_
  =- _=-=- -_=-=_,-"          |
jgs =- =- -=.--""#.yellow());
}

pub fn clock() {
    println!("{}", r#"         .--.
    .-._;.--.;_.-.
   (_.'_..--.._'._)
    /.' . 60 . '.\
   // .      / . \\
  |; .      /   . |;
  ||45    ()    15||
  |; .          . |;
   \\ .        . //
    \'._' 30 '_.'/
jgs  '-._'--'_.-'
         `""` "#.red());
}

pub fn no_city() {
  println!(r#"
        
~~~~~~~~"#);
}

pub fn tiny_city() {
  println!(r#"
    /.\    
~~~~|.|~~~~"#);
}

pub fn small_city() {
  println!(r#"
             _ 
   ./.\-/.\-|.|
~~~~|.|_|.|.|.|~~~~"#);
}

pub fn med_city() {
  println!(r#"
            |   _   _
      . | . x .|.|-|.|
   |\ ./.\-/.\-|.|.|.|
~~~|.|_|.|_|.|.|.|_|.|~~~"#);
}

pub fn large_city() {
  println!(r#"
                               ___        ___       T__
                    ________   | |~~~~~~~~| ||~~~~| |||
__|~~~~~~~~|   _/\_ |^^^^^^|  _| |--------| ||    | |##
|_|HHHHHHHH|  _|--| |------|_-#########################"#);
}

pub fn huge_city() {
  println!(r#"
                                    +              #####
                                   / \
 _____        _____     __________/ o \/\_________      _________
|o o o|_______|    |___|               | | # # #  |____|o o o o  | /\
|o o o|  * * *|: ::|. .|               |o| # # #  |. . |o o o o  |//\\
|o o o|* * *  |::  |. .| []  []  []  []|o| # # #  |. . |o o o o  |((|))
|o o o|**  ** |:  :|. .| []  []  []    |o| # # #  |. . |o o o o  |((|))
|_[]__|__[]___|_||_|__<|____________;;_|_|___/\___|_.|_|____[]___|  |"#);
}

pub fn massive_city() {
  println!("{}", r#"
                       .|
                       | |
                       |'|            ._____
               ___    |  |            |.   |' .---"|
       _    .-'   '-. |  |     .--'|  ||   | _|    |
    .-'|  _.|  |    ||   '-__  |   |  |    ||      |
    |' | |.    |    ||       | |   |  |    ||      |
 ___|  '-'     '    ""       '-'   '-.'    '`      |____
jgs~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"#.bright_blue());
}

pub fn barbarians() {
  println!("{}", r#"
      __      _
     /__\__  //
    //_____\///
   _| /-_-\)|/_
  (___\ _ //___\
  (  |\\_/// * \\
   \_| \_((*   *))
   ( |__|_\\  *//
   (o/  _  \_*_/
   //\__|__/\
  // |  | |  |
 //  _\ | |___)
//  (___|"#.purple());
}

pub fn population_boom() {
  println!("{}", r#"
   ,=""=,
  c , _,{{
  /\  @ )                 __
 /  ^~~^\          <=.,__/ '}}=
(_/ ,, ,,)          \_ _>_/~
 ~\_(/-\)'-,_,_,_,-'(_)-(_)  "#.bright_magenta());
}

pub fn disease() {
  println!("{}", r#"
      _____|~~\_____      _____________
  _-~               \    |    \
  _-    | )      /   |__/   \   \
  _-         )  |    |  |     \  \
  _-    | )      \   |--|      |  |
 __-_______________ /__/_______|  |_________
(                |----         |  |
 `---------------'--\\\\      .`--'
                              `||||"#.magenta());
}

pub fn just_rule() {
  println!("{}", r#"
           .--------.
          (          )
           ))_.--._((
          ((        ))
         *            *
        .'._.-=''=-._.'.
       (0'            'O)
        ||   . () .   ||
        || (\:\/\/:/) ||
        || / (o)(o) \ ||
        || \()(..)()/ ||
        '_..-('--')-.._'
        /'--..))((..--'\
       |  )   \  /   (  |
        \ '-._(*.)_.-' /
       //'-._(=||=)_.-'\\
      //  /|/ (#@) \|\  \\
     //  //'.      .'\\  \\
    (O| /_|/|  ||  |\|_\ |O)
     || |   |..||..|   | ||
  ^^ || |___. (  ) .___| || ^^
 (OO)||/   .'  )(  '.   \||(OO)
  || //   .o0O'  'O0o.   \\ ||
  || |'------------------'| ||
  || |[<>][<>][<>][<>][<>]| ||
 |__|'===================='|__|
 /                             \
/                               \
============================LGB=="#.yellow());
}

pub fn death() {
  println!("{}", r#"
              ...                            
             ;::::;                           
           ;::::; :;                          
         ;:::::'   :;                         
        ;:::::;     ;.                        
       ,:::::'       ;           OOO\         
       ::::::;       ;          OOOOO\        
       ;:::::;       ;         OOOOOOOO       
      ,;::::::;     ;'         / OOOOOOO      
    ;:::::::::`. ,,,;.        /  / DOOOOOO    
  .';:::::::::::::::::;,     /  /     DOOOO   
 ,::::::;::::::;;;;::::;,   /  /        DOOO  
;`::::::`'::::::;;;::::: ,#/  /          DOOO 
:`:::::::`;::::::;;::: ;::#  /            DOOO
::`:::::::`;:::::::: ;::::# /              DOO
`:`:::::::`;:::::: ;::::::#/               DOO
 :::`:::::::`;; ;:::::::::##                OO
 ::::`:::::::`;::::::::;:::#                OO
 `:::::`::::::::::::;'`:;::#                O 
  `:::::`::::::::;' /  / `:#                  
   ::::::`:::::;'  /  /   `#              "#.bright_black());
}

pub fn good_harvest() {
  println!("{}", r#"
        ,,,                      ,,,
       {{{{{{}}}}    ,,,             {{{{{{}}}}    ,,,
    ,,, ~Y~    {{{{{{}}}},,,      ,,, ~Y~    {{{{{{}}}},,, 
   {{{{}}}}}} |/,,,  ~Y~{{{{}}}}}}    {{{{}}}}}} |/,,,  ~Y~{{{{}}}}}}
    ~Y~ \|{{{{}}}}}}/\|/ ~Y~  ,,, ~Y~ \|{{{{}}}}}}/\|/ ~Y~  ,,,
    \|/ \|/~Y~  \|,,,|/ {{{{}}}}}}\|/ \|/~Y~  \|,,,|/ {{{{}}}}}}
    \|/ \|/\|/  \{{{{{{}}}}/  ~Y~ \|/ \|/\|/  \{{{{{{}}}}/  ~Y~
    \|/\\|/\|/ \\|~Y~//  \|/ \|/\\|/\|/ \\|~Y~//  \|/
    \|//\|/\|/,\\|/|/|// \|/ \|//\|/\|/,\\|/|/|// \|/
jgs^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"#.yellow());
}

pub fn bad_harvest() {
  println!(r#"
_\|/____\|/____\|/_"#);
}

pub fn normal_year() {
  println!("{}", r#"
                      ____
                     / ___`\
         /|         ( (   \ \
    |^v^v  V|        \ \/) ) )
    \  ____ /         \_/ / /
    ,Y`    `,            / /
    ||  -  -)           {{ }}
    \\   _\ |           | |
     \\ / _`\_         / /
     / |  ~ | ``\     _|_|
  ,-`  \    |  \ \  ,//(_}}
 /      |   |   | \/  \| |
|       |   |   | '   ,\ \
|     | \   /  /\  _/`  | |
\     |  | |   | ``     | |
 |    \  \ |   |        | |
 |    |   |/   |        / /
 |    |        |        | |"#.yellow());
}

pub fn victory_image() {
  println!("{}", r#"
               )\         O_._._._A_._._._O         /(
                \`--.___,'=================`.___,--'/
                 \`--._.__                 __._,--'/
                   \  ,. l`~~~~~~~~~~~~~~~'l ,.  /
       __            \||(_)!_!_!_.-._!_!_!(_)||/            __
       \\`-.__        ||_|____!!_|;|_!!____|_||        __,-'//
        \\    `==---='-----------'='-----------`=---=='    //
        | `--.                                         ,--' |
         \  ,.`~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~',.  /
           \||  ____,-------._,-------._,-------.____  ||/
            ||\|___!`======="!`======="!`======="!___|/||
            || |---||--------||-| | |-!!--------||---| ||
  __O_____O_ll_lO_____O_____O|| |'|'| ||O_____O_____Ol_ll_O_____O__
  o H o o H o o H o o H o o |-----------| o o H o o H o o H o o H o
 ___H_____H_____H_____H____O =========== O____H_____H_____H_____H___
                          /|=============|\
()______()______()______() '==== +-+ ====' ()______()______()______()
||{{_}}{{_}}||{{_}}{{_}}||{{_}}{{_}}/| ===== |_| ===== |\{{_}}{{_}}||{{_}}{{_}}||{{_}}{{_}}||
||      ||      ||     / |==== s(   )s ====| \     ||      ||      ||
======================()  =================  ()======================
----------------------/| ------------------- |\----------------------
                     / |---------------------| \
-'--'--'           ()  '---------------------'  ()
                   /| ------------------------- |\    --'--'--'
       --'--'     / |---------------------------| \    '--'
                ()  |___________________________|  ()           '--'-
  --'-          /| _______________________________  |\
 --' gpyy      / |__________________________________| \"#.yellow());
}
