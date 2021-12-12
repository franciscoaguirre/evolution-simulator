from scipy import stats
from pathlib import Path
import itertools

configs = [
  "population_100_mutation_0.1_crossover_0.6",
  "population_100_mutation_0.1_crossover_0.9",
  "population_100_mutation_0.01_crossover_0.6",
  "population_100_mutation_0.01_crossover_0.9",
]

def main():
  average = {}
  for config in configs:
    average[config] = []
    for i in range(1, 31):
        path = Path(f"{config}/instance_{i}/new_stats.txt")
        with path.open() as f:
            average[config].append(process_line(f))

  for (key_1, key_2) in itertools.combinations(average.keys(), 2):
    print(f"{key_1} vs {key_2}")
    # print(stats.ttest_rel(average[key_1], average[key_2]))
    print(stats.mannwhitneyu(average[key_1], average[key_2], method="exact"))
    print(stats.mannwhitneyu(average[key_1], average[key_2], alternative="less", method="exact"))
    print(stats.mannwhitneyu(average[key_2], average[key_1], alternative="less", method="exact"))

  # ks_p = stats.mannwhitneyu(average, 'norm')
  # print(f"Instance {i}: {ks_p=}")

def process_line(f):
    return float(f.readline().split("=")[-1].strip())

if __name__ == "__main__":
    main()
