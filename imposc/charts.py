from time import time_ns
from typing import List, Tuple, Union
from pathlib import Path
from matplotlib import pyplot as plt

def unique_file_name(extension: str) -> str:
    """ Returns a unique file name with the specified `extension` """
    return f"{time_ns()}.{extension}"

def files_size(files: List[Tuple[Union[str, Path], int]]) -> int:
    """ Sums the sizes from a supplied list of file sizes """
    return sum([x[1] for x in files])

def scatter_plot(data):
    x, y = zip(*map(lambda impact: (impact.phase(), impact.velocity()), data))

    file_name = unique_file_name("png")
    plt.plot(x, y, linestyle='', marker='.', markersize=1, mec='black', mfc='black')
    plt.savefig(file_name)

    return file_name
