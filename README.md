# Primes
A quick way to generate primes


## Usage
`<n>` -> prime numbers up to limit n
### Options 
`-npb`-> removes the progress bar; usefull in combination with `>` to print to file
\
`-h`-> help text

## Performance

The algorithm seems to perform in general at O(n) speed. With some numbers wildly out of the trend, figure 2 shows the full calculation time (in seconds). One can see how some calculations take drastically more time. Removing these results in figure 1, shows what seems to be a linear time graph for finding primes. (Note this was generated using --release and with n = 10_000_000)

![Figure_1](https://user-images.githubusercontent.com/84490604/196062670-9174520b-aca6-4749-a517-b0d285007088.png)
\
Figure 1

![Figure_2](https://user-images.githubusercontent.com/84490604/196062600-81317ac3-ef41-4ce8-ab64-0d5b521b3578.png)
\
Figure 2
