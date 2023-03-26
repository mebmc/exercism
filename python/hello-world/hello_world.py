#
# Skeleton file for the Python "Hello World" exercise.
#


def hello(name=''):
    """Print hello message."""
    if not name:
        name = "World"
    return "Hello, {}!".format(name)