def hello():
    print("Hello, ", end="")
    yield
    print("world!")
    yield

h = hello()
h.__next__()
h.__next__()
