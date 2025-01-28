# Interzic configuration

Interzic is the translation layer between different music providers, services, or even apps (ex: Youtube, Spotify, listenbrainz, Tauon, etc...). However, many services requires API keys to function. Here's how to set them:

## Youtube

Youtube integration requires creating a google cloud app [here](https://console.cloud.google.com/apis/dashboard). This app needs the `YouTube API v3` service. 

After obtaining the credentials, you will need to copy the json file as: {[config_root](./config_root.md)}/youtube_credentials.json

