import time


def square(i):
    return i * i


def format_str(name, age):
    return name + "is" + str(square(age)) + "years"


start = time.time()
for i in range(1, 1_000_000):
    res = format_str("alfie", i)
    if i % 10000 == 0:
        print(res)
end = time.time()

print("python time: " + str(end - start))
# python time: 0.3362870216369629
