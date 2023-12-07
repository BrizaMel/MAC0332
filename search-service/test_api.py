import requests

url = "http://localhost:3000/search"
req = {"projection":["movies.movie.title","movies.movie.revenue"],"filters":"movies.movie.runtime gt 200"}

resp = requests.post(url, json = req)

print(resp.text)