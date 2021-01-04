import os
import berserk
from game_client import Game
import argparse
parser = argparse.ArgumentParser(description="Client for rc2d2")
parser.add_argument("--depth", default=4, help="Sets the search depth")
args = parser.parse_args() 

token = os.getenv('API_TOKEN')
bot_id = os.getenv('BOT_ID')
DEPTH = int(args.depth)

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

