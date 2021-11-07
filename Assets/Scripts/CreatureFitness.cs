using GeneticSharp.Domain.Fitnesses;
using System.Collections.Concurrent;
using GeneticSharp.Domain.Chromosomes;
using System.Threading;

public class CreatureFitness : IFitness
{
    private float m_secondsForEvaluation;

    public CreatureFitness(float secondsForEvaluation)
    {
        m_secondsForEvaluation = secondsForEvaluation;
        ChromosomesToBeginEvaluation = new BlockingCollection<CreatureChromosome>();
        ChromosomesToEndEvaluation = new BlockingCollection<CreatureChromosome>();
    }

    public BlockingCollection<CreatureChromosome> ChromosomesToBeginEvaluation { get; private set; }
    public BlockingCollection<CreatureChromosome> ChromosomesToEndEvaluation { get; private set; }

    public double Evaluate(IChromosome initialChromosome)
    {
        var chromosome = initialChromosome as CreatureChromosome;

        ChromosomesToBeginEvaluation.Add(chromosome);

        do
        {
            Thread.Sleep(1000);
            chromosome.Fitness = chromosome.MaxDistance;
        } while (!chromosome.Evaluated);

        ChromosomesToEndEvaluation.Add(chromosome);

        do
        {
            Thread.Sleep(100);
        } while (!chromosome.Evaluated);

        return chromosome.MaxDistance;
    }
}
