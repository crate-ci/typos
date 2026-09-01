import os

from numpy.typing import NDArray  # should work

print(os.O_WRONLY)  # should work


async def consume(generator):
    await generator.asend(None)  # should work
