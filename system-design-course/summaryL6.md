# Byzantine Fault Tolerance
## Two Generals Problem
- Question: can't you solve it with private/public key cryptography?

## Byzantines General Problem
- multiple generals
- they all try to reach consensus
- how to treat trators?

### BGP Solutions for multiple cases
- n=1: trivial, you are the consensus
- n=2: 1 honest, 1 traitor => trivial, no consensus needed since you know the traitor
- n=3: 1 honest, 1 traitor => NO SOLUTION => you cannot tell who is lying...!
- Question: how many traitors can be tolerated?
  => no solution for 3m+1 with more >m traitors
  => example: 31 generals with 10 traitors works, 31 generals with 11 traitors DOES NOT!
  => example: 12 generals, 4 traitors => cannot be solved

### solvable BGP with => traitors < m
- 

