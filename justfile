get-input day:
    mkdir -p testdata/day_$(printf "%02d" {{day}})/sample
    mkdir -p testdata/day_$(printf "%02d" {{day}})/puzzle
    xh https://adventofcode.com/2022/day/{{day}}/input Cookie:session=$SESSION \
        --output testdata/day_$(printf "%02d" {{day}})/puzzle/input.txt
