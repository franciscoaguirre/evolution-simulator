using GeneticSharp.Domain.Chromosomes;
using GeneticSharp.Runner.UnityApp.Commons;
using System.Collections.Generic;
using UnityEngine;

public class CreatureChromosome : BitStringChromosome<IPhenotypeEntity>
{
    private CreatureSampleConfig m_config;

    public CreatureChromosome(CreatureSampleConfig config)
    {
        m_config = config;
        NodeCount = config.nodeCount;
        MuscleCount = config.muscleCount;

        var phenotypeEntities = new IPhenotypeEntity[m_config.nodeCount + m_config.muscleCount];

        for (int i = 0; i < m_config.nodeCount; i++)
        {
            phenotypeEntities[i] = new NodePhenotypeEntity(m_config);
        }

        for (int i = m_config.nodeCount; i < m_config.nodeCount + m_config.muscleCount; i++)
        {
            phenotypeEntities[i] = new MusclePhenotypeEntity(m_config);
        }

        phenotypeEntities = Correct(phenotypeEntities);

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

    public bool Evaluated { get; set; }

    public int NodeCount { get; private set; }
    public int MuscleCount { get; private set; }

    private IPhenotypeEntity[] Correct(IPhenotypeEntity[] phenotypes)
    {
        var connectedNodes = new List<int>();
        var danglingNodes = new List<int>();

        for (var i = m_config.nodeCount; i < m_config.nodeCount + m_config.muscleCount; i++)
        {
            var phenotype = phenotypes[i] as MusclePhenotypeEntity;

            if (phenotype.FirstNode == phenotype.SecondNode)
            {
                var random = new System.Random();
                var randomNode = random.Next(0, m_config.nodeCount);
                phenotype.SecondNode = randomNode;
            }

            connectedNodes.Add(phenotype.FirstNode);
            connectedNodes.Add(phenotype.SecondNode);
        }

        if (connectedNodes.Count == 0)
            return phenotypes;

        for (var i = 0; i < m_config.nodeCount; i++)
        {
            if (connectedNodes.IndexOf(i) != -1)
                continue;

            danglingNodes.Add(i);
        }

        var newMuscles = new List<IPhenotypeEntity>();
        foreach (var danglingNode in danglingNodes)
        {
            m_config.muscleCount++;
            var muscle = new MusclePhenotypeEntity(m_config);

            muscle.FirstNode = danglingNode;

            var random = new System.Random();
            var randomConnectedNode = random.Next(0, connectedNodes.Count - 1);
            muscle.SecondNode = connectedNodes[randomConnectedNode];

            newMuscles.Add(muscle);

            connectedNodes.Add(danglingNode);
        }

        var newMusclesArray = newMuscles.ToArray();
        var newArray = new IPhenotypeEntity[phenotypes.Length + newMusclesArray.Length];
        System.Array.Copy(phenotypes, newArray, phenotypes.Length);
        System.Array.Copy(newMusclesArray, 0, newArray, phenotypes.Length, newMusclesArray.Length);

        return newArray;
    }
}
