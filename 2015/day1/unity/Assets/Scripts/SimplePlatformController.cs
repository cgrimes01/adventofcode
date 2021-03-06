﻿using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class SimplePlatformController : MonoBehaviour {

    [HideInInspector] public bool facingRight = false;
    [HideInInspector] public bool jump = false;

    public float moveForce = 365f;
    public float maxSpeed = 5f;
    public float jumpForce = 1000f;
    public Transform groundCheck;

    private bool grounded = false;
    private Animator anim;
    private Rigidbody2D rb2d;

    // Use this for initialization
    void Awake () {
        anim = GetComponent<Animator>();
        rb2d = GetComponent<Rigidbody2D>();
        jump = false;
	}
	
	// Update is called once per frame
	void Update () {
        //grounded = Physics2D.Linecast(transform.position, groundCheck.position, 1 << LayerMask.NameToLayer("Ground"));

        //if(Input.GetButtonDown("Jump") && grounded)
        //{
        //    jump = true;
        //}
	}

    //private void FixedUpdate()
    //{
    //    float h = Input.GetAxis("Horizontal");

    //    if(h != 0)
    //    {
    //        MoveSanta(h);
    //    }
    //}

    public void Walk()
    {
        anim.SetFloat("Speed", 1);
        rb2d.velocity = new Vector2(5, 0);
    }

    public void Stop()
    {
        anim.SetFloat("Speed", 0);
        rb2d.velocity = new Vector2(0, 0);
    }

    public void MoveSanta(float h)
    {
        anim.SetFloat("Speed", Mathf.Abs(h));

        if (h * rb2d.velocity.x < maxSpeed)
        {
            rb2d.AddForce(Vector2.right * h * moveForce);
        }

        if (Mathf.Abs(rb2d.velocity.x) > maxSpeed)
        {
            rb2d.velocity = new Vector2(Mathf.Sign(rb2d.velocity.x) * maxSpeed, rb2d.velocity.y);
        }

        if (h > 0 && !facingRight)
        {
            Flip();
        }
        else if (h < 0 && facingRight)
        {
            Flip();
        }

        if (jump)
        {
            anim.SetTrigger("Jump");
            rb2d.AddForce(new Vector2(0f, jumpForce));
            jump = false;
        }
    }

    public void Appear()
    {
        Renderer[] renderers = this.gameObject.GetComponentsInChildren<Renderer>();
        foreach (Renderer r in renderers)
        {
            r.enabled = true;
        }
    }

    public void Disappear()
    {
        Renderer[] renderers = this.gameObject.GetComponentsInChildren<Renderer>();
        foreach (Renderer r in renderers)
        {
            r.enabled = false;
        }
    }

    public void Flip()
    {
        facingRight = !facingRight;
        Vector3 theScale = transform.localScale;
        theScale.x *= -1;
        transform.localScale = theScale;
    }
}
