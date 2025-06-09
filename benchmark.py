#!/usr/bin/env python3
import time

def fibonacci(n):
    if n <= 1:
        return n
    else:
        return fibonacci(n - 1) + fibonacci(n - 2)

def fibonacci_iter(n):
    if n <= 1:
        return n
    a, b, i = 0, 1, 2
    while i <= n:
        a, b = b, a + b
        i += 1
    return b

def is_prime(n):
    if n <= 1: return False
    if n <= 3: return True
    if n % 2 == 0: return False
    i = 3
    while i * i <= n:
        if n % i == 0: return False
        i += 2
    return True

def sum_numbers(limit):
    return sum(range(1, limit + 1))

# Benchmark tests
print("Python Benchmark Starting...")

tests = [
    ("Fibonacci(25) recursive", lambda: fibonacci(25)),
    ("Fibonacci(1000) iterative", lambda: fibonacci_iter(1000)),
    ("Prime check 982451653", lambda: is_prime(982451653)),
    ("Sum 1 to 100000", lambda: sum_numbers(100000))
]

for name, func in tests:
    start = time.time()
    result = func()
    end = time.time()
    print(f"{name}: {result} ({end-start:.6f}s)")