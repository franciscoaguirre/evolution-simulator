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

        for (int i = chromosome.NodeCount; i < chromosome.NodeCount + chromosome.MuscleCount; i++)
        {
            var phenotype = phenotypes[i];
            CreateMuscle(phenotype as MusclePhenotypeEntity);
        }

        // TODO: Check timeout
    }

    private void CreateNode(NodePhenotypeEntity nodePhenotype)
    {
        var node = (GameObject) Instantiate(m_nodePrefab, transform);
        node.transform.localPosition = nodePhenotype.Position;
        node.GetComponent<Rigidbody>().mass = nodePhenotype.Weight;
        Nodes.Add(node);
    }

    private void CreateMuscle(MusclePhenotypeEntity musclePhenotype)
    {
        var muscle = (GameObject) Instantiate(m_musclePrefab, transform);
        var muscleController = muscle.GetComponent<MuscleController>();
        var node1 = Nodes[musclePhenotype.FirstNode];
        var node2 = Nodes[musclePhenotype.SecondNode];
        muscleController.SetNodes(node1, node2);
        muscleController.Strength = musclePhenotype.Strength;
        muscleController.SetLength(musclePhenotype.MinLength, musclePhenotype.MaxLength);
    }
}
