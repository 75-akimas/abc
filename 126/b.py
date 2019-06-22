# your code goes here\
s = input()
op = ''
first = s[0:2]
second = s[2:4]
if (0 < int(first) <= 12):
	op += 'MM'
else:
	op += 'YY'
if (0 < int(second) <= 12):
	op += 'MM'
else:
	op += 'YY'

if (len(s) != 4):
	op = 'NA'
elif (op == 'MMMM'):
	op = 'AMBIGUOUS'
elif (op == 'YYYY'):
	if ((second == '00' and first == '00') or int(second) > 12 or int(first) > 12):
		op = 'NA'
print(op)

