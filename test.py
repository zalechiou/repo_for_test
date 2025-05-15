import threading
import time
import numpy as np
import pyo3_multithread_demo

def py_heavy_computation():
    acc = 0
    for i in range(100_000_000):
        acc += i
    print(f"Python thread done: {acc}")

if __name__ == "__main__":
    print("1️⃣ Python multithreading test (GIL blocked)")
    start = time.time()
    threads = [threading.Thread(target=py_heavy_computation) for _ in range(4)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    print(f"Python total time: {time.time() - start:.2f}s\n")

    print("2️⃣ Rust multithreading test (GIL released)")
    start = time.time()
    pyo3_multithread_demo.rust_heavy_computation(4)
    print(f"Rust total time: {time.time() - start:.2f}s\n")

    print("3️⃣ Rust async I/O simulation")
    start = time.time()
    pyo3_multithread_demo.rust_async_io_simulation(4)
    print(f"Async I/O total time: {time.time() - start:.2f}s\n")

    print("4️⃣ NumPy array sum in Rust")
    arr = np.random.rand(1_000_000)
    print("Sum result:", pyo3_multithread_demo.sum_numpy_array(arr))
