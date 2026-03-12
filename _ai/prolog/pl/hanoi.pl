% move(盤數, 起點, 終點, 輔助點)
hanoi(1, A, B, _) :-
    write('Move disk from '), write(A), write(' to '), write(B), nl.

hanoi(N, A, B, C) :-
    N > 1,
    M is N - 1,
    hanoi(M, A, C, B),
    hanoi(1, A, B, _),
    hanoi(M, C, B, A).

% 查詢範例：
% ?- hanoi(3, left, right, center).