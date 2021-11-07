using GeneticSharp.Domain;
using GeneticSharp.Domain.Crossovers;
using GeneticSharp.Domain.Mutations;
using GeneticSharp.Domain.Populations;
using GeneticSharp.Domain.Selections;
using GeneticSharp.Domain.Terminations;
using GeneticSharp.Runner.UnityApp.Commons;
using GeneticSharp.Infrastructure.Framework.Threading;
using UnityEngine;

public class GeneticAlgorithmRunner : SampleControllerBase
{
    public float timeScale = 1;
    private CreatureSampleConfig m_creatureSampleConfig;
    private int numberOfSimultaneousExecutions = 100;
    private CreatureFitness m_fitness;
    private PrefabPool m_prefabPool;
    public GameObject evaluationPrefab;

    protected override GeneticAlgorithm CreateGA()
    {
        m_fitness = new CreatureFitness(15f);
        m_creatureSampleConfig = new CreatureSampleConfig();
        var chromosome = new CreatureChromosome(m_creatureSampleConfig);
        var crossover = new CreatureCrossover();
        var mutation = new FlipBitMutation();
        var selection = new RouletteWheelSelection();
        var population = new Population(numberOfSimultaneousExecutions, numberOfSimultaneousExecutions, chromosome);
        var ga = new GeneticAlgorithm(population, m_fitness, selection, crossover, mutation);
        ga.Termination = new CreatureTermination();
        ga.TaskExecutor = new ParallelTaskExecutor
        {
            MinThreads = population.MinSize,
            MaxThreads = population.MaxSize,
        };
        return ga;
    }

    protected override void StartSample()
    {
        m_prefabPool = new PrefabPool(evaluationPrefab);
    }

    protected override void UpdateSample()
    {
        while (m_fitness.ChromosomesToEndEvaluation.Count > 0)
        {

        }
    }
}
