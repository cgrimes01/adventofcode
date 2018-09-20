using System.Collections;
using System.Collections.Generic;
using System.Threading;
using UnityEngine;
using UnityEngine.SceneManagement;

public class EnterBuildingScript : MonoBehaviour {

    public GameObject santa;
    public Camera santaCamera;

    private SimplePlatformController santaScript;

    private Animator santaAnim;
    private Rigidbody2D santaRb2d;

    // Use this for initialization
    void Start () {
        santaScript = santa.GetComponent<SimplePlatformController>();
        
        santaRb2d = santa.GetComponent<Rigidbody2D>();
        StartCoroutine("EnterBuilding");
    }

    IEnumerator EnterBuilding()
    {
        santaScript.Walk();
        yield return new WaitForSeconds(7);

        santaScript.Stop();
        yield return new WaitForSeconds(1);

        santaCamera.orthographicSize = 10;
        santaCamera.transform.position = new Vector3(0, 8, -7.5f);
        yield return new WaitForSeconds(1.5f);

        santaScript.Walk();
        yield return new WaitForSeconds(3.4f);

        santaScript.Stop();
        yield return new WaitForSeconds(1.5f);

        Renderer[] renderers = santa.GetComponentsInChildren<Renderer>();
        foreach (Renderer r in renderers)
        {
            r.enabled = false;
        }
        yield return new WaitForSeconds(0.5f);
        SceneManager.LoadScene("Stairs");
    }
	
	// Update is called once per frame
	void Update () {
		
	}
}
