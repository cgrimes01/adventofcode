using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class SpawnManager : MonoBehaviour {

    public int maxPlatforms = 20;
    public GameObject platform;
    public float horizontalMin = 6.5f;
    public float horizontalMax = 14f;

    public float verticalMin = -6;
    public float verticalMax = 6f;
    public float verticalMinExcluded = -1.1f;
    public float verticalMaxExcluded = 1.1f;

    private Vector2 originPosition;

	// Use this for initialization
	void Start () {
        originPosition = transform.position;
        Spawn();
	}

    void Spawn()
    {
        for(int i = 0; i < maxPlatforms; i++)
        {
            float xChange = Random.Range(horizontalMin, horizontalMax);
            float yChange = RandomWithRestrictions(verticalMin, verticalMax, verticalMinExcluded, verticalMaxExcluded);

            Vector2 randomPosition = originPosition + new Vector2(xChange, yChange);
            Instantiate(platform, randomPosition, Quaternion.identity);
            originPosition = randomPosition;
        }
    }

    float RandomWithRestrictions(float minRange, float maxRange, float minExcluded, float maxExcluded)
    {
        float randomVal = Random.Range(minRange, maxRange);

        randomVal = randomVal >= minExcluded && randomVal <= maxExcluded ? randomVal : maxExcluded;

        return randomVal;
    }
}
