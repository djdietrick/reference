f = open('file1.txt', 'w+')
f.write('One file')
f.close()

f = open('file2.txt', 'w+')
f.write('Two file')
f.close()

import zipfile

comp_file = zipfile.ZipFile('comp_file.zip','w')
comp_file.write('file1.txt', compress_type=zipfile.ZIP_DEFLATED)
comp_file.write('file2.txt', compress_type=zipfile.ZIP_DEFLATED)
comp_file.close()

zip_obj = zipfile.ZipFile('comp_file.zip', 'r')
zip_obj.extractall('extracted_content')
zip_obj.close()


import os
import shutil

dir_to_zip = os.getcwd() + '\\extracted_content'
output_filename = 'example'

shutil.make_archive(output_filename, 'zip', dir_to_zip)
shutil.unpack_archive('example.zip', 'final_unzip', 'zip')

os.unlink('file1.txt')
os.unlink('file2.txt')
os.unlink('comp_file.zip')
os.unlink('example.zip')
shutil.rmtree('extracted_content')
shutil.rmtree('final_unzip')