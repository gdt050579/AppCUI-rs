import os,sys

data = {}

template_values = r"""
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ${NAME}
{
	${VALUES}
}
"""
template = r"""
${ENUM}

static HASH_TO_VALUE: [Option<${NAME}>; ${DEVIDER}] = [
	${TABLE_VALUES}
];

static HASH_COLISION_VALIDATOR: [u64; ${DEVIDER}] = [
	${TABLE_HASHES}
];

impl ${NAME}
{
    pub(super) fn from_hash(hash: u64) -> Option<${NAME}> {
        let entry_index = (hash % ${DEVIDER}) as usize;
        if HASH_COLISION_VALIDATOR[entry_index] != hash {
            return None;
        }
        return HASH_TO_VALUE[entry_index];
    }	
    pub(crate) fn name(&self) -> &'static str {
	match self {
		${TO_STRING}
	}
    }
}
"""

def ComputeFNVHash(s):
	hash = 0xcbf29ce484222325
	for c in s.lower():
		hash = hash ^ (ord(c) & 0xFF)
		hash = hash & 0xFFFFFFFFFFFFFFFF
		hash = hash * 0x00000100000001B3
		hash = hash & 0xFFFFFFFFFFFFFFFF
	return hash

def Error(msg):
	print("[ERROR] -> ",msg)

def Info(msg):
	print("[INFO ] -> ",msg)

def LoadIni(fname):
	global data
	section = ""
	data["list"] = {}
	data["general"] = {}
	for line in open(fname,"rt"):
		line = line.strip()
		#skip empty lines
		if len(line)==0:
			continue
		# skip comments
		if line.startswith("#") or line.startswith(";"):
			continue
		# check if section
		if line.startswith("["):
			if line == "[general]":
				section = "general"
				continue
			elif line == "[list]":
				section = "list"
				continue
			else:
				Error("Invalid section `"+line+"` in file `"+fname+"`. Allowed sections are [general] and [list] !");
				return False
		# else we should have a key = value pair
		if not "=" in line:
			Error("Expectin a key = value pair in `"+line+"`")
			return False
		if section == "list":
			# format is value = key1,key2,...
			value = line.split("=",1)[0].strip()
			keys = line.split("=",1)[1].lower().strip().split(",")
			if section == "":
				Error("No section defined for `"+line+"`")
				return False
			for k in keys:
				data[section][k.lower().strip()] = value
		else:
			# format is key = value
			key = line.split("=",1)[0].lower().strip()
			value = line.split("=",1)[1].strip()		
			data[section][key] = value	

	#generic checks
	if not "name" in data["general"]:
		Error("Enum name field must be defined for [general] section")
		return False
	if not "valuetype" in data["general"]:
		Error("ValueType field must be defined for [general] section")
		return False		
	if (data["general"]["valuetype"] == "none") or (data["general"]["valuetype"] == "generate"):
		data["general"]["valuetype"] = ""
	if len(data["list"])==0:
		Error("No values added to [list] section")
		return False
	return True		

def ComputeHashDevider(m):
	l = len(m)
	while True:
		d = {}
		for k in m:
			newHash = ComputeFNVHash(k) % l
			if newHash in d:
				break
			d[newHash] = 1
		if len(d)==len(m):
			return l
		else:
			l = l + 1

def BuildCode(devider):
	global data
	global template
	global template_values
	values = ""
	idx = 0
	table_values = ["None"]*devider
	table_hashes = [0]*devider
	results = {}
	# to string part
	to_string = ""
	for k in data["list"]:
		if data["list"][k] not in results:
			values = values + data["list"][k]+" = "+str(idx)+",\n\t\t"
			to_string += "${NAME}::"+data["list"][k]+" => \""+data["list"][k]+"\",\n\t"
			results[data["list"][k]] = idx
			idx+=1
		hash = ComputeFNVHash(k)
		d_idx = hash % devider
		table_values[d_idx] = "Some(${NAME}::"+data["list"][k]+")"
		table_hashes[d_idx] = hash
	s_table_values = ""
	for k in table_values:
		s_table_values+=k+","
	s_table_hashes = ""
	for k in table_hashes:
		s_table_hashes+="0x%X,"%k
	value_type = data["general"]["valuetype"]
	if len(value_type)==0:
		values = template_values.replace(r"${VALUES}",values)
		value_type = "Type"
	else:
		values = ""
	
	s = template
	s = s.replace(r"${DEVIDER}",str(devider))
	s = s.replace(r"${ENUM}",values)
	s = s.replace(r"${TABLE_VALUES}",s_table_values)
	s = s.replace(r"${TABLE_HASHES}",s_table_hashes)
	s = s.replace(r"${TYPE}",value_type)
	s = s.replace(r"${TO_STRING}",to_string)
	s = s.replace(r"${NAME}",data["general"]["name"])
	return s

def main():
	global data
	if len(sys.argv)!=2:
		print("Usage: ini_parser_builde.py <ini_file>")
		return
	if LoadIni(sys.argv[1])==False:
		return
	devider = ComputeHashDevider(data["list"])
	Info("Devider  : "+str(devider))
	Info("Elements : "+str(len(data["list"])))
	Info("Name     : "+data["general"]["name"])
	code = BuildCode(devider)
	try:
		open(sys.argv[1]+".rs","wt").write(code)
		Info("Save file: OK")
	except:
		Error("Fail to create file: "+sys.argv[1]+".rs")
		return

main()