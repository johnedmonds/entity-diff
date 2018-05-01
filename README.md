# Introduction

Library to compute the edits required to transform one list of elements into another list of elements. Although you'll most likely be using it to diff two strings (i.e. a regular diff tool).

# Example

```
let string1 = "abc";
let string2 = "bcd";
let string1_as_vec: Vec<char> = string1.chars().collect();
let string2_as_vec: Vec<char> = string2.chars().collect();

let expected_inserted_char = 'd';

assert_eq!(diff(&string1_as_vec, &string2_as_vec)), vec![Edit::Delete, Edit::Keep, Edit::Keep, Edit::Insert(&expected_inserted_char)]);
```