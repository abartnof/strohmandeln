# ORIGINAL COMPUTATION (b): where is the King of the suit of a Queen exposed atop MY straw man, AFTER BOTH players have done the opening reveal?
import random
from collections import Counter
random.seed(7)
cards=[('s',r,s) for s in 'SCHD' for r in ['K','Q','C','V','p1','p2','p3','p4']]+[('t',t,None) for t in range(22)]
def reveal(hand,stacks):
    hand=hand[:]; exposed=[]; buried=[]
    for stk in stacks:
        stk=stk[:]
        while stk:
            c=stk[0]
            if len(stk)==1 or c[0]=='t' or c[1]=='K': hand.append(stk.pop(0))
            else: break
        exposed.append(stk[0] if stk else None); buried.append(stk[1:])
    return hand,exposed,buried
loc=Counter(); n=0
N=600000
for _ in range(N):
    d=cards[:]; random.shuffle(d)
    h0,h1=d[0:15],d[15:30]
    s0=[d[30:34],d[38:42],d[46:50]]; s1=[d[34:38],d[42:46],d[50:54]]
    H0,E0,B0=reveal(h0,s0); H1,E1,B1=reveal(h1,s1)
    for x in E0:
        if x and x[0]=='s' and x[1]=='Q':
            k=('s','K',x[2]); n+=1
            if k in H0: loc['K in my hand after reveal']+=1
            elif k in H1: loc['K in opp hand after reveal (can top the Queen at once)']+=1
            elif any(k in b for b in B0): loc['K asleep under my own face-up cards']+=1
            elif any(k in b for b in B1): loc['K asleep under opp face-up cards']+=1
            else: loc['K showing face-up on a stack']+=1
print("n exposed Queens atop own straw man:",n)
for k,v in sorted(loc.items(), key=lambda kv:-kv[1]): print(f"  {k}: {v/n:.3f}")
asleep=(loc['K asleep under my own face-up cards']+loc['K asleep under opp face-up cards'])/n
print("  P(King still asleep in some straw man) =",round(asleep,3))
print("  P(opponent holds the King in hand at once) =",round(loc['K in opp hand after reveal (can top the Queen at once)']/n,3))
