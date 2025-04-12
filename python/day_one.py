# Day 2: Exploring Python basics
def hello():
    print("hello")

# Call hello function
hello()

# Get user name
print("Hello, what is your name?")
name = input()
print(f"Hello, {name}!")
print(f"Your name has {len(name)} characters")

# Weather and name check
rain = True
secondname = name
if rain:
    if name == "Omar" and secondname == "Omar":
        print("Hello, Omar OMar!")
    print("Rain")
else:
    print("No rain")

# Age loops
age = 10
while age < 20:
    print("Counting up to 20...")
    age += 1
    print(age)
    break  # Stops after one iteration
while age >= 20:
    print("Counting from 20...")
    age += 1
    print(age)
    if age == 30:
        print("Reached 30!")
        break
    print(age)

# For loop
for i in range(1, 10):
    print(i)

# Exit program
import sys
while True:
    print("Type 'exit' to exit.")
    response = input()
    if response == "exit":
        sys.exit()
    print(f"You typed {response}.")