p = 475922286169261325753349249653048451545124879242694725395555128576210262817955800483758081
r = 475922286169261325753349249653048451545124878552823515553267735739164647307408490559963137

Cx.<x> = GF(p)[]
β = (x**3-1).roots()[1][0]

def endo(P):
    return P.parent()(P[0] * β, P[1])
CX.<X> = GF(r)[]
λ = (X**3-1).roots()[0][0]
M = Matrix([[-ZZ(λ), 1], [r, 0]])
N = M.LLL()

def print_scalar(x, name='x', field='Fr'):
    print("const {}: Self::ScalarField =".format(name))
    print("\tfield_new!({}, \"{}\");".format(field, x))


k = 46726240763639862128214388288720131204625575015731614850157206947646262134152
print(k)
for nn in (~N).coefficients():
    print(nn)
beta = vector([k,0]) * N**-1
print("β1 = ", int(beta[0]))
print("β2 = ", int(beta[1]))
b = vector([int(beta[0]), int(beta[1])]) * N
print("b1=", b[0])
print("b2=", b[1])
k1 = k-b[0]
k2 = -b[1]
print("k1=", k1)
print("k2=", k2)

E = EllipticCurve(GF(p), [0,4])
g = E(0x17F1D3A73197D7942695638C4FA9AC0FC3688C4F9774B905A14E3A3F171BAC586C55E83FF97A1AEFFB3AF00ADB22C6BB, 0x08B3F481E3AAA0F1A09E30ED741D8AE4FCF5E095D5D00AF600DB18CB2C04B3EDD03CC744A2888AE40CAA232946C5E7E1)
