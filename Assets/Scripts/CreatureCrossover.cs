using System;
using System.Collections.Generic;
using GeneticSharp.Domain.Crossovers;
using GeneticSharp.Domain.Chromosomes;
using GeneticSharp.Domain.Randomizations;

public class CreatureCrossover : CrossoverBase
{
    public CreatureCrossover(float mixProbability = 0.6f) : base(2, 2)
    {
        MixProbability = mixProbability;
    }

    public float MixProbability { get; set; }

    protected override IList<IChromosome> PerformCross(IList<IChromosome> parents)
    {
        var firstParent = parents[0] as CreatureChromosome;
        var secondParent = parents[1] as CreatureChromosome;
        var firstChild = firstParent.CreateNew() as CreatureChromosome;
        var secondChild = secondParent.CreateNew() as CreatureChromosome;

        int nodeCount = Math.Min(firstParent.NodeCount, secondParent.NodeCount);

        for (int i = 0; i < nodeCount; i++)
        {
            if (RandomizationProvider.Current.GetDouble() < MixProbability)
            {
                firstChild.ReplaceGene(i, firstParent.GetGene(i));
                secondChild.ReplaceGene(i, secondParent.GetGene(i));
            }
            else
            {
                firstChild.ReplaceGene(i, secondParent.GetGene(i));
                secondChild.ReplaceGene(i, firstParent.GetGene(i));
            }
        }

        for (int i = 0; i < Math.Min(firstParent.MuscleCount, secondParent.MuscleCount); i++)
        {
            if (RandomizationProvider.Current.GetDouble() < MixProbability)
            {
                firstChild.ReplaceGene(i + firstChild.NodeCount, firstParent.GetGene(i + firstParent.NodeCount));
                secondChild.ReplaceGene(i + secondChild.NodeCount, secondParent.GetGene(i + secondParent.NodeCount));
            }
            else
            {
                firstChild.ReplaceGene(i + firstChild.NodeCount, secondParent.GetGene(i + secondParent.NodeCount));
                secondChild.ReplaceGene(i + secondChild.NodeCount, firstParent.GetGene(i + firstParent.NodeCount));
            }
        }

        return new List<IChromosome> { firstChild, secondChild };
    }
}
