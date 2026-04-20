using System;
using UnityEngine;

public class Door : MonoBehaviour {
    public float openHeight = 3f;
    public float openSpeed = 3f;

    private float closedY;

    private bool isOpen;

    private void Awake() {
        closedY = transform.position.y;
    }
    
    public void SetOpen(bool open) {
        isOpen = open;
    }

    private void FixedUpdate() {
        float targetHeight = closedY + (isOpen ? openHeight : 0f);
        transform.position = new Vector3(
            transform.position.x,
            Mathf.Lerp(transform.position.y, targetHeight, openSpeed * Time.fixedDeltaTime),
            transform.position.z
        );
    }
}