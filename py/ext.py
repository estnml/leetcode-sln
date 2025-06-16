def facOfNRec(n) -> int:
    if n == 0:
        return 1
    return facOfNRec(n - 1) * n


def facOfNIterative(n) -> int:
    i = 1
    result = 1
    while i <= n:
        result *= i
        i += 1
    return result


# 2 2 2 2 2
def powerOfNRec(n, p) -> int:
    if p == 0:
        return 1
    return powerOfNRec(n, p - 1) * n


def powerOfNIterative(n, p):
    result = 1
    i = 1
    while i <= p:
        result *= n
        i += 1
    return result

# e^x = x^0/ 0! + x^1/1! + x^2/2! + ... + x^n-1/ (n-1)! + x^n/n!
# e^x = x^n-1/ (n-1)! + x^n/n!
# def taylorSeriesRec(x) -> float:
#     taylorSeriesRec(x-1) * 
