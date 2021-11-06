using GeneticSharp.Runner.UnityApp.Commons;
using UnityEngine;

public class NodePhenotypeEntity : PhenotypeEntityBase
{
    public const int weightBits = 8;
    public const int positionBits = 8;

    public NodePhenotypeEntity(CreatureSampleConfig config)
    {
        Phenotypes = new IPhenotype[]
        {
            new Phenotype("x", positionBits),
            new Phenotype("y", positionBits),
            new Phenotype("z", positionBits),
            new Phenotype("Weight", weightBits),
        };
    }

    public Vector3 Position
    {
        get
        {
            return new Vector3(
                (float) Phenotypes[0].Value,
                (float) Phenotypes[1].Value,
                (float) Phenotypes[2].Value
            );
        }
    }

    public float Weight
    {
        get
        {
            return (float) Phenotypes[3].Value;
        }
    }
}
