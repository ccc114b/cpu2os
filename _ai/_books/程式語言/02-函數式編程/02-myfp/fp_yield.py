def each(a, f):
    for x in a:
        f(x)

def map(a, f):
    for x in a:
        yield f(x)

def filter(a, f):
    for x in a:
        if f(x): yield x

def reduce(a, f, init):
    r = init
    for x in a:
        r = f(r, x)
    return r

if __name__=="__main__":
    a = range(1,5)
    print('a=', a)
    each(a, lambda x:print(x))
    each(a, lambda x:print(x) if x%2==0 else None)
    print(list(map(a, lambda x:x*x)))
    print(list(filter(a, lambda x:x%2==1)))
    print(reduce(a, lambda x,y:x+y, 0))
