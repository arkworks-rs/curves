p = 21888242871839275222246405745257275088696311157297823662689037894645226208583
r = 21888242871839275222246405745257275088548364400416034343698204186575808495617

Cx.<x> = GF(p)[]
β = (x**3-1).roots()[0][0]

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
