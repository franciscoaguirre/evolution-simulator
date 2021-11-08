using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Helpers
{
    public static float MinMax(float x, float minValue, float maxValue)
    {
        return (x - minValue) / (maxValue - minValue);
    }
}
