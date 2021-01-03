import os
import berserk
from game_client import Game

token = os.getenv('API_TOKEN')
bot_id = os.getenv('BOT_ID')
DEPTH = 4

session = berserk.TokenSession(token)
client = berserk.Client(session)

acceptChallenge = True

for event in client.bots.stream_incoming_events():
    if event['type'] == 'challenge':
        game_id = event['challenge']['id']
        challenge = event['challenge']

        if challenge['challenger']['id'] == bot_id:
            continue

        if acceptChallenge:
            client.bots.accept_challenge(game_id)
            acceptChallenge = False
        else:
            client.bots.decline_challenge(game_id)

    elif event['type'] == 'gameStart':
        game_id = event['game']['id']
        game = Game(client, game_id, bot_id, DEPTH)
        game.run()
    elif event['type'] == 'gameFinish':
        acceptChallenge = True

