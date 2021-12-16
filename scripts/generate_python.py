import argparse

import hypothesmith
from hypothesis import HealthCheck, given, settings

generated_source = []

@settings(max_examples=1, suppress_health_check=[HealthCheck.return_value])
@given(hypothesmith.from_grammar(start="single_input"))
def generate(source):
    print(source)
    generated_source.append(source)


def main():
    """generate example scripts for consumption by other processes"""
    parser = argparse.ArgumentParser()
    parser.add_argument('-o', '--outfile')
    args = parser.parse_args()

    # @given will call the decorated function for us, meaning all values are
    # already generated by the end of function call but None is returned because
    # it may have been called multiple times
    generate()
    for some_python in generated_source:
        if args.outfile:
            with open(args.outfile, 'w') as out:
                out.write(some_python)
        else:
            print(some_python)


main()