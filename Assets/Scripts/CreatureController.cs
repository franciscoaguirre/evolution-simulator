using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CreatureController : MonoBehaviour
{
    public CreatureChromosome Chromosome { get; private set; }

    public void SetChromosome(CreatureChromosome chromosome, CreatureSampleConfig config)
    {
        Chromosome = chromosome;
        Chromosome.MaxDistance = 0;

        // TODO: Some setup

        var phenotypes = chromosome.GetPhenotypes();
        
        for (int i = 0; i < chromosome.NodeCount; i++)
        {
            var phenotype = phenotypes[i];
            CreateNode(phenotype as NodePhenotypeEntity);
        }

        for (int i = 0; i < chromosome.MuscleCount; i++)
        {
            var phenotype = phenotypes[i];
            CreateMuscle(phenotype as MusclePhenotypeEntity);
        }

        // TODO: Check timeout
    }

    private void CreateNode(NodePhenotypeEntity phenotypeEntity)
    {
        // TODO
    }

    private void CreateMuscle(MusclePhenotypeEntity phenotypeEntity)
    {
        // TODO
    }
}
