template = [
	(0,0,0,"Black"),
	(0,0,1,"Blue"),
	(0,1,0,"Green"),
	(1,0,0,"Red"),
	(0,1,1,"Aqua"),
	(1,1,0,"Yeallow"),
	(1,0,1,"Pink"),
	(1,1,1,"White")
]

v = []
for r in range(0,5):
	for g in range(0,5):
		for b in range(0,5):
			v+=[[r,g,b,"",0,False]]

for t in template:
	for i in range(1,5):
		k = t[0]*25*i+t[1]*5*i+t[2]*i
		v[k][3] = t[3]
		v[k][4] = i*25;
		v[k][5] = True

v[0][3] = "Black"
v[0][4] = 0
v[0][5] = True

for i in v:
	if i[5]==False:
		best = 1000000
		res = None
		for j in v:
			if j[5] == True:
				dif = abs(j[0]-i[0])+ abs(j[1]-i[1]) + abs(j[2]-i[2])
				m = max(abs(j[0]-i[0]),abs(j[1]-i[1]),abs(j[2]-i[2]))
				dif = dif * 100 + m;
				if dif < best:
					best = dif
					res = j
		i[4] = res[4]
		i[3] = res[3]
		#print("Similarity: ",i," <-> ",res," => ",best)	
		#raise SystemExit

for i in v:
	print(i)

s = "const colors_64 = ["
for i in v:
	s += "Color::"+i[3]+","
s = s[:-1]+"]"
print(s)

s = "const color_64_proc: [u8;125] = ["
for i in v:
	s += str(i[4])+","
s = s[:-1]+"]"
print(s)
