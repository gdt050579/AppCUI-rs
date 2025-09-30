s = "matches!(ch as u32, "
for line in open("unicode_table.txt","rt"):
	line = line.strip()
	if len(line)==0: continue
	if line.startswith("#"): continue
	interval = ("0x" + line.split(";",1)[0].strip()).replace("..","..=0x")
	ty = line.split("#",1)[0].split(";",1)[1].strip()
	if ty=="W" or ty=="F":
		s += interval+" | "
if s.endswith("| "):
	s = s[:-3]
s += ")"
print(s)