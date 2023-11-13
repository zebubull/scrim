# This file scrapes http://dnd5e.wikidot.com for a list of all 5e spells
# It is not the fastest but it works, just trust me on this one

from bs4 import BeautifulSoup as soupme
import urllib.request as req
import json
r = req.urlopen('http://dnd5e.wikidot.com/spells')

soup = soupme(r.read().decode('utf8'), features='lxml')
r.close()

links = [x['href'] for x in soup.find_all('a', href=True) if x.parent.name == 'td']

stub = 'http://dnd5e.wikidot.com'

def get_stuff(link):
    r = req.urlopen(stub + link)
    soup = soupme(r.read().decode('utf8'), features='lxml')
    r.close()

    name = soup.find('div', 'page-header').string
    content = soup.find('div', id='page-content')
    current = next(content.children)
    first = current
    tags = []
    
    while current.next_sibling is not None:
        if "\\n" not in repr(current) and "div" not in repr(current):
            tags.append(current)
        current = current.next_sibling

    source = tags[0].string
    short = tags[1].string
    info = repr(tags[2]).replace('<p>', '').replace('</p>', '').replace('<strong>', '').replace('</strong>', '').replace('<br/>', '')
    lists = ' '.join(list(tags.pop().strings))
    btags = tags[3::]
    body = []
    for tag in btags:
        body.append(''.join(list(tag.strings)))
    body = '\n\n'.join(body)

    fshort = f'{source}\n{short}\n'
    final = f'{info}\n\n{body}\n\n{lists}'.replace('\n\n\n\n', '\n\n').replace('\n\n\n', '\n\n').replace('   ', ' ').replace('  ', ' ').replace(' ,', ',')
    
    return { 'name': name, 'description_short': fshort, 'description': final }

entries = { 'entries': {} }
print('')
for i, link in enumerate(links, 1):
    print('\r', end='')
    print(f'{i} of {len(links)}', end='')
    stuff = get_stuff(link)
    spells['entries'][stuff['name'].lower()] = stuff

with open('spells.json', 'w') as f:
    f.write(json.dumps(spells, indent=2))

