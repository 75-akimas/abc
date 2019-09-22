n = int(input())
a1 = list(map(int, input().split()))
a2 = list(map(int, input().split()))

dst = list()
for i in range(n):
    dst.append(int(0))
src = 0
for i in range(n):
    src += a1[i]
    dst[i] = src
    for j in range(i,n):
        dst[i] += a2[j]
dst.append(src)
print(max(dst))
