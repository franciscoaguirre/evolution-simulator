using GeneticSharp.Runner.UnityApp.Commons;
using UnityEngine;

public class MusclePhenotypeEntity : PhenotypeEntityBase
{
    public const int lengthBits = 8;
    public const int strengthBits = 8;
    public const int indexBits = 4;

    public MusclePhenotypeEntity(CreatureSampleConfig config)
    {
        Phenotypes = new IPhenotype[]
        {
            new Phenotype("maxLength", lengthBits),
            new Phenotype("minLength", lengthBits),
            new Phenotype("strength", strengthBits),
            new Phenotype("firstNode", indexBits),
            new Phenotype("secondNode", indexBits),
        };
    }

    public float MaxLength
    {
        get
        {
            return (float) Phenotypes[0].Value;
        }
    }

    public float MinLength
    {
        get
        {
            return (float) Phenotypes[1].Value;
        }
    }

    public float Strength
    {
        get
        {
            return (float)Phenotypes[2].Value;
        }
    }

    public int FirstNode
    {
        get
        {
            return (int) Phenotypes[3].Value;
        }
    }
    
    public int SecondNode
    {
        get
        {
            return (int) Phenotypes[4].Value;
        }
    }
}
