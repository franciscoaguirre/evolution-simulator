using GeneticSharp.Domain.Terminations;
using GeneticSharp.Domain;
using UnityEngine;

public class CreatureTermination : TerminationBase
{
    private float m_lastFitness;
    private int m_stagnantGenerationsCount;

    protected override bool PerformHasReached(IGeneticAlgorithm geneticAlgorithm)
    {
        var ga = geneticAlgorithm as GeneticAlgorithm;

        var bestFitness = geneticAlgorithm.BestChromosome.Fitness.Value;

        if (bestFitness <= m_lastFitness)
        {
            m_stagnantGenerationsCount += 1;
        }
        else
        {
            m_stagnantGenerationsCount = 1;
        }

        m_lastFitness = (float) bestFitness;

        foreach (var chromosome in ga.Population.CurrentGeneration.Chromosomes)
        {
            chromosome.Fitness = null;
        }

        return m_stagnantGenerationsCount >= 50;
    }
}
