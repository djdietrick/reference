import requests
import bs4

base_url = 'http://books.toscrape.com/catalogue/page-{}.html'

res = requests.get(base_url.format(1))
soup = bs4.BeautifulSoup(res.text,'lxml')

two_stars = soup.select('.product_pod')[0].select('.star-rating.Two')
print(two_stars)