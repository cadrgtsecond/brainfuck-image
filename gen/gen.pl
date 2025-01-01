% Ensure set_prolog_flag(double_quotes, chars).
%
% We'll be using prolog to implement the brainfuck generation logic
%
% The state of the brainfuck interpreter is represented by using a pair of lists
% If the cells contain -2, -1, 0, 1, 2 ..... and the head is on 0, then the state would be
%
% ```
% state([-1, -2], [0, 1, 2])
% state(Back, [Head|Rest])
% ```
%
% Reading the code further will make it clearer. Here is the interpreter

% Parses brainfuck code to a tree
parse_bf([]) --> [].
parse_bf([+|Rest]) --> "+", parse_bf(Rest).
parse_bf([-|Rest]) --> "-", parse_bf(Rest).
parse_bf([<|Rest]) --> "<", parse_bf(Rest).
parse_bf([>|Rest]) --> ">", parse_bf(Rest).
parse_bf([.|Rest]) --> ".", parse_bf(Rest).
parse_bf([loop(Loop)|Rest]) --> "[", parse_bf(Loop), "]", parse_bf(Rest).

eval_op(+, state(Back, [Head|Rest]), state(Back, [Headp|Rest])) :- Headp is Head + 1.
eval_op(-, state(Back, [Head|Rest]), state(Back, [Headp|Rest])) :- Headp is Head - 1.
eval_op(<, state([Head|Back], Rest), state(Back, [Head|Rest])).
eval_op(<, state([], Rest), state([], [0|Rest])).
eval_op(>, state(Back, [Head|Rest]), state([Head|Back], Rest)).
eval_op(>, state(Back, [Head]), state([Head|Back], [0])).
eval_op('.', State, State) :- State = state(_, [Code|_]), print(Code), char_code(Char, Code), print(Char).

eval_op(loop(Content), State, Statep) :-
  State = state(_, [0| _]),
  evaluate(Content, State, Statep),
  Statep = state(_, [Heap | _]),
  Headp \= 0,
  !.
eval_op(loop(_), State, Statep).

evaluate([], State, State).
evaluate([Op|Rest]) --> eval_op(Op), evaluate(Rest).

rev(In, Out) :- rev(In, Out, []).
rev([], Out, Out).
rev([Head|Tail], Out, Acc) :- rev(Tail, Out, [Head|Acc]).
