p = 52435875175126190479447740508185965837690552500527637822603658699938581184513
r = 13108968793781547619861935127046491459309155893440570251786403306729687672801

Cx.<x> = GF(p)[]

CX.<X> = GF(r)[]
λ = 8913659658109529928382530854484400854125314752504019737736543920008458395397
assert GF(r)(λ)**2 == GF(r)(-2)
M = Matrix([[-ZZ(λ), 1], [r, 0]])
N = M.LLL()
# we want det(N) == r
N[1] *= -1

def print_scalar(x, name='x', field='Fr'):
    print("const {}: Self::ScalarField =".format(name))
    print("\tfield_new!({}, \"{}\");".format(field, x))


k = 13020469846541679229284871966049163930547512860064266443491754589208008054493
print(k)
beta = vector([k,0]) * N**-1
print("β1=", int(beta[0]))
print("β2=", int(beta[1]))
b = vector([int(beta[0]), int(beta[1])]) * N
print("b1=",b[0])
print("b2=",b[1])
k1 = k-b[0]
k2 = -b[1]
print('k1=', k1)
print('k2=', k2)
