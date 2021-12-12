from pathlib import Path

def main():
    for i in range(1, 31):
        path = Path(f"instance_{i}")

        count = 0
        best = 0
        worst = 0
        median = 0
        average = 0
        standard_deviation = 0

        for file in path.iterdir():
            if file.name == "stats.ron" or file.name == "new_stats.txt":
                continue

            with file.open() as f:
                count += 1
                best += float(process_line(f))
                worst += float(process_line(f))
                median += float(process_line(f))
                average += float(process_line(f))
                standard_deviation += float(process_line(f))

        best /= count
        worst /= count
        median /= count
        average /= count
        standard_deviation /= count

        new_path = path / "new_stats.txt"
        new_path.touch()
        with new_path.open('w') as f:
            f.write(f"{best=}\n")
            f.write(f"{worst=}\n")
            f.write(f"{median=}\n")
            f.write(f"{average=}\n")
            f.write(f"{standard_deviation=}\n")
            f.write(f"{count=}\n")

        count = 0

def process_line(f):
    return f.readline().split(": ")[-1].strip()

if __name__ == "__main__":
    main()
