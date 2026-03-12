% 檢查 X 是否在列表中
% 情況 1：X 就是列表的第一個元素
is_member(X, [X|_]).

% 情況 2：X 在列表的剩餘部分（Tail）中
is_member(X, [_|Tail]) :- is_member(X, Tail).

% 查詢範例：
% ?- is_member(apple, [banana, apple, orange]). % true
% ?- is_member(X, [a, b, c]).                   % 會依序找出 X = a, b, c