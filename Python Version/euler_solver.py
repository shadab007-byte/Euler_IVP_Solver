import math
import csv

# Define the ODE: dy/dt = cos(t) - y
def f(t, y):
    return math.cos(t) - y

# Euler method implementation
def euler_method(f, t0, y0, t_end, n):
    h = (t_end - t0) / n
    t_values = [t0]
    y_values = [y0]
    
    t = t0
    y = y0

    for _ in range(n):
        y = y + h * f(t, y)
        t = t + h
        t_values.append(t)
        y_values.append(y)
    
    return t_values, y_values

# Parameters
t0 = 0
y0 = 1
t_end = 5
n = 20

# Solve the IVP
t_vals, y_vals = euler_method(f, t0, y0, t_end, n)

# Save results to CSV
with open("results.csv", "w", newline="") as file:
    writer = csv.writer(file)
    writer.writerow(["t", "y"])
    for t, y in zip(t_vals, y_vals):
        writer.writerow([t, y])
