import os,sys,datetime

# use create_vfs folder root_in_vsf
# example: create_vsf C:\ABC\CDE\XYZ E:\

for (r,dirs,files) in os.walk(sys.argv[1]):
	for d in dirs:
		name = os.path.join(r,d)
		creation_time = os.path.getctime(name)
		readable_time = datetime.datetime.fromtimestamp(creation_time)
		pname = name.replace(sys.argv[1],sys.argv[2])
		print("d,"+pname+",0,"+str(readable_time).split(".",1)[0])
	for f in files:
		name = os.path.join(r,f)
		size = os.path.getsize(name)
		creation_time = os.path.getctime(name)
		readable_time = datetime.datetime.fromtimestamp(creation_time)
		pname = name.replace(sys.argv[1],sys.argv[2])
		print("f,"+pname+","+str(size)+","+str(readable_time).split(".",1)[0])
