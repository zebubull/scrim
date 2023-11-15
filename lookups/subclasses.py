from bs4 import BeautifulSoup
import urllib.request as req
import json

urls = [
    'https://dnd5e.wikidot.com/artificer:alchemist',
    'https://dnd5e.wikidot.com/artificer:armorer',
    'https://dnd5e.wikidot.com/artificer:artillerist',
    'https://dnd5e.wikidot.com/artificer:battle-smith',
    'https://dnd5e.wikidot.com/barbarian:battlerager',
    'https://dnd5e.wikidot.com/barbarian:beast',
    'https://dnd5e.wikidot.com/barbarian:berserker',
    'https://dnd5e.wikidot.com/barbarian:giant',
    'https://dnd5e.wikidot.com/barbarian:storm-herald',
    'https://dnd5e.wikidot.com/barbarian:totem-warrior',
    'https://dnd5e.wikidot.com/barbarian:wild-magic',
    'https://dnd5e.wikidot.com/barbarian:zealot',
    'https://dnd5e.wikidot.com/bard:creation',
    'https://dnd5e.wikidot.com/bard:eloquence',
    'https://dnd5e.wikidot.com/bard:glamour',
    'https://dnd5e.wikidot.com/bard:lore',
    'https://dnd5e.wikidot.com/bard:spirits',
    'https://dnd5e.wikidot.com/bard:swords',
    'https://dnd5e.wikidot.com/bard:valor',
    'https://dnd5e.wikidot.com/bard:whispers',
    'https://dnd5e.wikidot.com/cleric:arcana',
    'https://dnd5e.wikidot.com/cleric:death',
    'https://dnd5e.wikidot.com/cleric:forge',
    'https://dnd5e.wikidot.com/cleric:grave',
    'https://dnd5e.wikidot.com/cleric:knowledge',
    'https://dnd5e.wikidot.com/cleric:life',
    'https://dnd5e.wikidot.com/cleric:light',
    'https://dnd5e.wikidot.com/cleric:nature',
    'https://dnd5e.wikidot.com/cleric:order',
    'https://dnd5e.wikidot.com/cleric:peace',
    'https://dnd5e.wikidot.com/cleric:tempest',
    'https://dnd5e.wikidot.com/cleric:trickery',
    'https://dnd5e.wikidot.com/cleric:twilight',
    'https://dnd5e.wikidot.com/cleric:war',
    'https://dnd5e.wikidot.com/cleric:ambition',
    'https://dnd5e.wikidot.com/cleric:solidarity',
    'https://dnd5e.wikidot.com/cleric:strength',
    'https://dnd5e.wikidot.com/cleric:zeal',
    'https://dnd5e.wikidot.com/druid:dreams',
    'https://dnd5e.wikidot.com/druid:land',
    'https://dnd5e.wikidot.com/druid:moon',
    'https://dnd5e.wikidot.com/druid:shepherd',
    'https://dnd5e.wikidot.com/druid:spores',
    'https://dnd5e.wikidot.com/druid:stars',
    'https://dnd5e.wikidot.com/druid:wildfire',
    'https://dnd5e.wikidot.com/fighter:arcane-archer',
    'https://dnd5e.wikidot.com/fighter:banneret',
    'https://dnd5e.wikidot.com/fighter:battle-master',
    'https://dnd5e.wikidot.com/fighter:cavalier',
    'https://dnd5e.wikidot.com/fighter:champion',
    'https://dnd5e.wikidot.com/fighter:echo-knight',
    'https://dnd5e.wikidot.com/fighter:eldritch-knight',
    'https://dnd5e.wikidot.com/fighter:psi-warrior',
    'https://dnd5e.wikidot.com/fighter:rune-knight',
    'https://dnd5e.wikidot.com/fighter:samurai',
    'https://dnd5e.wikidot.com/fighter:battle-master:maneuvers',
    'https://dnd5e.wikidot.com/monk:mercy',
    'https://dnd5e.wikidot.com/monk:ascendant-dragon',
    'https://dnd5e.wikidot.com/monk:astral-self',
    'https://dnd5e.wikidot.com/monk:drunken-master',
    'https://dnd5e.wikidot.com/monk:four-elements',
    'https://dnd5e.wikidot.com/monk:kensei',
    'https://dnd5e.wikidot.com/monk:long-death',
    'https://dnd5e.wikidot.com/monk:open-hand',
    'https://dnd5e.wikidot.com/monk:shadow',
    'https://dnd5e.wikidot.com/monk:sun-soul',
    'https://dnd5e.wikidot.com/monk:four-elements:disciplines',
    'https://dnd5e.wikidot.com/sorcerer:aberrant-mind',
    'https://dnd5e.wikidot.com/sorcerer:clockwork-soul',
    'https://dnd5e.wikidot.com/sorcerer:draconic-bloodline',
    'https://dnd5e.wikidot.com/sorcerer:divine-soul',
    'https://dnd5e.wikidot.com/sorcerer:lunar-sorcery',
    'https://dnd5e.wikidot.com/sorcerer:shadow-magic',
    'https://dnd5e.wikidot.com/sorcerer:storm-sorcery',
    'https://dnd5e.wikidot.com/sorcerer:wild-magic',
    'https://dnd5e.wikidot.com/sorcerer:pyromancy',
    'https://dnd5e.wikidot.com/warlock:archfey',
    'https://dnd5e.wikidot.com/warlock:celestial',
    'https://dnd5e.wikidot.com/warlock:fathomless',
    'https://dnd5e.wikidot.com/warlock:fiend',
    'https://dnd5e.wikidot.com/warlock:the-genie',
    'https://dnd5e.wikidot.com/warlock:great-old-one',
    'https://dnd5e.wikidot.com/warlock:hexblade',
    'https://dnd5e.wikidot.com/warlock:undead',
    'https://dnd5e.wikidot.com/warlock:undying',
    'https://dnd5e.wikidot.com/warlock:eldritch-invocations',
    'https://dnd5e.wikidot.com/wizard:abjuration',
    'https://dnd5e.wikidot.com/wizard:bladesinging',
    'https://dnd5e.wikidot.com/wizard:chronurgy',
    'https://dnd5e.wikidot.com/wizard:conjuration',
    'https://dnd5e.wikidot.com/wizard:divination',
    'https://dnd5e.wikidot.com/wizard:enchantment',
    'https://dnd5e.wikidot.com/wizard:evocation',
    'https://dnd5e.wikidot.com/wizard:graviturgy',
    'https://dnd5e.wikidot.com/wizard:illusion',
    'https://dnd5e.wikidot.com/wizard:necromancy',
    'https://dnd5e.wikidot.com/wizard:order-of-scribes',
    'https://dnd5e.wikidot.com/wizard:transmutation',
    'https://dnd5e.wikidot.com/wizard:war-magic',
]

