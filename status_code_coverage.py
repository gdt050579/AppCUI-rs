import sys

found_overall_rap = False
for line in open(sys.argv[1],"rt"):
	if line.startswith("|| Uncovered Lines") or line.startswith("|| Tested/Total Lines"):
		print()
		print(line.strip())
		print("=================================================================================")
		found_overall_rap = "Tested/Total" in line
		continue
	if sys.argv[2] in line:
		if found_overall_rap:
			v1 = int(line.rsplit("/",1)[0].rsplit(" ",1)[1].strip())
			v2 = int(line.rsplit("/",1)[1].split(" ",1)[0].strip())
			rap = int(v1*100/v2)
			print(line.replace("||","  [%3d %%] -> "%(rap)).rstrip())
		else:
			print(line.replace("||","  ").rstrip())
	