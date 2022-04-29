from sage.all import *
from sage.rings.finite_rings.finite_field_constructor import FiniteField
from sage.schemes.elliptic_curves.constructor import EllipticCurve

p = 8444461749428370424248824938781546531375899335154063827935233455917409239041 # BLS12-377 scalar field
Fp = FiniteField(p)
r = 2111115437357092606062206234695386632838870926408408195193685246394721360383
Fpz = Fp['z']
z = Fpz.gen()

# Twisted Edwards
a_te = Fp(-1)
d_te = Fp(3021)

G_te = [\
    Fp(4497879464030519973909970603271755437257548612157028181994697785683032656389),\
    Fp(4357141146396347889246900916607623952598927460421559113092863576544024487809)\
    ]

# Montgomery
a_m = Fp(3990301581132929505568273333084066329187552697088022219156688740916631500114)
b_m = Fp(4454160168295440918680551605697480202188346638066041608778544715000777738925)

G_m = [(1+G_te[1]) / (1-G_te[1]), (1+G_te[1]) / (1-G_te[1])/ G_te[0]]
assert 2*(a_te+d_te)/ (a_te - d_te) == a_m
assert 4/(a_te-d_te) == b_m

# Short Weierstrass
a_ws = (3-a_m**2) / (3*b_m**2)
b_ws = (2*a_m**3 - 9*a_m) / (27*b_m**3)
E = EllipticCurve([a_ws, b_ws])
assert E.order()%r == 0
G_ws = [G_m[0]/b_m + a_m / (3*b_m), G_m[1]/b_m]

G = E(G_ws)
assert r*G == 0

print("a_ws = {}".format(a_ws))
print("b_ws = {}".format(b_ws))
print("xG = {}".format(G_ws[0]))
print("yG = {}".format(G_ws[1]))


