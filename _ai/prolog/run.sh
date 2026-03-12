#!/bin/bash
set -x
python3 prolog.py pl/fact.pl <<EOF
factorial(5, W).
exit
EOF

python3 prolog.py pl/family.pl <<EOF
grandparent(john, ann).
sibling(mary, tom). 
exit
EOF

python3 prolog.py pl/hanoi.pl <<EOF
hanoi(3, left, right, center).
exit
EOF

python3 prolog.py pl/ismember.pl <<EOF
is_member(apple, [banana, apple, orange]).
is_member(X, [a, b, c]).
exit
EOF

python3 prolog.py pl/qsort.pl <<EOF
qsort([3, 1, 4, 1, 5, 9], L).
exit
EOF
