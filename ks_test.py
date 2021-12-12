from scipy import stats
from pathlib import Path

def main():
  average = []
  for i in range(1, 31):
      path = Path(f"instance_{i}/new_stats.txt")
      with path.open() as f:
          average.append(process_line(f))

  ks_p = stats.kstest(average, 'norm')
  print(f"Instance {i}: {ks_p=}")

def process_line(f):
    return float(f.readline().split("=")[-1].strip())

if __name__ == "__main__":
    main()
