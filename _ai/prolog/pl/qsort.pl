qsort([], []).
qsort([Pivot|Tail], Sorted) :-
    split(Pivot, Tail, Less, Greater),
    qsort(Less, SortedLess),
    qsort(Greater, SortedGreater),
    append(SortedLess, [Pivot|SortedGreater], Sorted).

% 拆分列表為大於與小於 Pivot 的兩部分
split(_, [], [], []).
split(Pivot, [X|T], [X|Less], Greater) :-
    X =< Pivot, split(Pivot, T, Less, Greater).
split(Pivot, [X|T], Less, [X|Greater]) :-
    X > Pivot, split(Pivot, T, Less, Greater).

% 查詢範例：
% ?- qsort([3, 1, 4, 1, 5, 9], L). % L = [1, 1, 3, 4, 5, 9]