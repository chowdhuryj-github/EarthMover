import gymnasium
import gymnasium_env

# Initialize environment with human render mode
env = gymnasium.make('gymnasium_env/GridWorld-v0', render_mode="human")

# Reset environment and start interacting
observation, info = env.reset()

for _ in range(100):  # Run for 100 steps
    env.render()  # Render the environment
    action = env.action_space.sample()  # Sample a random action
    observation, reward, done, _, info = env.step(action)  # Take a step
    if done:  # If done, reset the environment
        env.reset()

env.close()  # Close the environment and the pygame window after running
