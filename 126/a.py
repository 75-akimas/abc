n, k = (int(i) for i in input().split())
s = input()
for i in range(n):
	if (i != int(k-1)):
		print(s[i], end='')
	else:
		print(s[i].lower(), end='')
