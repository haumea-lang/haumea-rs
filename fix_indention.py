import os
import os.path
for root, _, files in os.walk("src"):
    for file in files:
        if file[-2:] != "rs": continue
        content = open(os.path.join(root, file)).read()
        content = content.replace("\t", "    ")
        open(os.path.join(root, file), "w").write(content)
