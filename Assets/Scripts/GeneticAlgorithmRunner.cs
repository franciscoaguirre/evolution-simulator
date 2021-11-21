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
    public GameObject m_creaturePrefab;
    public Vector3 m_lastPosition;

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
        m_lastPosition = new Vector3(0f, 1f, 0f);
    }

    protected override void UpdateSample()
    {
        while (m_fitness.ChromosomesToEndEvaluation.Count > 0)
        {
            CreatureChromosome chromosome;
            m_fitness.ChromosomesToEndEvaluation.TryTake(out chromosome);

            chromosome.Evaluated = true;
        }

        while (m_fitness.ChromosomesToBeginEvaluation.Count > 0)
        {
            CreatureChromosome chromosome;
            m_fitness.ChromosomesToBeginEvaluation.TryTake(out chromosome);
            chromosome.Evaluated = false;
            chromosome.MaxDistance = 0;

            var creature = (GameObject) Instantiate(m_creaturePrefab);

            var creatureController = creature.GetComponent<CreatureController>();
            creature.transform.position = m_lastPosition;

            m_lastPosition += Vector3.forward * 2f;

            creatureController.SetChromosome(chromosome, m_creatureSampleConfig); // TODO: Add config
        }
    }
}
