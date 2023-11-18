cargo install --path .

mkdir -p ~/.scrim
cp lookups/spells.json ~/.scrim/spells.json
cp lookups/weapons.json ~/.scrim/weapons.json
cp lookups/classes.json ~/.scrim/classes.json
cp lookups/races.json ~/.scrim/races.json
cp lookups/subclasses.json ~/.scrim/subclasses.json

echo "installed successfully"
