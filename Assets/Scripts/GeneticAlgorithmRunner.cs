using GeneticSharp.Domain;
using GeneticSharp.Domain.Crossovers;
using GeneticSharp.Domain.Mutations;
using GeneticSharp.Domain.Populations;
using GeneticSharp.Domain.Selections;
using GeneticSharp.Domain.Terminations;
using GeneticSharp.Runner.UnityApp.Commons;
using UnityEngine;

public class GeneticAlgorithmRunner : SampleControllerBase
{
    public float timeScale = 1;
    private CreatureSampleConfig m_creatureSampleConfig;

    protected override GeneticAlgorithm CreateGA()
    {
        // TODO: Instantiate Fitness
        m_creatureSampleConfig = new CreatureSampleConfig();
        var chromosome = new CreatureChromosome(m_carSampleConfig);
        var crossover = new CreatureCrossover();
        var mutation = new FlipBitMutation();
        var selection = new RouletteWheelSelection();
        var population = new Population();
    }
}
