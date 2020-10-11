f = open('practice.txt', 'w+') # Create file
f.write("Testing 1,2,3...")
f.close()

import os
print(os.getcwd())     # Current working dir (base dir with venv)
print(os.listdir())    # List everything in current dir
#os.listdir('../basics')

import shutil
shutil.move('practice.txt', os.getcwd() + '/src/modules/')

# Deleting files
os.unlink(os.getcwd() + '/src/modules/practice.txt')

# os.rmdir(path) - deletes dir, must be empty
# shutil.rmtree(path) - nukes dir

# Walk is a generator of tuples
for folder,sub_folders,files in os.walk(os.getcwd() + "/src"):
    print(f"Currently looking at {folder}")
    print(f"Subfolders are {sub_folders}")
    print(f"Files in this dir are {files}")