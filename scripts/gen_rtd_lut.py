# SPDX-License-Identifier: LGPL-3.0-or-later
# SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>


def r_relative(t):
    """
    Calculate the relative resistance of an temperature

    divide actual resistance by 100 Ohm / 1000 Ohm before comparing with the
    values in the table
    """
    A = 3.9083e-3
    B = -5.775e-7
    C = -4.183e-12

    if t >= 0:
        r = 1 + A * t + B * t**2
    else:
        r = 1 + A * t + B * t**2 + C * (t - 100) * t**3

    return r


if __name__ == "__main__":
    t_min = -50
    t_max = 600
    t_step = 1

    print("[")

    for i, t in enumerate(range(t_min, t_max + t_step, t_step)):
        print("{:.5f}, ".format(r_relative(t)), end="")
        if i % 10 == 9:
            print()

    print()
    print("]")
    print()
