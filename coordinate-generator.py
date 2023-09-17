import csv
import random

# Specify file name
filename = "coordinates.csv"

# Open file for writing
with open(filename, mode='w', newline='') as file:
    writer = csv.writer(file)

    # Write the header
    writer.writerow(["x", "y"])

    # Generate and write 50 random floating-point coordinates
    for _ in range(2000):
        x = random.uniform(-300.0, 300.0)  # Generates a random float between -100 and 100
        y = random.uniform(-300.0, 300.0)
        writer.writerow([x, y])

print(f"{filename} has been generated!")
