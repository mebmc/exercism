"""Module for testing for leap years"""


def is_leap_year(year):
    """"is_leap_year returns true if provided year is a leap year and false otherwise."""
    if year % 400 == 0:
        return True
    elif year % 100 == 0:
        return False
    elif year % 4 == 0:
        return True
    return False