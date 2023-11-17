from bs4 import BeautifulSoup
import urllib.request as req
import json

url = 'http://dnd5e.wikidot.com/weapons'

def get_text(tag, join=''):
    text = tag.string
    if text is None:
        text = join.join(list(tag.strings))
    return text

def parse_tr(tag):
    node = tag.td
    title = get_text(node)
    text = ""
    node = node.next_sibling
    while node is not None:
        t = get_text(node)
        if len(t) > 2:
            text += f'{get_text(node)}\n\n'
        node = node.next_sibling
    return { "name": title, "description_short": "", "description": text}
    

def parse_table(tag):
    node = next(tag.children).next_sibling
    texts = {}
    while node is not None:
        if node.name == 'tr' and node.th is None:
            val = parse_tr(node)
            texts[val['name'].lower()] = val
        node = node.next_sibling
    return texts

def page_to_lines(soup):
    body = soup.find(id='page-content')

    current_node = next(body.children)
    
    entries = { 'entries': {} }
    while current_node is not None:
        if current_node.name == 'table':
            entries['entries'].update(parse_table(current_node))

        current_node = current_node.next_sibling
    
    return entries

def main():
    r = req.urlopen(url)

    soup = BeautifulSoup(r.read().decode('utf8'), features='lxml')
    r.close()
    entries = page_to_lines(soup)

    with open('weapon_dump.json', 'w') as f:
        f.write(json.dumps(entries, indent=2))

if __name__ == "__main__":
    main()
