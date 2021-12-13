from pathlib import Path

def main():
    best = 0
    worst = 0
    median = 0
    average = 0
    standard_deviation = 0
    count = 0
    for i in range(1, 31):
        path = Path(f"execution_{i}/new_stats.txt")
        with path.open() as f:
            best += process_line(f)
            worst += process_line(f)
            median += process_line(f)
            average += process_line(f)
            standard_deviation += process_line(f)
            count += process_line(f)
    best /= 30
    worst /= 30
    median /= 30
    average /= 30
    standard_deviation /= 30
    count /= 30
    print(f"{best=}");
    print(f"{worst=}");
    print(f"{median=}");
    print(f"{average=}");
    print(f"{standard_deviation=}");
    print(f"{count=}");

def process_line(f):
    return float(f.readline().split("=")[-1].strip())

if __name__ == "__main__":
    main()
