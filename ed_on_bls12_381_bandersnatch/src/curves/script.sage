D = -8
H = hilbert_class_polynomial(D)
p = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
Fp = GF(p)
FpX = Fp['X']
X = FpX.gen()
j = FpX(H).roots()[0][0]

# Weierstrass curve: y² = x³ + A * x + b
E = EllipticCurve(j=j)
if not((E.order()//4).is_prime()):
    # we want its twist, defined with sqrt5 (of Fp²)
    assert Fp(5).is_square() == False
    E=EllipticCurve([E.a4() * 5**2, E.a6() * 5**3])
a = E.a4()
b = E.a6()
r = E.order()//4
assert r.is_prime()
assert (E.quadratic_twist().order()//(2**7*3**3)).is_prime()

Fr = GF(r)
L = -Fr(-2).sqrt()

alpha = E.division_polynomial(2).roots()[0][0]
P = E.lift_x(alpha)
phi0,phi1 = E.isogeny(P)
E2 = E.isogeny_codomain(P)
# choice of u leads to psi or -psi
u = list(set((X**4 - E.a4()/E2.a4()).roots()) & set((X**6 - E.a6()/E2.a6()).roots()))[1][0]
assert u**4 == E.a4()/E2.a4()
assert u**6 == E.a6()/E2.a6()
u2 = u**2
u3 = u**3
r0,r1,_ = phi0.numerator()(X,1).list()
s0 = phi0.denominator()(X,1).list()[0]
t0,t1,_ = phi1.numerator()(X,1).list()

xG_W = 39553122782156925410137929261478471062900417019319705623270394372822154181075
yG_W = 12728420207359683933724156020262601669879381269724833745618077135849731841102

#######
# check
psiG = E(u2*(xG_W**2 + 44800*xG_W + 2257920000)/(xG_W + 44800),u3*yG_W*(xG_W**2+2*44800*xG_W + t0)/(xG_W+44800)**2)
assert psiG == int(L)*G
#####
