using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class FloorsScript : MonoBehaviour {

    public GameObject santa;
    public Text floorNumberText;
    public GameObject christmasTree1;
    public GameObject christmasTree2;

    private Animator santaAnim;
    private Rigidbody2D santaRb2d;
    private SimplePlatformController santaScript;
    private int floorNumber;

    // Use this for initialization
    void Start () {
        santaScript = santa.GetComponent<SimplePlatformController>();
        santaRb2d = santa.GetComponent<Rigidbody2D>();
        santaAnim = santa.GetComponent<Animator>();
        floorNumber = 0;
        floorNumberText.text = floorNumber.ToString();

        StartCoroutine("GetToFloor");
    }

    IEnumerator GetToFloor()
    {
        foreach (char c in InputInfo.InputString)
        {
            if (c == '(')
            {
                if (!santaScript.facingRight) { santaScript.Flip(); }
                yield return new WaitForSeconds(0.5f);
                santaAnim.SetTrigger("Jump");
                santaRb2d.AddForce(new Vector2(300, 700));
                yield return new WaitForSeconds(1.5f);
                santaScript.Disappear();
                santa.transform.position = new Vector2(-4f, -2.97f);
                yield return new WaitForSeconds(0.5f);
                santaScript.Appear();
                yield return new WaitForSeconds(0.5f);
                santaAnim.SetTrigger("Jump");
                santaRb2d.AddForce(new Vector2(320, 700));
                yield return new WaitForSeconds(1);
                santa.transform.position = new Vector2(0, 0);
                floorNumber++;
            }
            if (c == ')')
            {
                if (santaScript.facingRight) { santaScript.Flip(); }
                yield return new WaitForSeconds(0.5f);
                santaAnim.SetTrigger("Jump");
                santaRb2d.AddForce(new Vector2(-470, 0));
                yield return new WaitForSeconds(1.5f);
                santaScript.Disappear();
                santa.transform.position = new Vector2(4f, 2.8f);
                yield return new WaitForSeconds(0.5f);
                santaScript.Appear();
                yield return new WaitForSeconds(0.5f);
                santaAnim.SetTrigger("Jump");
                santaRb2d.AddForce(new Vector2(-470, 0));
                yield return new WaitForSeconds(1);
                santa.transform.position = new Vector2(0, 0);
                floorNumber--;
            }
            floorNumberText.text = floorNumber.ToString();
        }
        yield return new WaitForSeconds(1);
        christmasTree1.SetActive(true);
        christmasTree2.SetActive(true);
    }

    // Update is called once per frame
    void Update () {
       
    }
}
