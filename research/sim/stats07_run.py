# ORIGINAL COMPUTATION for research strand 07 (Strohmandeln)
import random, itertools
from math import comb
from fractions import Fraction as F
random.seed(20261014)

# ---- deck: (kind, rank, suit); suit cards 'K','Q','C','V', pips ; tarocks 0..21 (0 = Skus)
SUITS='SCHD'  # spades clubs hearts diamonds
cards=[]
for s in SUITS:
    for r in ['K','Q','C','V','p1','p2','p3','p4']:
        cards.append(('s',r,s))
for t in range(22): cards.append(('t',t,None))
assert len(cards)==54
def nominal(c):
    k,r,s=c
    if k=='t': return 5 if r in (0,1,21) else 1
    return {'K':5,'Q':4,'C':3,'V':2}.get(r,1)
def is_trull(c): return c[0]=='t' and c[1] in (0,1,21)
def frac(c):  # fractional value, n=3: nominal-2/3 for 1-pointers->1/3 ...  (nominal-1)+1/3 for valued; empties 1/3
    v=nominal(c); return F(1,3) if v==1 else F(v-1)+F(1,3)

# ---- (1) point total checks
tot=sum(nominal(c) for c in cards)
print("(1) raw nominal sum of 54 cards =",tot," groups of 3 =",54//3," minus 2 each =",tot-2*(54//3))
print("    fractional-value sum =",sum(frac(c) for c in cards))
print("    kings",sum(nominal(c) for c in cards if c[1]=='K'),"queens",sum(nominal(c) for c in cards if c[1]=='Q'),
      "knights",sum(nominal(c) for c in cards if c[1]=='C'),"jacks",sum(nominal(c) for c in cards if c[1]=='V'),
      "trull",sum(nominal(c) for c in cards if is_trull(c)),
      "other tarocks",sum(nominal(c) for c in cards if c[0]=='t' and not is_trull(c)),
      "pips",sum(nominal(c) for c in cards if c[0]=='s' and c[1].startswith('p')))
print("    27 tricks in a game (27 cards each, 1 card each per trick) ->",27)
print("    win threshold 36 of 70 (35 = 35 + 2 cards counted in threes -> loses for declarer)")

# ---- (2) exact hypergeometric facts for a 15-card hand from 54 (22 tarocks, 4 kings)
N=54;H=15
def hyp(K,n,k): return F(comb(K,k)*comb(N-K,n-k),comb(N,n))
print("\n(2) 15-card hand: tarock-count distribution (exact)")
cum=F(0)
for k in range(0,16):
    p=hyp(22,H,k); cum+=p
    print(f"    {k:2d} tarocks  p={float(p):.4f}  P(>=k)={float(1-cum+p):.4f}")
print("    E[tarocks]=",15*22/54)
print("    P(no tarock in 15)=",float(hyp(22,H,0)),"  (the redeal/fold case)  = 1 in",round(1/float(hyp(22,H,0))))
trull=F(15*14*13,54*53*52)
print("    P(holding all three of Skus, Mond, Pagat in hand)=",float(trull),"= 1 in",round(1/float(trull)))
print("    P(holding Skus+Mond)=",float(F(15*14,54*53)))
print("    P(four kings in hand)=",float(F(comb(50,11),comb(54,15))),"= 1 in",round(1/float(F(comb(50,11),comb(54,15)))))
print("    P(>=11 tarocks in 15-card hand, Wareh 'more than half' threshold)=",float(sum(hyp(22,H,k) for k in range(12,16))),"(>=12:)", " (>=11:",float(sum(hyp(22,H,k) for k in range(11,16))),")")

# ---- (3) straw men
print("\n(3) straw men: 12 cards of 54 are your straw men (3 stacks x 4)")
print("    expected tarocks among your 12 straw cards a priori =",12*22/54," kings =",12*4/54,"; 'takers' (K or tarock) =",12*26/54)
# given you've seen your 15-card hand with t tarocks and k kings, the other 39 cards contain 22-t tarocks, 4-k kings
for t,k in [(6,1),(7,2),(9,2),(4,0),(11,2)]:
    print(f"    you hold {t} tarocks, {k} kings -> expect in your straw {12*((22-t)+(4-k))/39:.2f} takers ({12*(22-t)/39:.2f} tarocks + {12*(4-k)/39:.2f} kings)")
print("    P(a given 4-card stack, blind, has no K/tarock at all (28 non-king suit cards)) =",float(F(comb(28,4),comb(54,4))))
print("    P(top card of a stack is a K/tarock (taker)) =",26/54)

# ---- Monte Carlo of the opening reveal
def deal():
    d=cards[:]; random.shuffle(d)
    hands=[d[0:15],d[15:30]]
    st=[[d[30+4*i:34+4*i] for i in range(0,6,2)], [d[30+4*i:34+4*i] for i in range(1,6,2)]]
    # stacks list order: top card = index 0
    return hands,st
def reveal(hand,stacks):
    hand=hand[:]; exposed=[]; buried=[]; taken=0
    for stk in stacks:
        stk=stk[:]
        while stk:
            c=stk[0]
            if len(stk)==1 or c[0]=='t' or c[1]=='K':   # last card of a stack is taken regardless (Wareh, Werner: stays open only in variant)
                hand.append(stk.pop(0)); taken+=1
            else: break
        if stk: exposed.append(stk[0]); buried.append(stk[1:])
        else: exposed.append(None); buried.append([])
    return hand,exposed,buried,taken
M=100000
from collections import Counter
taken_c=Counter(); allface=0; emptied=Counter(); handsize=0; faceupstacks=Counter()
tark_after=Counter(); none_taken=0
pts_hand=[]; pts_all=[]; tark27=Counter(); trull27=0; trullA=0
for _ in range(M):
    hands,st=deal()
    h,e,b,t=reveal(hands[0],st[0])
    taken_c[t]+=1
    handsize+=len(h)
    emp=sum(1 for x in e if x is None); emptied[emp]+=1
    faceupstacks[sum(1 for x in e if x is not None and len(b[e.index(x)])==3)]+=0
    # a straw man 'entirely face-up suit cards' = original 4-card stack, no taker among its cards -> nothing could be taken, nothing buried-by-reveal
    # (the top three cards of a stack are only revealed one at a time; a 'fully non-taker stack' = all 4 non-takers; 3 non-takers+1 taker last card -> last card taken anyway)
    tark27[sum(1 for c in hands[0]+[x for s in st[0] for x in s] if c[0]=='t')]+=1
    trull27 += all(any(x==c for x in hands[0]+[y for s in st[0] for y in s]) for c in [('t',0,None),('t',1,None),('t',21,None)])
    trullA += sum(1 for c in hands[0] if is_trull(c))==3
print("\n    MC",M,"deals, player 0:")
print("    cards taken into hand at the opening reveal (0..12): ", {k:round(v/M,4) for k,v in sorted(taken_c.items())})
print("    mean taken =",sum(k*v for k,v in taken_c.items())/M,"; mean hand size after reveal =",handsize/M)
print("    P(no stack opened at all = 0 taken)=",taken_c[0]/M)
print("    number of stacks completely used up at opening:", {k:round(v/M,4) for k,v in sorted(emptied.items())})
print("    P(Trull all in player's 27 cards)=",trull27/M,"exact:",float(F(27*26*25,54*53*52)))
print("    tarocks among a player's 27 cards: mean",sum(k*v for k,v in tark27.items())/M,"; P(>=12)=",sum(v for k,v in tark27.items() if k>=12)/M,"; P(>=14)=",sum(v for k,v in tark27.items() if k>=14)/M)

# exact: distribution of 'taken at opening' is not needed; also exact P(stack fully non-taker 4 cards)
print("    exact P(all four cards of a given stack are non-taker) = C(28,4)/C(54,4) =",float(F(comb(28,4),comb(54,4))))
print("    exact P(some stack of the 3 is all non-takers) ~ MC below")
cnt=0;cnt2=Counter()
for _ in range(M):
    hands,st=deal()
    n=sum(1 for s in st[0] if all(c[0]=='s' and c[1]!='K' for c in s))
    cnt2[n]+=1
print("    number of a player's 3 stacks that are entirely non-taker (nothing can ever be taken from them early):",{k:round(v/M,4) for k,v in sorted(cnt2.items())})

# ---- (4) heuristic take-up measure (Wareh): safe trick points >= 20..25 (fractional values), operationalised 3 ways
def safe_score(hand, variant):
    sc=F(0)
    for c in hand:
        if is_trull(c) or (c[0]=='s' and c[1]=='K'): sc+=F(13,3)
    suits=Counter(c[2] for c in hand if c[0]=='s')
    for c in hand:
        if c[0]=='s' and c[1]=='Q':
            haveK=any(x[0]=='s' and x[1]=='K' and x[2]==c[2] for x in hand)
            if variant=='QwithK' and haveK: sc+=F(10,3)
            if variant=='Qlong' and (haveK or suits[c[2]]>=3): sc+=F(10,3)
            if variant=='Qany': sc+=F(10,3)
    return sc
res={v:Counter() for v in ['QwithK','Qlong','Qany']}
resT={v:Counter() for v in res}
K=100000
for _ in range(K):
    d=random.sample(cards,15)
    nt=sum(1 for c in d if c[0]=='t')
    for v in res:
        s=safe_score(d,v)
        for th in (20,22,25):
            if s>=th:
                res[v][th]+=1
                if nt>=6: resT[v][th]+=1
print("\n(4) share of 15-card hands whose 'safe-trick' value (Skus/Mond/Pagat/Kings 4 1/3 each + Queens 3 1/3, rule variants) reaches Wareh's thresholds")
for v in res:
    print("    ",v,{f">={th}":round(res[v][th]/K,4) for th in (20,22,25)}, " and also >=6 tarocks:",{f">={th}":round(resT[v][th]/K,4) for th in (20,22,25)})

# ---- (5) Wareh's claim: 'Four times out of ten you will succeed' - exposed Queen atop a straw man; where is the King of its suit?
loc=Counter(); n=0
for _ in range(500000):
    hands,st=deal()
    h,e,b,t=reveal(hands[0],st[0])
    for i,x in enumerate(e):
        if x is not None and x[0]=='s' and x[1]=='Q':
            # locate king of that suit, using the full-information deal
            kc=('s','K',x[2]); n+=1
            if kc in h: loc['own hand (incl. already taken)']+=1
            elif any(kc in s for s in b): loc['own buried straw']+=1
            elif kc in hands[1]: loc['opp hand (before opp reveal)']+=1
            elif any(kc in s for s in st[1]): loc['opp straw stack (maybe taken at opp reveal)']+=1
            else: loc['other']+=1
print("\n(5) exposed Queen atop my straw man at the start: location of the King of that suit (n=%d)"%n)
for k,v in loc.items(): print(f"    {k}: {v/n:.3f}")
