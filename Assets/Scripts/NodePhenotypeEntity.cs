using GeneticSharp.Runner.UnityApp.Commons;
using UnityEngine;

public class NodePhenotypeEntity : PhenotypeEntityBase
{
    public const int weightBits = 8;
    public const int positionBits = 8;

    public const float minValue = 1f;
    public const float maxValue = 10f;

    public NodePhenotypeEntity(CreatureSampleConfig config)
    {
        Phenotypes = new IPhenotype[]
        {
            new Phenotype("x", positionBits)
            {
                MinValue = 1,
                MaxValue = 10,
            },
            new Phenotype("y", positionBits)
            {
                MinValue = 1,
                MaxValue = 10,
            },
            new Phenotype("z", positionBits)
            {
                MinValue = 1,
                MaxValue = 10,
            },
            new Phenotype("Weight", weightBits)
            {
                MinValue = 1,
                MaxValue = 10,
            },
        };
    }

    public Vector3 Position
    {
        get
        {
            return new Vector3(
                Helpers.MinMax((float) Phenotypes[0].Value, minValue, maxValue),
                Helpers.MinMax((float) Phenotypes[1].Value, minValue, maxValue),
                Helpers.MinMax((float) Phenotypes[2].Value, minValue, maxValue)
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
