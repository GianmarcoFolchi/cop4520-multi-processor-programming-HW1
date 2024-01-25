This program relies on maintaining mutual exclusivity for the primes_counter variable, ensuring that only one thread accesses it at a time. To achieve this, I implemented a strategy where a single variable (primes_counter) keeps track of the prime number to be checked. I then created eight threads, each responsible for incrementing this variable while checking its primality.

I chose this approach of incrementing the primes_counter variable over assigning each thread a specific range of numbers to check. This decision aims to evenly distribute the workload among threads, making it an efficient way to tackle the problem. Each thread increments the primes_counter by one, ensuring that the task of checking each number is evenly distributed across the threads.

The primary bottleneck in this design is the is_prime function. To ensure accurate results, I opted for a deterministic function instead of alternatives like the Miller-Rabin primality test.

In my own experimentation, I observed that my machine (an M1 MacBook Air) takes approximately 30 seconds to complete this task. I anticipate that a more powerful system would likely yield improved performance.