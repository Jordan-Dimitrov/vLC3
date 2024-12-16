.orig x3000
	JSR INIT
	JSR displayBoard
	LD R5, p1turnchar
	LEA R4, p1TurnMessage
	JSR playerTurn
	JSR displayBoard
	AND R1, R1, #0
	ADD R1, R1, #4
loop2
	LEA R4, p2TurnMessage
	LD R5, p2turnchar
	JSR playerTurn
	JSR displayBoard
	LEA R4, p1TurnMessage
	LD R5, p1turnchar
	JSR playerTurn
	JSR displayBoard
	ADD R1, R1, #-1
	BRnp loop2
	LEA R0, tie
	TRAP x22
	HALT
INIT
	LD R6, characterStorage
	LD R0, emptySpace
	AND R1,R1, #0
	ADD R1, R1, #9
resetBoard
	STR R0, R6, #0
	ADD R6,R6, #1
	ADD R1,R1, #-1
	BRp resetBoard
RET
displayBoard
	ST R7, SaveR7
	AND R2, R2, #0
	ADD R2, R2, #3
	LD R6, characterStorage
loop
	LDR R0, R6, #0
	TRAP x21
	LEA R0, col
	TRAP x22
	ADD R6, R6, #1
	LDR R0, R6, #0
	TRAP x21
	LEA R0, col
	TRAP x22
	ADD R6, R6, #1
	LDR R0, R6, #0
	TRAP x21
	LD R0, nullTerminator
	TRAP x21
	LEA R0, row
	TRAP x22
	LD R0, nullTerminator
	TRAP x21
	ADD R6, R6, #1
	ADD R2, R2, #-1
	BRnp loop
	LD R7, SaveR7
RET
playerTurn
	ST R7, SaveR7
p1TurnReDo
	AND R3, R3, #0
	LD R6, characterStorage
	LD R2, ASCIIoffset
	AND R0, R0, #0
	ADD R0, R4, #0
	TRAP x22
	LEA R0, turnMessage
	TRAP x22
	TRAP x20
	TRAP x21
	ADD R3, R0, R2
	LD R0, nullTerminator
	TRAP x21
	ADD R6, R6, R3
	LDR R3, R6, #0
	LD R2, spaceCheckNum
	ADD R3, R3, R2
	BRnp p1TurnReDo
	STR R5, R6, #0
	LD R7, SaveR7
	JSR playerWinCheck
	LD R7, SaveR7
RET
playerWinCheck
Row1check
	NOT R5, R5
	ADD R5, R5, #1
	LD R6, characterStorage
	LDR R0, R6, #0
	ADD R0, R0, R5
	BRnp Row2check
	LDR R0, R6, #1
	ADD R0, R0, R5
	BRnp Row2check
	LDR R0, R6, #2
	ADD R0, R0, R5
	BRnp Row2check
	BRnzp whoWon
Row2check
	LDR R0, R6, #3
	ADD R0, R0, R5
	BRnp Row3check
	LDR R0, R6, #4
	ADD R0, R0, R5
	BRnp Row3check
	LDR R0, R6, #5
	ADD R0, R0, R5
	BRnp Row3check
	BRnzp whoWon
Row3check
	LDR R0, R6, #6
	ADD R0, R0, R5
	BRnp Col1check
	LDR R0, R6, #7
	ADD R0, R0, R5
	BRnp Col1check
	LDR R0, R6, #8
	ADD R0, R0, R5
	BRnp Col1check
	BRnzp whoWon
Col1check
	LDR R0, R6, #0
	ADD R0, R0, R5
	BRnp Col2check
	LDR R0, R6, #3
	ADD R0, R0, R5
	BRnp Col2check
	LDR R0, R6, #6
	ADD R0, R0, R5
	BRnp Col2check
	BRnzp whoWon
Col2check
	LDR R0, R6, #1
	ADD R0, R0, R5
	BRnp Col3check
	LDR R0, R6, #4
	ADD R0, R0, R5
	BRnp Col3check
	LDR R0, R6, #7
	ADD R0, R0, R5
	BRnp Col3check
	BRnzp whoWon
Col3check
	LDR R0, R6, #2
	ADD R0, R0, R5
	BRnp Vert1check
	LDR R0, R6, #5
	ADD R0, R0, R5
	BRnp Vert1check
	LDR R0, R6, #8
	ADD R0, R0, R5
	BRnp Vert1check
	BRnzp whoWon
Vert1check
	LDR R0, R6, #0
	ADD R0, R0, R5
	BRnp Vert2check
	LDR R0, R6, #4
	ADD R0, R0, R5
	BRnp Vert2check
	LDR R0, R6, #8
	ADD R0, R0, R5
	BRnp Vert2check
	BRnzp whoWon
Vert2check
	LDR R0, R6, #2
	ADD R0, R0, R5
	BRnp nullWinner
	LDR R0, R6, #4
	ADD R0, R0, R5
	BRnp nullWinner
	LDR R0, R6, #6
	ADD R0, R0, R5
	BRnp nullWinner
	BRnzp whoWon
nullWinner
RET
whoWon
	LD R1, p1turnchar
	ADD R1, R1, R5
	BRz p1Winner
	LD R1, p2turnchar
	ADD R1, R1, R5
	BRz p2Winner
p1Winner
	JSR displayBoard
	LEA R0, p1WinnerMessage
	TRAP x22
	HALT
p2Winner
	JSR displayBoard
	LEA R0, p2WinnerMessage
	TRAP x22
	HALT
p1Negation .fill xFFA8
SaveR7 .fill x0000
tie .Stringz "Tie\n"
emptySpace .fill x20
spaceCheckNum .fill xFFE0
nullTerminator .fill #10
characterStorage .fill x2FF7
ASCIIoffset .fill xFFCF
p1turnchar .fill x58
p2turnchar .fill x4F
p1TurnMessage .stringz "Player 1 "
p2TurnMessage .stringz "Player 2 "
turnMessage .stringz "Enter a Number between 1-9: "
col .stringz " | "
row .Stringz "----------"
p1WinnerMessage .stringz "Player 1 wins\n"
p2WinnerMessage .stringz "Player 2 wins\n"
p2Negation .fill xFFB1
.end
