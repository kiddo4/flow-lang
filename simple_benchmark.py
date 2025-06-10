#!/usr/bin/env python3
import time

print("Python Simple Benchmark Starting...")

# Test 1: Simple arithmetic loop
print("Test 1: Arithmetic loop (sum 1 to 100000)")
start = time.time()
sum_result = 0
i = 1
while i <= 100000:
    sum_result += i
    i += 1
end = time.time()
print(f"Sum result: {sum_result} ({end-start:.6f}s)")

# Test 2: Multiplication loop
print("Test 2: Multiplication loop")
start = time.time()
product = 1
j = 1
while j <= 20:
    product *= j
    j += 1
end = time.time()
print(f"Product result: {product} ({end-start:.6f}s)")

# Test 3: Nested loops
print("Test 3: Nested loops")
start = time.time()
total = 0
x = 1
while x <= 100:
    y = 1
    while y <= 100:
        total += 1
        y += 1
    x += 1
end = time.time()
print(f"Total iterations: {total} ({end-start:.6f}s)")

print("Python Simple Benchmark Complete!")