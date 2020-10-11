import requests
import bs4

result = requests.get('https://en.wikipedia.org/wiki/Jonas_Salk')

soup = bs4.BeautifulSoup(result.text,"lxml")

print(soup.select('title')[0].getText())
# soup.select('.some_class')
# soup.select('#some_id')

# Images
pic_element = soup.select('.thumbimage')[0]
print(pic_element['src'])

image_link = requests.get('http:' + pic_element['src'])

f = open('my_img.jpg', 'wb')
f.write(image_link.content)
f.close()

import os
os.unlink('my_img.jpg')