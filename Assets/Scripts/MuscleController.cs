using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class MuscleController : MonoBehaviour
{
    private GameObject m_node1;
    private GameObject m_node2;
    private Rigidbody m_node1Rb;
    private Rigidbody m_node2Rb;
    private float m_minLength;
    private float m_maxLength;
    private float m_period;
    private IEnumerator m_internalClock;

    public float Strength { get; set; }

    // Start is called before the first frame update
    void Start()
    {
        m_period = Random.Range(0.0f, 2.0f);
    }

    public void SetNodes(GameObject node1, GameObject node2)
    {
        m_node1 = node1;
        m_node2 = node2;
        m_node1Rb = m_node1.GetComponent<Rigidbody>();
        m_node2Rb = m_node1.GetComponent<Rigidbody>();
        m_internalClock = InternalClock();
        StartCoroutine(m_internalClock);
    }

    public void SetLength(float minLength, float maxLength)
    {
        m_minLength = minLength;
        m_maxLength = maxLength;
    }

    public void FixedUpdate()
    {
        if (!m_node1 || !m_node2)
        {
            return;
        }

        Debug.DrawLine(m_node1.transform.position, m_node2.transform.position, Color.magenta);
        // var distance = Vector3.Distance(m_node1.transform.position, m_node2.transform.position);
        // if (distance <= m_minLength || distance >= m_maxLength)
        // {
        //     m_node1Rb.velocity = Vector3.zero;
        //     m_node2Rb.velocity = Vector3.zero;
        // }
    }

    public IEnumerator InternalClock()
    {
        while (true)
        {
            yield return new WaitForSeconds(m_period);
            var direction = Vector3.Normalize(m_node1.transform.position - m_node2.transform.position);
            m_node1Rb.AddForce(-direction * Strength);
            m_node2Rb.AddForce(direction * Strength);
        }
    }
}
