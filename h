ECHO(1)                               User Commands                              ECHO(1)

NNAAMMEE
       echo - display a line of text

SSYYNNOOPPSSIISS
       eecchhoo [_S_H_O_R_T_-_O_P_T_I_O_N]... [_S_T_R_I_N_G]...
       eecchhoo _L_O_N_G_-_O_P_T_I_O_N

DDEESSCCRRIIPPTTIIOONN
       Echo the STRING(s) to standard output.

       --nn     do not output the trailing newline

       --ee     enable interpretation of backslash escapes

       --EE     disable interpretation of backslash escapes (default)

       ----hheellpp display this help and exit

       ----vveerrssiioonn
              output version information and exit

       If --ee is in effect, the following sequences are recognized:

       \\     backslash

       \a     alert (BEL)

       \b     backspace

       \c     produce no further output

       \e     escape

       \f     form feed

       \n     new line

       \r     carriage return

       \t     horizontal tab

       \v     vertical tab

       \0NNN  byte with octal value NNN (1 to 3 digits)

       \xHH   byte with hexadecimal value HH (1 to 2 digits)

       NOTE:  your  shell may have its own version of echo, which usually supersedes the
       version described here.  Please refer to your shell's documentation  for  details
       about the options it supports.

AAUUTTHHOORR
       Written by Brian Fox and Chet Ramey.

RREEPPOORRTTIINNGG BBUUGGSS
       GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
       Report any translation bugs to <https://translationproject.org/team/>

CCOOPPYYRRIIGGHHTT
       Copyright  © 2020 Free Software Foundation, Inc.  License GPLv3+: GNU GPL version
       3 or later <https://gnu.org/licenses/gpl.html>.
       This is free software: you are free to change and redistribute it.  There  is  NO
       WARRANTY, to the extent permitted by law.

SSEEEE AALLSSOO
       Full documentation <https://www.gnu.org/software/coreutils/echo>
       or available locally via: info '(coreutils) echo invocation'

GNU coreutils 8.32                    February 2024                              ECHO(1)
