using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class CreatureController : MonoBehaviour
{
    [Header("Prefabs")]
    [SerializeField] private GameObject m_nodePrefab;
    [SerializeField] private GameObject m_musclePrefab;

    public CreatureChromosome Chromosome { get; private set; }
    public List<GameObject> Nodes { get; private set; }
    public List<GameObject> Muscles { get; private set; }

    public void Awake()
    {
        Nodes = new List<GameObject>();
        Muscles = new List<GameObject>();
    }

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

    private void CreateNode(NodePhenotypeEntity nodePhenotype)
    {
        var node = (GameObject) Instantiate(m_nodePrefab, transform);
        node.transform.position = nodePhenotype.Position;
        node.GetComponent<Rigidbody>().mass = nodePhenotype.Weight;
        Debug.Log(nodePhenotype.Position);
        Nodes.Add(node);
    }

    private void CreateMuscle(MusclePhenotypeEntity muscle)
    {
        // TODO
    }
}
