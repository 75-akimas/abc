n, k = (int(i) for i in input().split())
sum = 0
for i in range(n):
j = i+1
count = 0
while (j < k):
j *= 2
count += 1
sum += 1/n * pow(1/2, count)
print(sum)