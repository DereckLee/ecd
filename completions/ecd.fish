# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_ecd_global_optspecs
	string join \n color= h/help V/version
end

function __fish_ecd_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_ecd_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_ecd_using_subcommand
	set -l cmd (__fish_ecd_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c ecd -n "__fish_ecd_needs_command" -l color -d 'When to colorize encoding labels (`[UTF-8]`, `[SKIP]`, …) in batch output' -r -f -a "auto\t'Colorize when stdout is a terminal and `NO_COLOR` is unset'
never\t'Never colorize (default)'
always\t'Always colorize (ANSI escapes; useful for piping to `less -R`)'"
complete -c ecd -n "__fish_ecd_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c ecd -n "__fish_ecd_needs_command" -s V -l version -d 'Print version'
complete -c ecd -n "__fish_ecd_needs_command" -f -a "check" -d 'Detect file encoding(s)'
complete -c ecd -n "__fish_ecd_needs_command" -f -a "encodings" -d 'List valid encoding names'
complete -c ecd -n "__fish_ecd_needs_command" -f -a "convert" -d 'Convert file encoding'
complete -c ecd -n "__fish_ecd_needs_command" -f -a "complete" -d 'Generate shell completions to stdout'
complete -c ecd -n "__fish_ecd_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ecd -n "__fish_ecd_using_subcommand check" -s f -l file -d 'File(s) to check (repeatable)' -r -F
complete -c ecd -n "__fish_ecd_using_subcommand check" -s d -l dir -d 'Directory/directories to scan (repeatable); defaults to `.` when `-r` or `-p` is given alone' -r -f -a "(__fish_complete_directories)"
complete -c ecd -n "__fish_ecd_using_subcommand check" -s p -l pattern -d 'Glob pattern to filter files (`*.java`; with `-r` matches at any depth)' -r
complete -c ecd -n "__fish_ecd_using_subcommand check" -s i -l ignore -d 'Ignore files with this encoding (case-insensitive)' -r -f -a "utf-8\t''
utf-16le\t''
utf-16be\t''
gbk\t''
big5\t''
shift_jis\t''
euc-jp\t''
euc-kr\t''
iso-2022-jp\t''
koi8-r\t''
koi8-u\t''
windows-1251\t''
x-mac-cyrillic\t''
iso-8859-2\t''
iso-8859-3\t''
iso-8859-4\t''
iso-8859-5\t''
iso-8859-6\t''
iso-8859-7\t''
iso-8859-8\t''
iso-8859-8-i\t''
iso-8859-10\t''
iso-8859-13\t''
iso-8859-14\t''
iso-8859-15\t''
iso-8859-16\t''
windows-1250\t''
windows-1252\t''
windows-1253\t''
windows-1254\t''
windows-1255\t''
windows-1256\t''
windows-1257\t''
windows-1258\t''
ibm866\t''
windows-874\t''
macintosh\t''
ascii\t''"
complete -c ecd -n "__fish_ecd_using_subcommand check" -s e -l exclude -d 'Additional directory names to exclude' -r
complete -c ecd -n "__fish_ecd_using_subcommand check" -s j -l jobs -d 'Number of parallel jobs' -r
complete -c ecd -n "__fish_ecd_using_subcommand check" -l color -d 'When to colorize encoding labels (`[UTF-8]`, `[SKIP]`, …) in batch output' -r -f -a "auto\t'Colorize when stdout is a terminal and `NO_COLOR` is unset'
never\t'Never colorize (default)'
always\t'Always colorize (ANSI escapes; useful for piping to `less -R`)'"
complete -c ecd -n "__fish_ecd_using_subcommand check" -s r -l recursive -d 'Recursively scan into subdirectories (used with `-d`, or current directory when `-d` is omitted)'
complete -c ecd -n "__fish_ecd_using_subcommand check" -l no-default-excludes -d 'Disable default directory excludes (.git, node_modules, target)'
complete -c ecd -n "__fish_ecd_using_subcommand check" -s v -l verbose -d 'Print statistics to stderr'
complete -c ecd -n "__fish_ecd_using_subcommand check" -s q -l quiet -d 'Suppress normal output (errors only)'
complete -c ecd -n "__fish_ecd_using_subcommand check" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c ecd -n "__fish_ecd_using_subcommand encodings" -l color -d 'When to colorize encoding labels (`[UTF-8]`, `[SKIP]`, …) in batch output' -r -f -a "auto\t'Colorize when stdout is a terminal and `NO_COLOR` is unset'
never\t'Never colorize (default)'
always\t'Always colorize (ANSI escapes; useful for piping to `less -R`)'"
complete -c ecd -n "__fish_ecd_using_subcommand encodings" -s l -l list -d 'List valid encoding names (default action)'
complete -c ecd -n "__fish_ecd_using_subcommand encodings" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c ecd -n "__fish_ecd_using_subcommand convert" -s f -l file -d 'File to convert' -r -F
complete -c ecd -n "__fish_ecd_using_subcommand convert" -l from -d 'Source encoding' -r -f -a "utf-8\t''
utf-16le\t''
utf-16be\t''
gbk\t''
big5\t''
shift_jis\t''
euc-jp\t''
euc-kr\t''
iso-2022-jp\t''
koi8-r\t''
koi8-u\t''
windows-1251\t''
x-mac-cyrillic\t''
iso-8859-2\t''
iso-8859-3\t''
iso-8859-4\t''
iso-8859-5\t''
iso-8859-6\t''
iso-8859-7\t''
iso-8859-8\t''
iso-8859-8-i\t''
iso-8859-10\t''
iso-8859-13\t''
iso-8859-14\t''
iso-8859-15\t''
iso-8859-16\t''
windows-1250\t''
windows-1252\t''
windows-1253\t''
windows-1254\t''
windows-1255\t''
windows-1256\t''
windows-1257\t''
windows-1258\t''
ibm866\t''
windows-874\t''
macintosh\t''
ascii\t''"
complete -c ecd -n "__fish_ecd_using_subcommand convert" -l to -d 'Target encoding' -r -f -a "utf-8\t''
utf-16le\t''
utf-16be\t''
gbk\t''
big5\t''
shift_jis\t''
euc-jp\t''
euc-kr\t''
iso-2022-jp\t''
koi8-r\t''
koi8-u\t''
windows-1251\t''
x-mac-cyrillic\t''
iso-8859-2\t''
iso-8859-3\t''
iso-8859-4\t''
iso-8859-5\t''
iso-8859-6\t''
iso-8859-7\t''
iso-8859-8\t''
iso-8859-8-i\t''
iso-8859-10\t''
iso-8859-13\t''
iso-8859-14\t''
iso-8859-15\t''
iso-8859-16\t''
windows-1250\t''
windows-1252\t''
windows-1253\t''
windows-1254\t''
windows-1255\t''
windows-1256\t''
windows-1257\t''
windows-1258\t''
ibm866\t''
windows-874\t''
macintosh\t''
ascii\t''"
complete -c ecd -n "__fish_ecd_using_subcommand convert" -s o -l output -d 'Output file (stdout when omitted)' -r -F
complete -c ecd -n "__fish_ecd_using_subcommand convert" -l color -d 'When to colorize encoding labels (`[UTF-8]`, `[SKIP]`, …) in batch output' -r -f -a "auto\t'Colorize when stdout is a terminal and `NO_COLOR` is unset'
never\t'Never colorize (default)'
always\t'Always colorize (ANSI escapes; useful for piping to `less -R`)'"
complete -c ecd -n "__fish_ecd_using_subcommand convert" -l strict -d 'Fail on decode/encode errors (no replacement characters)'
complete -c ecd -n "__fish_ecd_using_subcommand convert" -l bom -d 'Write BOM for UTF-8 / UTF-16 output'
complete -c ecd -n "__fish_ecd_using_subcommand convert" -l force -d 'Overwrite existing output file'
complete -c ecd -n "__fish_ecd_using_subcommand convert" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c ecd -n "__fish_ecd_using_subcommand complete" -l color -d 'When to colorize encoding labels (`[UTF-8]`, `[SKIP]`, …) in batch output' -r -f -a "auto\t'Colorize when stdout is a terminal and `NO_COLOR` is unset'
never\t'Never colorize (default)'
always\t'Always colorize (ANSI escapes; useful for piping to `less -R`)'"
complete -c ecd -n "__fish_ecd_using_subcommand complete" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c ecd -n "__fish_ecd_using_subcommand help; and not __fish_seen_subcommand_from check encodings convert complete help" -f -a "check" -d 'Detect file encoding(s)'
complete -c ecd -n "__fish_ecd_using_subcommand help; and not __fish_seen_subcommand_from check encodings convert complete help" -f -a "encodings" -d 'List valid encoding names'
complete -c ecd -n "__fish_ecd_using_subcommand help; and not __fish_seen_subcommand_from check encodings convert complete help" -f -a "convert" -d 'Convert file encoding'
complete -c ecd -n "__fish_ecd_using_subcommand help; and not __fish_seen_subcommand_from check encodings convert complete help" -f -a "complete" -d 'Generate shell completions to stdout'
complete -c ecd -n "__fish_ecd_using_subcommand help; and not __fish_seen_subcommand_from check encodings convert complete help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
