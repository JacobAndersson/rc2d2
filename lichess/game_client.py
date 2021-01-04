import threading
import random
import chess
import rc2d2

class Game(threading.Thread):
    def __init__(self, client, game_id, bot_id, depth, **kwargs):
        super().__init__(**kwargs)
        self.bot_id = bot_id
        self.game_id = game_id
        self.client = client
        self.stream = client.bots.stream_game_state(game_id)
        self.current_state = next(self.stream)
        self.depth = depth

        self.is_white = True

    def run(self):
        if self.current_state:
            try:
                self.is_white = self.current_state['white']['id'] == self.bot_id
            except KeyError:
                self.is_white = False

            state = self.current_state['state']
            self.handle_state_change(state)


        for event in self.stream:
            if 'winner' in event:
                break

            if event['type'] == 'gameState':
                self.handle_state_change(event)
    
    def handle_state_change(self, game_state):
        all_moves = game_state['moves']
        num_moves = len(all_moves.split(' '))
        whites_turn = num_moves % 2 == 0
        if whites_turn == self.is_white or (self.is_white and len(all_moves) == 0):
            move = rc2d2.find_best_move(all_moves, self.depth)
            try: 
                self.client.bots.make_move(self.game_id, move)
            except TypeError:
                pass
