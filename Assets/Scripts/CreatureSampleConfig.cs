using UnityEngine;

[CreateAssetMenu(menuName = "CreatureSampleConfig", order = 1)]
public class CreatureSampleConfig : ScriptableObject
{
    [Header("Creature")]
    public int nodeCount = 8;
    public int muscleCount = 5;
}
