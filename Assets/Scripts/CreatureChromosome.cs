using GeneticSharp.Domain.Chromosomes;
using GeneticSharp.Runner.UnityApp.Commons;

public class CreatureChromosome : BitStringChromosome<IPhenotypeEntity>
{
    private CreatureSampleConfig m_config;
    
    public int m_nodeCount;
    public int m_muscleCount;

    public CreatureChromosome(CreatureSampleConfig config)
    {
        m_config = config;
        m_nodeCount = config.nodeCount;
        m_muscleCount = config.muscleCount;

        var phenotypeEntities = new IPhenotypeEntity[m_config.nodeCount + m_config.muscleCount];

        for (int i = 0; i < m_config.nodeCount; i++)
        {
            phenotypeEntities[i] = new NodePhenotypeEntity(m_config);
        }

        for (int i = m_config.nodeCount; i < m_config.nodeCount + m_config.muscleCount; i++)
        {
            phenotypeEntities[i] = new MusclePhenotypeEntity(m_config);
        }

        SetPhenotypes(phenotypeEntities);
        CreateGenes();
    }

    public override IChromosome CreateNew()
    {
        return new CreatureChromosome(m_config);
    }

    public float MaxDistance
    {
        get;
        set;
    }
}
