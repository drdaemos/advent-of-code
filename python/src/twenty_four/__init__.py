from argparse import ArgumentParser, Namespace
from . import day_05, day_06, day_07, day_08, day_09, day_10, day_11, day_12, day_13, day_14, day_15, day_16, day_17, day_18, day_19, day_20, day_21, day_22, day_23, day_24

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
        day_20.main,
        day_21.main,
        day_22.main,
        day_23.main,
        day_24.main,
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
