from argparse import ArgumentParser, Namespace
from . import day_05
from . import day_06
from . import day_07
from . import day_08
from . import day_09
from . import day_10
from . import day_11
from . import day_12
from . import day_13
from . import day_14
from . import day_15
from . import day_16
from . import day_17
from . import day_18
from . import day_19

def main():
    args = parse_args()
    starting = 5
    solutions = [
        day_05.main,
        day_06.main,
        day_07.main,
        day_08.main,
        day_09.main,
        day_10.main,
        day_11.main,
        day_12.main,
        day_13.main,
        day_14.main,
        day_15.main,
        day_16.main,
        day_17.main,
        day_18.main,
        day_19.main,
    ]

    if args.solution:
        day_number = int(args.solution)
        if day_number < starting or len(solutions) + starting - 1 < day_number:
            raise Exception("No solution for day {args.solution}")
        
        solutions[day_number - starting](args.debug)
    else:
        for solution in solutions:
            solution(args.debug)

def parse_args() -> Namespace:
    parser = ArgumentParser()
    parser.add_argument("-n", "--solution", dest="solution",
                        help="run only solution for the specific day")
    parser.add_argument("-d", "--debug",
                        action="store_true", dest="debug",
                        help="output debug information")

    return parser.parse_args()

if __name__ == "__main__":
    main()
