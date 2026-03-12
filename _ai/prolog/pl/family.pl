% 事實：parent(父母, 子女)
parent(john, mary).
parent(john, tom).
parent(mary, ann).
parent(mary, joe).

% 事實：性別
male(john).
male(tom).
male(joe).
female(mary).
female(ann).

% 規則：定義祖父母
grandparent(X, Z) :- parent(X, Y), parent(Y, Z).

% 規則：定義兄弟姐妹
sibling(X, Y) :- parent(Z, X), parent(Z, Y), X \= Y.

% 查詢範例：
% ?- grandparent(john, ann).  % 會回傳 true
% ?- sibling(mary, tom).     % 會回傳 true