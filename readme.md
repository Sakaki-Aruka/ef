# What is "ef"?
"ef" is a util command about check a text file.  
Made by Rust.

# Usage
- -f : target file name. **(need)**
- -s : regex pattern **(need)**
- -n : display line numbers or not. flag.
- -l : displays lines what are placed near a line what be matched with a given regex pattern. (int/ 0 ~)

---

(e.g on `test.txt`)
```text
hello world
second line
third line
forth line
fifth line
goodbye world
```

command : `cargo run -- -f=test.txt -s=world -l=1`
```text
==========
hello world
second line
==========
fifth line
goodbye world
```

---

command : `cargo run -- -f=test.txt -s=world -l=1 -n`
```text
==========
1 hello world
2 second line
==========
5 fifth line
6 goodbye world
```

