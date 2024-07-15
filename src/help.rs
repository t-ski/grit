pub static TEXT: &str = "\
Usage: grit <command> [--<arg:key>|-<arg:shorthand> *?]*

→ <command>

• patch             Start development of a patch.
• minor             Start development of a minor.
• major             Start development of a major.

    Options:
    --name, -n      Name to identify development.

• complete          Complete the current development.

    Flags:
    --keep          Keep development branch, i.e. do not delete.

    Options:
    --target, -t    Name of target branch if different than main.

• status            Display status information.
";