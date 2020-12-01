using System.Collections;
using System.Collections.Generic;
using System.Threading;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;

public class GoButton : MonoBehaviour {

    public Button frontButton;
    public InputField frontInput;

    // Use this for initialization
    void Start () {
        frontButton.onClick.AddListener(TaskOnClick);
    }
	
	// Update is called once per frame
	void TaskOnClick () {
        Thread.Sleep(1000);
        InputInfo.InputString = frontInput.text;
        SceneManager.LoadScene("EnterBuilding");
    }
}
