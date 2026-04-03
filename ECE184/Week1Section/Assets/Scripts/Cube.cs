using UnityEngine;

public class Cube : MonoBehaviour {
    private new Renderer renderer;
    
    void Start() {
        renderer = GetComponent<Renderer>();
        renderer.material.color = Random.ColorHSV(0f, 1f, 0f, 1f, 0.5f, 1f);
    }
}
