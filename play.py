import json
import requests
import random
import uuid

import cProfile
import re

BASE_URL = "http://localhost:8000"

def start_new_game():
    response = requests.post(f"{BASE_URL}/new_game")
    if response.status_code == 200:
        return response.json()
    else:
        print("Failed to start a new game")
        return None

def perform_action(game_id, action):
    response = requests.post(f"{BASE_URL}/perform_action/{game_id}", json=action)
    if response.status_code == 200:
        return response.json()
    else:
        print(f"Failed to perform action: {response.text}")
        return None

def get_legal_moves(game_id):
    response = requests.get(f"{BASE_URL}/legal_moves/{game_id}")
    if response.status_code == 200:
        return response.json()
    else:
        print(f"Failed to get legal moves: {response.text}")
        return None

def play_random_game():
    

    move_count = 0
    max_moves = 500
    for i in range(50):
        game = start_new_game()
        if not game:
            return
        
        print(game)

        game_id = game
        print(f"Started new game with ID: {game_id}")

        move_count = 0

        while move_count < max_moves:
            legal_moves = get_legal_moves(game_id)

            if not legal_moves:
                print("No legal moves available, ending game.")
                break
            
            action = random.choice(legal_moves)

            observation = perform_action(game_id, action)

            # print(observation['board'])

            if not game:
                print("Failed to perform action, ending game.")
                break

            if observation['winner'] != "Undecided":
                print("winner: ", observation['winner'])
                break
            
            # print(f"Performed action: {action}")
            
            move_count += 1
        print("next")

if __name__ == "__main__":
    cProfile.run('play_random_game()')
    # play_random_game()
