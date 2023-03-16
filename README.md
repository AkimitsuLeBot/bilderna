# Bilderna

<h4 align="center">
    <a href="https://discord.gg/RMSZ5MY" > Discord </a>
</h4>

<div align="center">    
<a href="https://top.gg/bot/471749111125770250">
    <img src="https://top.gg/api/widget/status/471749111125770250.svg" alt="Akimitsu" />
</a>
<a href="https://top.gg/bot/471749111125770250">
    <img src="https://top.gg/api/widget/servers/471749111125770250.svg" alt="Akimitsu" />
</a>
</div>

Bilderna is the service that takes care of generating the pictures for the Akimitsu bot.

It comes from the Swedish, and means "the pictures" (original, eh ?)

When starting, it will listen for any incoming requests on `0.0.0.0:3000`

## Entrypoints

Valid class names are:
- ARCHER
- KNGIHT
- MAGE
- TRAVELER

Valid city names are:
- Hamnen
- Kvarnen
- Herrgården
- Slotet
- Jägarens
- Timmerjacka
- Ängen
- Havetgrandsen
- Oas
- Ökenstad

The generated map will be different based on the local time (UTC 0), and the season.
There is a winter map, and a summer map.
There is a night map and a day map.

**POST: /in_city**
```json5
{
  "origin" : "Timmerjacka", // Valid city name
  "class" : "KNIGHT"// valid class name
}
```

**POST: /traveling**
```json5
{
  "origin": "Hamnen", // Valid city name
  "destination": "Kvarnen",// Valid city name
  "progress": 0, //number between 0 & 100,
  "class": "ARCHER", // Valid class name
}
```