def get_text(tag, join=''):
    text = tag.string
    if text is None:
        text = join.join(list(tag.strings))
    return text

def parse_ul(tag):
    texts = []
    node = next(tag.children)
    while node is not None:
        if node.name == 'li':
            texts.append(f'  -- {get_text(node)}\n\n')
        node = node.next_sibling
    return texts

def parse_tr(tag):
    node = tag.td
    text = f'  -- {get_text(node)}'
    node = node.next_sibling
    if node is not None:
        text += ':'
    while node is not None:
        text += f' {get_text(node)}'
        node = node.next_sibling
    return text.replace('\n', '') + '\n'
    

def parse_table(tag):
    node = next(tag.children).next_sibling
    title = get_text(node).replace("\n", "")
    texts = [f'{title}\n']
    while node is not None:
        if node.name == 'tr' and node.th is None:
            texts.append(parse_tr(node))
        node = node.next_sibling
    texts.append('\n')
    return texts

def page_to_lines(soup):
    title = soup.find('div', 'page-title').string
    content = soup.find(id='page-content')
    description = get_text(content.p)

    body = content.div.div.div
    # Skip the source text
    current_node = next(body.children).next_sibling
    source = get_text(current_node)
    current_node = current_node.next_sibling

    data = []
    while current_node is not None:
        if current_node.name == 'h3':
            text = get_text(current_node)
            data.append(f'{text}\n\n')
        elif current_node.name == 'p':
            text = get_text(current_node)
            data.append(f'{text}\n\n')
        elif current_node.name == 'ul':
            data.extend(parse_ul(current_node))
        elif current_node.name == 'table':
            data.extend(parse_table(current_node))

        current_node = current_node.next_sibling
    
    return { 'name': title, 'description_short': f'{source}\n\n{description}\n\n', 'description': ''.join(data) }

def main():
    entries = { 'entries': {} }
    for i, url in enumerate(urls, 1):
        print(f'\r{i} of {len(urls)}', end='')
        r = req.urlopen(url)

        soup = BeautifulSoup(r.read().decode('utf8'), features='lxml')
        r.close()
        try:
            entry = page_to_lines(soup)
            entries['entries'][entry['name'].lower()] = entry
        except:
            print(f'{url} failed...')
    print('')
    with open('class_dump.json', 'w') as f:
        f.write(json.dumps(entries, indent=2))

if __name__ == "__main__":
    main()
