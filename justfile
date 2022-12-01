get-input day:
    mkdir -p input
    xh https://adventofcode.com/2022/day/{{day}}/input Cookie:session=$SESSION \
        --output input/day$(printf "%02d" {{day}}).txt
