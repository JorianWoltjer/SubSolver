# SubSolver

**Substitution Cipher Solver**

A command-line tool to solve [Substitution Ciphers](https://en.wikipedia.org/wiki/Substitution_cipher) using a **wordlist** instead of statistical analysis. Statistical analysis is sometimes not viable, when the text is in another language, or is very short. Most often this required manual analysis of guessing words, and looking up words that fit with specific patterns. This tool automates that process!

Using a wordlist, it can try all words in all positions, and use the knowledge of **repeated letters** and **spaces** to determine possible plaintexts and keys. This does mean however that *all* words in the plaintext need to be in the given wordlist. Because of this it is recommended to use a big wordlist, as this tool will handle it with no problem. 

It is written in Rust, and written for **speed**. Using and efficient algorithm that checks only what is absolutely required, most ciphertexts are cracked in seconds. It spits out all possible plaintexts and keys, so in the end you can look at them yourself to see if any of the solutions make sense. 

The implementation of the algorithm is taken from [this repository](https://github.com/wjmolina/substitution-cipher-breaker) by [@wjmolina](https://github.com/wjmolina). [This MIT paper included](https://github.com/wjmolina/substitution-cipher-breaker/raw/master/RobustDictionaryAttackofShortSimpleSubstitutionCiphers.pdf) goes through all the ideas of the algorithm and why they work. 

## Installation

```Bash
cargo install sub-solver
```

## Usage

```Shell
$ sub-solver --help
Substitution Cipher Solver

Usage: sub-solver [OPTIONS] <--string <STRING>|--file <FILE>>

Options:
  -s, --string <STRING>      Ciphertext string to solve
  -f, --file <FILE>          Path to the ciphertext file
  -w, --wordlist <WORDLIST>  Path to the wordlist file (default: built-in english.txt)
  -k, --key <KEY>            Starting key, letter mapping (default: empty, example: "a:b,c:d,e:f", "ab,cd,ef", "b?d?f?????????????????????")
  -F, --fill-key             Fill in unknowns in solution with random unused letters (default: false)
  -n, --no-cache             Disable dictionary cache (default: false)
  -h, --help                 Print help
```

## Examples

* Plaintext: "some english text to showcase my tool in action"
* Key: `IYFWDQZMRVJHXLCKOUTEAGPBSN` ([CyberChef](https://gchq.github.io/CyberChef/#recipe=Substitute('ABCDEFGHIJKLMNOPQRSTUVWXYZ','IYFWDQZMRVJHXLCKOUTEAGPBSN',true)&input=U29tZSBlbmdsaXNoIHRleHQgdG8gc2hvd2Nhc2UgbXkgdG9vbCBpbiBhY3Rpb24))
* Ciphertext: "Tcxd dlzhrtm edbe ec tmcpfitd xs ecch rl ifercl"

Use `-s` to input a ciphertext string directly, or `-f` to specify a path to a file containing the ciphertext. 

```Shell
$ time ./target/release/sub-solver -s "Tcxd dlzhrtm edbe ec tmcpfitd xs ecch rl ifercl"
[*] Using empty starting key
[*] Using built-in english wordlist
[+] Loaded 13255 unique patterns
[+] Saved dictionary cache
[*] Input string: "Tcxd dlzhrtm edbe ec tmcpfitd xs ecch rl ifercl"
[+] Parsed 9 input words
[+] Pruned impossible words
[*] Starting to find solutions...
?xoetc?la??nh??w?iys???m?g -> some english text to showcase my tool in action
?xoetc?la??nh??w?irs???m?g -> some english text to showcase mr tool in action
?xoetc?la??nh??w?ius???m?g -> some english text to showcase mu tool in action
[+] Finished! (3 solutions)

real    0m0.117s
```

If we have a really short ciphertext, there may be a lot of possible solutions. If you know any part of the key however, you can specify it with the `-k` argument to reduce the searching to only fit that starting key. 

* Plaintext: "some simple example"
* Key: `IYFWDQZMRVJHXLCKOUTEAGPBSN` ([CyberChef](https://gchq.github.io/CyberChef/#recipe=Substitute('ABCDEFGHIJKLMNOPQRSTUVWXYZ','IYFWDQZMRVJHXLCKOUTEAGPBSN',true)&input=c29tZSBzaW1wbGUgZXhhbXBsZQ))
* Ciphertext: "tcxd trxkhd dbixkhd"

There are a few different formats for the starting key depending on what makes the most sense to you. You can use a comma-separated list of two characters each, that map the ciphertext letter to a plaintext letter. If we in this example know the "dbixkhd" word is "example", we know that 'd' must be 'e', 'b' must be 'x', etc. The starting key in this case would be `de,bx,ia,xm,kp,hl` for all the characters we know. 

```Shell
$ sub-solver -s "tcxd trxkhd dbixkhd" -k 'de,bx,ia,xm,kp,hl'
[*] Using starting key: "de,bx,ia,xm,kp,hl"
...
[*] Starting to find solutions...
?xoe???la?p??????i?d???m?? -> dome dimple example
?xoe???la?p??????u?r???m?? -> rome rumple example
?xie???la?p??????u?r???m?? -> rime rumple example
?xoe???la?p??????i?s???m?? -> some simple example
[+] Finished! (4 solutions)
```

The rest of the options work as follows:

* `-w`, `--wordlist` = Specify a path to your own wordlist, instead of the built-in english wordlist of 58.000 words
* `-F`, `--fill-key` = Fill unknown characters in the final printed key with a possible guess of what those characters may be (example: "?xoe???la?p??????i?s???m??" -> "bxoecdflagphjknqritsuvwmyz")
* `-n`, `--no-cache` = Turn off saving and loading the dictionary from the file cache. Normally, any time a wordlist is turned into a dictionary, it is cached to a file so that does not have to happen again for multiple runs
