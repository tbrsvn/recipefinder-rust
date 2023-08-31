# Recipe Finder
A simple Rust program that finds recipes with the ingredients you provide

This program requires you to create your own JSON called recipes.json, as I'm not sure if I can distribute mine.
The format for each recipe should go something like this:
```
{
    "Recipe Name": {
            "directions": [
                "some",
                "stuff"
            ],
            "ingredients": [
                "Yummy",
                "Things"
            ],
            "language": "en-US",
            "source": "TheWebsite.com",
            "title": "Recipe Name",
            "url": "TheWebsite.com/bestrecipe"
             
    },
    "Another Recipe": {
    ....json cont
}
```
